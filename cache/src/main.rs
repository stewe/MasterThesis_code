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
        let mut publisher: Socket = ctx.socket(zmq::PUB).unwrap();
        publisher.bind("tcp://*:5560").unwrap();

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
                                    let responses = cache_enclave::ecall_handle_request(msg);
                                    match responses.len() {
                                        0 => {},
                                        1 => { socket.send(&responses.first().unwrap(), 0).unwrap(); },
                                        _ => {
                                            socket.send(&responses.first().unwrap(), 0).unwrap();
                                            for resp in responses.iter().skip(1) {
                                                publisher.send(&resp, 0).unwrap();
                                            }
                                        },
                                    }
                                },
                                _  => { break }
                            }
                        }
                        loop {
                            match subscriber.recv_bytes(DONTWAIT) { // TODO adapt to message format (e.g. google protocol buffers); handle in an approriate way!
                                Ok(msg) =>  {
                                    info!("msg: {:?}", &msg);
                                    let _ = cache_enclave::ecall_handle_sub_msg(msg);
                                    // for resp in responses { subscriber.send(&resp, 0).unwrap(); }
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
        socket.connect("tcp://localhost:5555").unwrap();

        let filters: [(&str,&[u8]);5] = [
            ("clamp15", &[10, 7, 99, 108, 97, 109, 112, 49, 53]),
            ("voltage", &[10, 15, 105, 110, 118, 97, 108, 105, 100]),
            ("unlcutch", &[10, 8, 117, 110, 99, 108, 117, 116, 99, 104]),
            ("speed-error", &[10, 11, 115, 112, 101, 101, 100, 45, 101, 114, 114, 111, 114]),
            ("speed-unsafe", &[10, 12, 115, 112, 101, 101, 100, 45, 117, 110, 115, 97, 102, 101]),
        ];

        for f in filters.iter() {
            socket.set_subscribe(format!("{}{}", "{\"msg_type\":\"", f.0).as_bytes()).unwrap();
            socket.set_subscribe(f.1).unwrap();
        }

        // socket.set_subscribe(&[]).unwrap(); // every message

    socket
}
