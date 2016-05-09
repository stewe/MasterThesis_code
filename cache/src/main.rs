#[macro_use]
extern crate log;
// extern crate env_logger;
extern crate simple_logger;
extern crate time;
extern crate zmq;
extern crate enclave_cache;

// extern crate zmq_rs;
// extern crate zmq_ffi;

// use std::collections::{HashMap};
// use std::sync::Arc;
// use std::sync::mpsc::{Sender};
// use std::thread::JoinHandle;

use enclave_cache::{CacheEnclave};
use zmq::{Socket, Context};

// use zmq_rs;


// ------------------------------------------------------------------------------------
fn main() {
    // env_logger::init().unwrap();
    simple_logger::init().unwrap();
    info!("Cache started.");

    let mut cache_enclave = CacheEnclave::new(10, 60000); // TODO configure
    let mut ctx = Context::new();


    // main thread: receive requests, forward into enclave
        let mut socket: Socket = ctx.socket(zmq::REP).unwrap();
        socket.bind("tcp://*:5550").unwrap();


        let mut msg = zmq::Message::new().unwrap();
        loop {
            socket.recv(&mut msg, 0).unwrap();
            debug!("Received request: {:?}", msg.as_str().clone());

            let resp = cache_enclave.handle_request(&mut msg);
            socket.send(resp.as_bytes(), 0).unwrap();
        }

}
