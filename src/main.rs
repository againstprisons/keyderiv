extern crate hyper;
extern crate iron;
extern crate params;
extern crate mount;
extern crate env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate crypto;

use std::env;
use std::os::unix::io::FromRawFd;
use std::ops::Deref;
use hyper::net::{HttpListener, NetworkListener};
use iron::prelude::*;
use iron::Protocol;
use iron::status;
use params::{Params, Value, FromValue};
use crypto::scrypt::{ScryptParams, scrypt};

lazy_static! {
    // keys
    static ref INDEX_KEY: String = env::var("INDEX_KEY").expect("INDEX_KEY not provided");
    static ref ENCRYPT_KEY: String = env::var("ENCRYPT_KEY").expect("ENCRYPT_KEY not provided");

    // scrypt params
    static ref SCRYPT_PARAMS: ScryptParams = ScryptParams::new(4, 8, 1);
}

pub fn generate_key(data: &String, salt: &String) -> String {
    let mut data_vec = Vec::<u8>::new();
    for byte in data.bytes() {
        data_vec.push(byte);
    }

    let mut salt_vec = Vec::<u8>::new();
    for byte in salt.bytes() {
        salt_vec.push(byte);
    }

    let mut output_vec = vec![0; 32];
    scrypt(&data_vec, &salt_vec, &SCRYPT_PARAMS, output_vec.as_mut_slice());

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
            let output = generate_key(&data, INDEX_KEY.deref());

            Ok(Response::with((status::Ok, output)))
        },

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
            let output = generate_key(&data, ENCRYPT_KEY.deref());

            Ok(Response::with((status::Ok, output)))
        },

        _ => {
            Ok(Response::with(status::BadRequest))
        }
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

