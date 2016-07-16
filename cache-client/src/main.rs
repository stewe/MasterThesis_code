#[macro_use]
extern crate log;
// extern crate env_logger;
extern crate simple_logger;
extern crate zmq;
extern crate enclave_cache_client as client;

use client::client_enclave;
use zmq::{Socket, Context};

fn main() {
    simple_logger::init().unwrap();
    info!("Hello, world, it's the first client of enclave_cache!");

    let mut ctx = Context::new();
    let mut socket: Socket = ctx.socket(zmq::REQ).unwrap();
    socket.connect("tcp://localhost:5550").unwrap();

    let mut req = vec!();

    loop {
        if !req.is_empty() {
            // info!("client sends: {:?}", req);
            socket.send(&req, 0).unwrap();
            let resp = socket.recv_bytes(0).unwrap();
            req = client_enclave::ecall_handle_request(resp);

        } else {
            req = client_enclave::foo();
        }
}

}
