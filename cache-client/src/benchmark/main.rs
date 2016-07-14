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

    let mut msg_format = MsgFormat::Protobuf;
    let mut request = "Start";

    if env::args().len() == 1 {
        panic!("Use the following parameters:
                action=start | stop
                format=json | protobuf (optional, default: Protobuf)
                ");
    }

    for arg in env::args().skip(1) {
        if arg.len() < 2 { panic!("Invalid argument: {}", arg) }
        let mut splitted = arg.split('=');
        let (k, val) = (splitted.next().expect(format!("Invalid argument: {}", arg).as_str()), splitted.next().expect(format!("Invalid argument: {}", arg).as_str()));
        match k {
            "action" => {
                match val {
                    "start" => {
                        info!("Cache benchmark started.");
                        request = "Start";
                    },
                    "stop" => {
                        info!("Cache benchmark stopped.");
                        request = "Stop";
                    },
                    _ => panic!("Invalid parameter, use start or stop.")
                }
            },
            "format" => {
                msg_format = match val {
                    "json" => MsgFormat::Json,
                    "protobuf" => MsgFormat::Protobuf,
                    _ => panic!("Unknown message format. Use one of the following:\njson protobuf"),
                }
            },
            _ => panic!("Invalid argument: {}", arg),
        }
    }

    let mut ctx = Context::new();
    let mut socket: Socket = ctx.socket(zmq::REQ).unwrap();
    socket.connect("tcp://localhost:5550").unwrap();


    socket.send(&encode_cache_msg(vec![], request, MsgPolicy::Plain, None, 0, msg_format).unwrap(), 0).unwrap();

    let result_msg = decode_cache_msg(socket.recv_bytes(0).unwrap(), msg_format).unwrap();
    if request == "Stop" {
        info!("Benchmark result: {} requests per second.", decode_u32_msg(result_msg.msg, msg_format).unwrap());
    }

    socket.close().unwrap();
    ctx.destroy().unwrap();

}
