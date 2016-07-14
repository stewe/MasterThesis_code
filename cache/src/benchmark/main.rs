extern crate msg_lib;
extern crate sgx_isa;
#[macro_use]
extern crate log;
// extern crate env_logger;
extern crate zmq;
extern crate simple_logger;

use msg_lib::{decode_cache_msg, decode_u32_msg,
                encode_cache_msg, MsgPolicy, MsgFormat};
use zmq::{Socket, Context};
use std::env;

fn main() {
    simple_logger::init().unwrap();

    if env::args().len() == 1 {
        panic!("Use one the following parameters: start | stop");
    }

    // let msg_format = MsgFormat::Json;
    let msg_format = MsgFormat::Protobuf;

    let mut ctx = Context::new();
    let mut socket: Socket = ctx.socket(zmq::REQ).unwrap();
    socket.connect("tcp://localhost:5550").unwrap();

    let request;

    match env::args().skip(1).next().unwrap().as_str() {
        "start" => {
            info!("Cache benchmark started.");
            request = "Start";
        },
        "stop" => {
            info!("Cache benchmark stopped.");
            request = "Stop";
        },
        _ => panic!("Invalid parameter, use start or stop.")
    };

    socket.send(&encode_cache_msg(vec![], request, MsgPolicy::Plain, None, 0, msg_format).unwrap(), 0).unwrap();

    let result_msg = decode_cache_msg(socket.recv_bytes(0).unwrap(), msg_format).unwrap();
    if request == "Stop" {
        info!("Benchmark result: {} requests per second.", decode_u32_msg(result_msg.msg, msg_format).unwrap());
    }

    // loop {
    //     if !req.is_empty() {
    //         // info!("client sends: {:?}", req);
    //         socket.send(&req, 0).unwrap();
    //         let resp = socket.recv_bytes(0).unwrap();
    //         req = client_enclave::ecall_handle_request(resp);
    //
    //     } else {
    //         req = client_enclave::foo();
    //     }
    // }

    socket.close().unwrap();
    ctx.destroy().unwrap();

}
