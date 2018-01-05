extern crate hyper;
extern crate iron;
extern crate mount;
extern crate env_logger;
#[macro_use]
extern crate log;

use hyper::net::{HttpListener, NetworkListener};
use iron::prelude::*;
use iron::Protocol;
use iron::status;
use mount::Mount;
use std::env;
use std::os::unix::io::FromRawFd;

//mod handler;

fn testhandler(req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    resp.set_mut("hi!")
        .set_mut(status::Ok);

    Ok(resp)
}

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", format!("{}=info", env!("CARGO_PKG_NAME")));
    }

    let _logger = env_logger::init();
    info!("Starting up...");

    let mut mount = Mount::new();
    //info!("Mounting /index");
    //mount.mount("/index", handler::index);
    //info!("Mounting /encrypt")
    //mount.mount("/encrypt", handler::encrypt);
    info!("Mounting test /");
    mount.mount("/", testhandler);

    debug!("Making iron chain");
    let chain = Chain::new(mount);

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

    info!("earmms_keyderiv in flight at {}", netstr);
    Iron::new(chain).listen(listener, Protocol::http()).unwrap();
}

