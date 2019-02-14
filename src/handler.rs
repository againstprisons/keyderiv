use iron::prelude::*;
use iron::status;
use iron::Protocol;
use params::{FromValue, Params, Value};

use crate::find_target;
use crate::keyderiv_crypto::{encrypt_secret, generate_key};
use crate::util::hex_to_bytes;

pub fn handler_fn(req: &mut Request) -> IronResult<Response> {
    let map = req.get_ref::<Params>().unwrap();
    let target_name: String = match map.get("target") {
        Some(&Value::String(ref val)) => val.clone(),
        _ => return Ok(Response::with((status::BadRequest, "no target"))),
    };

    let target = match find_target(&target_name) {
        Some(t) => t,
        _ => return Ok(Response::with((status::BadRequest, "no target"))),
    };

    let nonce: Vec<u8> = match map.get("nonce") {
        Some(&Value::String(ref val)) => hex_to_bytes(&val.clone()),
        _ => return Ok(Response::with((status::BadRequest, "no nonce"))),
    };

    let mode = match map.get("mode") {
        Some(&Value::String(ref val)) => {
            let mode = val.clone();
            if mode != "index" && mode != "encrypt" {
                return Ok(Response::with((status::BadRequest, "no mode")));
            }

            mode
        }

        _ => return Ok(Response::with((status::BadRequest, "no mode"))),
    };

    let key = if mode == "encrypt" {
        target.encrypt_key
    } else {
        target.index_key
    };

    // generate the string to feed the key generation function
    let formatted_input = {
        let table = match map.get("table") {
            Some(&Value::String(ref val)) => val.clone(),
            _ => return Ok(Response::with((status::BadRequest, "no table"))),
        };

        let column = match map.get("column") {
            Some(&Value::String(ref val)) => val.clone(),
            _ => return Ok(Response::with((status::BadRequest, "no column"))),
        };

        if mode == "encrypt" {
            let row = match map.get("row") {
                Some(&Value::String(ref val)) => val.clone(),
                _ => return Ok(Response::with((status::BadRequest, "no row"))),
            };

            format!("{}:{}:{}", table, column, row)
        } else {
            format!("{}:{}", table, column)
        }
    };

    // generate the key
    let generated_key = generate_key(&formatted_input, &key);

    // encrypt with shared secret
    let encrypted_output = encrypt_secret(&generated_key, &target.shared_secret, &nonce);
    match encrypted_output {
        Ok(val) => Ok(Response::with((status::Ok, val))),
        Err(e) => Ok(Response::with((status::InternalServerError, e))),
    }
}
