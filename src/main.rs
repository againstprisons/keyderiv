extern crate env_logger;
extern crate hyper;
extern crate iron;
extern crate mount;
extern crate params;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate crypto;

use crypto::scrypt::{scrypt, ScryptParams};
use hyper::net::{HttpListener, NetworkListener};
use iron::prelude::*;
use iron::status;
use iron::Protocol;
use params::{FromValue, Params, Value};
use std::env;
use std::ops::Deref;
use std::os::unix::io::FromRawFd;

lazy_static! {
    // keys
    static ref INDEX_KEY: String = env::var("INDEX_KEY").expect("INDEX_KEY not provided");
    static ref ENCRYPT_KEY: String = env::var("ENCRYPT_KEY").expect("ENCRYPT_KEY not provided");

    static ref INDEX_KEY_BYTES: Vec<u8> = hex_to_bytes(INDEX_KEY.deref());
    static ref ENCRYPT_KEY_BYTES: Vec<u8> = hex_to_bytes(ENCRYPT_KEY.deref());

    // scrypt params
    static ref SCRYPT_PARAMS: ScryptParams = ScryptParams::new(4, 8, 1);
}

pub fn hex_to_bytes(data: &str) -> Vec<u8> {
    let input_chars: Vec<_> = data.chars().collect();

    input_chars
        .chunks(2)
        .map(|chunk| {
            let first = chunk[0].to_digit(16).unwrap();
            let second = chunk[1].to_digit(16).unwrap();
            ((first << 4) | second) as u8
        })
        .collect()
}

pub fn generate_key(data: &str, salt_vec: &Vec<u8>) -> String {
    let mut data_vec = Vec::<u8>::new();
    for byte in data.bytes() {
        data_vec.push(byte);
    }

    let mut output_vec: Vec<u8> = vec![0; 32];
    scrypt(
        &data_vec,
        &salt_vec,
        &SCRYPT_PARAMS,
        output_vec.as_mut_slice(),
    );

    let mut output = String::new();
    for byte in output_vec.iter() {
        output.push_str(format!("{:02x}", byte).as_str());
    }

    output
}

pub fn handler(req: &mut Request) -> IronResult<Response> {
    let map = req.get_ref::<Params>().unwrap();

    match map.get("mode") {
        Some(&Value::String(ref m)) if m == "index" => {
            let table_value = map.get("table");
            if table_value.is_none() {
                return Ok(Response::with((status::BadRequest, "table not specified")));
            }

            let column_value = map.get("column");
            if column_value.is_none() {
                return Ok(Response::with((status::BadRequest, "column not specified")));
            }

            let table: String = FromValue::from_value(&table_value.unwrap()).unwrap();
            let column: String = FromValue::from_value(&column_value.unwrap()).unwrap();

            let data = format!("{}:{}", table, column);
            let output = generate_key(&data, INDEX_KEY_BYTES.deref());

            Ok(Response::with((status::Ok, output)))
        }

        Some(&Value::String(ref m)) if m == "encrypt" => {
            let table_value = map.get("table");
            if table_value.is_none() {
                return Ok(Response::with((status::BadRequest, "table not specified")));
            }

            let column_value = map.get("column");
            if column_value.is_none() {
                return Ok(Response::with((status::BadRequest, "column not specified")));
            }

            let row_value = map.get("row");
            if row_value.is_none() {
                return Ok(Response::with((status::BadRequest, "row not specified")));
            }

            let table: String = FromValue::from_value(&table_value.unwrap()).unwrap();
            let column: String = FromValue::from_value(&column_value.unwrap()).unwrap();
            let row: String = FromValue::from_value(&row_value.unwrap()).unwrap();

            let data = format!("{}:{}:{}", table, column, row);
            let output = generate_key(&data, ENCRYPT_KEY_BYTES.deref());

            Ok(Response::with((status::Ok, output)))
        }

        _ => Ok(Response::with(status::BadRequest)),
    }
}

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", format!("{}=info", env!("CARGO_PKG_NAME")));
    }

    let _logger = env_logger::init();
    info!("Starting up...");

    debug!("Finding socket");
    let mut listener = env::var("LISTEN_FD")
        .ok()
        .and_then(|fd| fd.parse().ok())
        .and_then(|fd| {
            info!("Found LISTEN_FD, binding to that socket");
            Some(unsafe { HttpListener::from_raw_fd(fd) })
        })
        .unwrap_or_else(|| {
            info!("No LISTEN_FD, creating a socket ourselves");
            let host = env::var("HOST").unwrap_or("0.0.0.0".into());
            let port = env::var("PORT").unwrap_or("8080".into());
            let addr = format!("{}:{}", host, port);
            HttpListener::new(addr).unwrap()
        });

    let netstr = listener
        .local_addr()
        .and_then(|a| Ok(format!("{}", a)))
        .unwrap_or("LISTEN_FD".into());

    debug!("Making iron chain");
    let chain = Chain::new(handler);

    info!("earmms_keyderiv in flight at {}", netstr);
    Iron::new(chain).listen(listener, Protocol::http()).unwrap();
}
