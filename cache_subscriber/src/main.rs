extern crate msg_lib;
#[macro_use]
extern crate log;
// extern crate env_logger;
extern crate simple_logger;
// extern crate time;
extern crate zmq;

use std::env;
use std::str::FromStr;

use msg_lib::{ decode_cache_msg, decode_u32_msg,
                //encode_bool_msg, encode_u8_msg, encode_bytes_vec_msg,
                encode_sub_cache_msg,
                MsgFormat, MsgPolicy, slice_to_vec};
use std::time::{Duration, Instant};
use zmq::{Socket, Context};

struct Param<'a>(Option<u32>, Vec<Vec<u8>>, &'a mut Socket, &'a mut Socket);


fn main() {
    // simple_logger::init().unwrap();
    // simple_logger::init_with_level(log::LogLevel::Debug).unwrap();
    simple_logger::init_with_level(log::LogLevel::Info).unwrap();
    debug!("Started the cache subscriber.");

    let mut msg_format = MsgFormat::Protobuf;
    let mut action = "";
    let mut valuenr = 0;

    if env::args().len() == 1 {
        panic!("Use the following parameters:
                action=latency | throughput
                format=json | protobuf (optional, default: Protobuf)
                ");
    }

    let mut ctx = Context::new();

    let mut subscriber = ctx.socket(zmq::SUB).unwrap();
    subscriber.connect("tcp://localhost:5560").unwrap();
    subscriber.set_subscribe(&[]).unwrap();
    // TODO subscribe to specific filters

    let mut requester: Socket = ctx.socket(zmq::REQ).unwrap();
    requester.connect("tcp://localhost:5550").unwrap();

    for arg in env::args().skip(1) {
        if arg.len() < 2 { panic!("Invalid argument: {}", arg) }
        let mut splitted = arg.split('=');
        let (k, val) = (splitted.next().expect(format!("Invalid argument: {}", arg).as_str()), splitted.next().expect(format!("Invalid argument: {}", arg).as_str()));
        match k {
            "action" => {
                match val {
                    "latency" => {
                        debug!("Cache subscriber started measuring latency.");
                        action = "latency";
                    },
                    "stop" => {
                        info!("Cache benchmark stopped.");
                        action = "Stop";
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
            "valuenr" => { valuenr = u32::from_str(val).expect("Invalid value for value number, expecting a u32.") },
            _ => panic!("Invalid argument: {}", arg),
        }
    }

    let number = if valuenr > 0 { Some(valuenr) } else { None };
    let mut param = Param(number, vec![vec![]],  &mut requester, &mut subscriber);

    match action {
        "latency" => {
            param.1 = slice_to_vec(&[
                slice_to_vec(&("clamp15".as_bytes())),
                slice_to_vec(&("invalid-voltage".as_bytes())),
                slice_to_vec(&("speed-error".as_bytes())),
                slice_to_vec(&("speed-unsafe".as_bytes())),
                slice_to_vec(&("unclutch".as_bytes())),
                slice_to_vec(&("sized".as_bytes())),
                ]);

            // only one request
            // info!("time for subscription and receiving all values: {:?}",
            // measure_cached_subscription(&mut param, msg_format));

            let iterations = 1000;
            // = latency???
            let (dur, value_size) = average_request_time(&mut param, iterations, msg_format);
            info!("; {}; {}; {}; {};",
                    valuenr, (value_size - value_size%100), dur.as_secs(), dur.subsec_nanos());

        },
        _ => panic!("Nothing to do."),
    }

    // clean up zmq
    param.2.close().unwrap();
    param.3.close().unwrap();
    ctx.destroy().unwrap();
}

fn average_request_time(param: &mut Param, n: u32, msg_format: MsgFormat) -> (Duration, usize) {
    let (dur, value_size) = measure_cached_subscription(param, msg_format);
    let dur = (0..n-1).fold(dur, |acc, _| acc + measure_cached_subscription(param, msg_format).0) / n;
    (dur, value_size)
}

fn measure_cached_subscription(param: &mut Param, msg_format: MsgFormat) -> (Duration, usize) {
    let start = Instant::now();
    let value_size = cached_subscription(param, msg_format);
    (start.elapsed(), value_size)
}

fn cached_subscription(param: &mut Param, msg_format: MsgFormat) -> usize {
    // let (topics, req, sub) = (param.0, param.1, param.2);
    // TODO think about timer (timeout for this function...)

    for topic in param.1.iter() {
        param.3.set_subscribe(&topic).unwrap();
    }
    let request = encode_sub_cache_msg(param.0, param.1.clone(), "SUB", MsgPolicy::Plain, None, msg_format).unwrap();
    param.2.send(&request, 0).unwrap();

    let resp = param.2.recv_bytes(0).unwrap();
    let cache_msg = decode_cache_msg(resp, msg_format).unwrap();
    let total_values = decode_u32_msg(cache_msg.msg, msg_format).unwrap();
    let mut value_size = 0;

    for _ in 0..total_values {
        let value = param.3.recv_bytes(0).unwrap();
        if value_size != 0 { assert_eq!(value_size, value.len()); }
        value_size = value.len();
        trace!("received value: {:?}", value);
    }

    debug!("Received {} values.", total_values);
    value_size
}
