#![recursion_limit = "1024"]

extern crate common;
extern crate chrono;
extern crate core;
extern crate futures;
extern crate indradb;
extern crate libc;
extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate tokio_core;
extern crate uuid;

use futures::future::Future;
use std::env;
use std::sync::Arc;
use tokio_core::reactor::Core;

fn main() {
    let mut core = Core::new().unwrap();

    let port_str = env::var("PORT").unwrap_or_else(|_| "27615".to_string());
    let port = port_str
        .parse::<u16>()
        .expect("Could not parse environment variable `PORT`");
    let binding = format!("127.0.0.1:{}", port);

    common::server::start(core, &binding).expect("Expected to be able to start the server");
}
