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

use enclave_cache::cache_enclave;
// use enclave_cache::{CacheEnclave};
use zmq::{Socket, Context, DONTWAIT, poll, POLLIN};

// use zmq_rs;


// ------------------------------------------------------------------------------------
fn main() {
    // env_logger::init().unwrap();
    simple_logger::init().unwrap();
    info!("Cache started.");

    // let mut cache_enclave = CacheEnclave::new_default();
    let mut ctx = Context::new();


    // main thread: receive requests, forward into enclave
         // TODO consider DEALER/ROUTER for asynchronous communication like ADD, see http://zguide.zeromq.org/page:all#advanced-request-reply
        let mut socket: Socket = ctx.socket(zmq::REP).unwrap();
        socket.bind("tcp://*:5550").unwrap();

        // TODO the first published message will not arrive -> send some dummy message(s)
        // use REP socket for sub-requests
        // TODO publisher socket

        let mut subscriber = init_subscriber_socket(&mut ctx);

        loop {
            // read message from zmq socket
            match poll(&mut [subscriber.as_poll_item(POLLIN), socket.as_poll_item(POLLIN)], 20) {
                Ok(n) => {
                    if n > 0 {
                        loop {
                            match socket.recv_bytes(DONTWAIT) { // TODO adapt to message format (e.g. google protocol buffers); handle in an approriate way!
                                Ok(msg) =>  {
                                    info!("msg: {:?}", &msg);
                                    let resp = cache_enclave::ecall_handle_request(msg);
                                    if !resp.is_empty() { socket.send(&resp, 0).unwrap(); }
                                },
                                _  => { break }
                            }
                        }
                        loop {
                            match subscriber.recv_bytes(DONTWAIT) { // TODO adapt to message format (e.g. google protocol buffers); handle in an approriate way!
                                Ok(msg) =>  {
                                    info!("msg: {:?}", &msg);
                                    let resp = cache_enclave::ecall_handle_request(msg);
                                    if !resp.is_empty() { subscriber.send(&resp, 0).unwrap(); }
                                },
                                _  => { break }
                            }
                        }
                    }

                },
                _ => {},
            }
        }

}

// pub fn init_sensor_socket(sensor: &Sensor, ctx: &mut Context) -> Socket {
pub fn init_subscriber_socket(ctx: &mut Context) -> Socket {
        let mut socket: Socket = ctx.socket(zmq::SUB).unwrap();
        // for filter in &sensor.filters {
        //     assert!(socket.set_subscribe(filter.as_bytes()).is_ok());
        // }

        // socket.connect("tcp://localhost:5555").unwrap();
        socket.connect("tcp://localhost:5551").unwrap();
        socket.connect("tcp://localhost:5552").unwrap();
        socket.connect("tcp://localhost:5553").unwrap();
        socket.connect("tcp://localhost:5554").unwrap();
        socket.connect("tcp://localhost:5554").unwrap();
        socket.connect("tcp://localhost:5556").unwrap();

        // subscribe to unclutch json
        socket.set_subscribe("{\"msg_type\":\"pub/unclutch\",".as_bytes()).unwrap();
        // subscribe to unclutch protobuf
        // socket.set_subscribe(&[10, 12, 112, 117, 98, 47, 117, 110, 99, 108, 117, 116, 99, 104]).unwrap();
        socket.set_subscribe("\n\u{c}pub/unclutch".as_bytes()).unwrap();

        // subscribe to clamp15 json
        socket.set_subscribe("{\"msg_type\":\"pub/clamp15\",".as_bytes()).unwrap();
        // subscribe to clamp15 protobuf
        socket.set_subscribe("\n\u{b}pub/clamp15".as_bytes()).unwrap();


        // socket.set_subscribe(&[]).unwrap(); // every message
        // socket.set_subscribe("2".as_bytes()).unwrap();
        // socket.set_subscribe("a".as_bytes()).unwrap();

    socket
}
