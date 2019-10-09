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
extern crate dotenv;
extern crate structopt;
#[macro_use]
extern crate serde_derive;
extern crate rust_sodium;
extern crate toml;

use hyper::net::{HttpListener, NetworkListener};
use iron::prelude::*;
use iron::Protocol;
use std::env;
use std::fs::File;
use std::io::Read;
use std::os::unix::io::FromRawFd;
use std::path::PathBuf;
use std::sync::Mutex;
use structopt::StructOpt;

mod config;
use config::Config;
mod target;
use target::Target;
mod handler;
mod keyderiv_crypto;
mod util;
use handler::handler_fn;

lazy_static! {
    pub static ref TARGETS: Mutex<Vec<Target>> = Mutex::new(Vec::new());
}

pub fn find_target(name: &str) -> Option<Target> {
    let targets = TARGETS.lock().unwrap();
    for target in targets.clone() {
        if target.name == name {
            return Some(target.clone());
        }
    }

    None
}

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    /// Read from .env in working directory
    #[structopt(long = "dotenv")]
    dotenv: bool,

    /// Path to the configuration file
    #[structopt(long = "config", parse(from_os_str))]
    config: Option<PathBuf>,
}

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", format!("{}=info", env!("CARGO_PKG_NAME")));
    }

    env_logger::init().ok();

    let opt = Opt::from_args();

    if opt.dotenv {
        info!("Loading .env");
        dotenv::dotenv().ok();
    }

    let config_path = opt.config.unwrap_or_else(|| {
        env::var("KEYDERIV_CONFIG")
            .unwrap_or_else(|_| {
                error!("No keyderiv config found, aborting.");
                error!("Either pass --config=<path> or set env var KEYDERIV_CONFIG=<path>");

                panic!("No keyderiv config found.");
            })
            .into()
    });

    info!("Config file at path {:?}", &config_path);

    debug!("Reading config file");
    let mut config_str = String::new();
    File::open(config_path)
        .expect("Failed to read config file")
        .read_to_string(&mut config_str)
        .expect("Failed to read config file");

    debug!("Parsing config file");
    let config: Config = toml::from_str(&config_str).expect("Failed to read config file");

    info!("Loaded config for {} targets", config.target.len());

    {
        debug!("Pushing targets");

        let mut targets = TARGETS.lock().unwrap();
        for cfgtarget in config.target.clone() {
            let target = Target::new(cfgtarget);
            targets.push(target.clone());
        }

        assert_eq!(config.target.len(), targets.len());
        debug!(
            "Pushing targets successful, TARGETS has length {}",
            targets.len()
        );
    };

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
    let chain = Chain::new(handler_fn);

    info!("earmms_keyderiv in flight at {}", netstr);
    Iron::new(chain).listen(listener, Protocol::http()).unwrap();
}
