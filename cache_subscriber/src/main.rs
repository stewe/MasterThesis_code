extern crate msg_lib;
#[macro_use]
extern crate log;
// extern crate env_logger;
extern crate simple_logger;
// extern crate time;
extern crate zmq;

use std::env;
use std::str::FromStr;
use std::thread;

use msg_lib::{ decode_cache_msg, decode_u32_msg,
                //encode_bool_msg, encode_u8_msg, encode_bytes_vec_msg,
                encode_cache_msg, encode_sub_cache_msg,
                MsgFormat, MsgPolicy, slice_to_vec};
use std::time::{Duration, Instant};
use std::thread::sleep;
use zmq::{Context, DONTWAIT, Socket};

struct Param<'a>(Option<u32>, &'a mut Socket, &'a mut Socket);


fn main() {

    let mut msg_format = MsgFormat::Protobuf;
    let mut action = "";
    let mut valuenr = 0;
    let mut threads = None;
    let mut period = None;

    if env::args().len() == 1 {
        panic!("Use the following parameters:
                action=latency | throughput | request
                threads=NR_OF_REQUESTER (optional, default: 1)
                period=REQUEST_PERIOD_IN_MS (optional, default: NONE)
                format=json | protobuf (optional, default: Protobuf)
                ");
    }

    for arg in env::args().skip(1) {
        if arg.len() < 2 { panic!("Invalid argument: {}", arg) }
        let mut splitted = arg.split('=');
        let (k, val) = (splitted.next().expect(format!("Invalid argument: {}", arg).as_str()),
                        splitted.next().expect(format!("Invalid argument: {}", arg).as_str()));
        match k {
            "log" => match val {
                "yes" => { simple_logger::init_with_level(log::LogLevel::Info).unwrap(); },
                "debug" => { simple_logger::init_with_level(log::LogLevel::Debug).unwrap(); },
                "trace" => { simple_logger::init_with_level(log::LogLevel::Trace).unwrap(); },
                "no" => {},
                _ => panic!("Invalid logging mode. Use one of the following:
                            yes | no | debug | trace"),
            },
            "action" => {
                match val {
                    "latency" => {
                        debug!("Cache subscriber started measuring latency.");
                        action = "latency";
                    },
                    "throughput" => {
                        debug!("Cache subscriber started measuring throughput.");
                        action = "throughput";
                    },
                    "request" => {
                        debug!("Cache subscriber started");
                        action = "request";
                    },
                    "threads" => { threads = Some(usize::from_str(val)
                                            .expect("Invalid value for threads, expecting a usize.")) },
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
            "valuenr" => { valuenr = u32::from_str(val)
                                    .expect("Invalid value for value number, expecting a u32.") },
            "period" => { period = Some(u64::from_str(val)
                                    .expect("Invalid value for periods, expecting a u32.")) },
            "threads" => { threads = Some(usize::from_str(val)
                                    .expect("Invalid value for thread number, expecting a usize.")) },
            _ => panic!("Invalid argument: {}", arg),
        }
    }

    debug!("Started the cache subscriber.");

    let mut ctx = Context::new();

    let number = if valuenr > 0 { Some(valuenr) } else { None };

    match action {
        "latency" => {
            let iterations = 1000;
            let (mut req, mut sub) = get_sockets(&mut ctx);
            let mut param = Param(number, &mut req, &mut sub);
            let (dur, value_size) = average_request_time(&mut param, iterations, msg_format);
            info!("; {}; {}; {}; {};",
                    valuenr, (value_size - value_size%50), dur.as_secs(), dur.subsec_nanos());
            param.1.close().unwrap();
            param.2.close().unwrap();
        },
        "throughput" => {
            let iterations = 10;
            let (mut req, mut sub) = get_sockets(&mut ctx);
            let mut param = Param(number, &mut req, &mut sub);
            let (requests, value_size) = average_cache_throughput(&mut param, iterations, msg_format, None);
            info!("; {}; {}; {};",
                    valuenr, (value_size - value_size%50), requests);
            param.1.close().unwrap();
            param.2.close().unwrap();
        },
        "request" => {
            start_requester(&mut ctx, threads.unwrap_or(1), period, valuenr, msg_format);
        },
        _ => panic!("Nothing to do."),
    }

    ctx.destroy().unwrap();
}

fn get_sockets(ctx: &mut Context) -> (Socket, Socket) {
    let mut subscriber = ctx.socket(zmq::SUB).unwrap();
    subscriber.connect("tcp://localhost:5560").unwrap();
    // TODO subscribe to specific filters

    let filters: [(&str,&[u8]);6] = [
        ("clamp15", &[10, 7, 99, 108, 97, 109, 112, 49, 53]),
        ("invalid-voltage", &[10, 15, 105, 110, 118, 97, 108, 105, 100]),
        ("unclutch", &[10, 8, 117, 110, 99, 108, 117, 116, 99, 104]),
        ("speed-error", &[10, 11, 115, 112, 101, 101, 100, 45, 101, 114, 114, 111, 114]),
        ("speed-unsafe", &[10, 12, 115, 112, 101, 101, 100, 45, 117, 110, 115, 97, 102, 101]),
        ("sized", &[10, 5, 115, 105, 122, 101, 100]),
    ];

    for f in filters.iter() {
        subscriber.set_subscribe(format!("{}{}", "{\"msg_type\":\"", f.0).as_bytes()).unwrap();
        subscriber.set_subscribe(f.1).unwrap();
    }
    let mut requester: Socket = ctx.socket(zmq::REQ).unwrap();
    requester.connect("tcp://localhost:5550").unwrap();
    (requester, subscriber)
}

fn start_requester(ctx: &mut Context, threads: usize, period_ms: Option<u64>, valuenr: u32, msg_format: MsgFormat) {

    let mut handles = vec![];
    for _ in 0..threads {
        let mut socket = ctx.socket(zmq::REQ).unwrap();
        socket.connect("tcp://localhost:5550").unwrap();

        let period_duration = Duration::from_millis(period_ms.unwrap_or(0));
        let start = Instant::now();
        let mut wait_until = start + period_duration;

        // ... pass to a thread, request in a loop (no period vs period)
        handles.push(thread::spawn(move || {
            let filters = vec![slice_to_vec("sized".as_bytes())];
            let request = encode_sub_cache_msg(Some(valuenr), filters, "SUB", MsgPolicy::Plain, None, msg_format).unwrap();
            info!("request: {:?}", &request);
            loop {
                socket.send(&request, 0).unwrap();
                debug!("sent request.");
                socket.recv_bytes(0).unwrap();
                debug!("received response.");
                if period_ms.is_some() {
                    let now = Instant::now();
                    if wait_until > now {
                        let dur = wait_until - now;
                        debug!("sleeping for {:?}", dur);
                        sleep(dur); // TODO check, maybe transform period into (seconds, milliseconds)
                    }
                    wait_until = wait_until + period_duration;
                }
            }
        }));
    }
    info!("Spawned {} requester threads.", handles.len());
    for handle in handles {
        handle.join().unwrap();
    }
}

fn average_cache_throughput(param: &mut Param, n: u32, msg_format: MsgFormat, sleep_duration: Option<u64>)
-> (u32, usize) {
    let (requests, value_size) = cache_throughput(param, msg_format, sleep_duration);
    let n = if value_size !=0 { n } else { n-1 };
    let requests = (0..n-1).fold(requests,
                        |acc, _| acc + cache_throughput(param, msg_format, sleep_duration).0) / n;
    (requests, value_size)
}

fn cache_throughput(param: &mut Param, msg_format: MsgFormat, sleep_duration: Option<u64>)
-> (u32, usize) {
    param.1.send(&encode_cache_msg(vec![], "Start", MsgPolicy::Plain, None, 0, msg_format).unwrap(), 0).unwrap();
    let resp = param.1.recv_bytes(0).unwrap();
    debug!("resp: {:?}", resp);
    let mut value_size = 0;
    // try to receive one published message in order to get the message size
    for i in 1..10 {
        match param.2.recv_bytes(DONTWAIT) {
            Ok(v) => {
                value_size = v.len();
                break;
            },
            _ => {
                debug!("Need to wait for a published message.");
                if i < 10 { sleep(Duration::from_millis(i*100)); }
            },
        }
    }
    sleep(Duration::from_secs(sleep_duration.unwrap_or(5)));    // 5s for throughput measurement

    param.1.send(&encode_cache_msg(vec![], "Stop", MsgPolicy::Plain, None, 0, msg_format).unwrap(), 0).unwrap();
    let result_msg = decode_cache_msg(param.1.recv_bytes(0).unwrap(), msg_format).unwrap();
    debug!("received: {:?}", result_msg);
    (decode_u32_msg(result_msg.msg, msg_format).unwrap(), value_size)
}

fn average_request_time(param: &mut Param, iterations: u32, msg_format: MsgFormat) -> (Duration, usize) {
    let (dur, value_size) = measure_cached_subscription(param, msg_format);
    let dur = (0..iterations-1).fold(dur, |acc, _| acc + measure_cached_subscription(param, msg_format).0) / iterations;
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

    let filters = slice_to_vec(&[
        slice_to_vec(&("clamp15".as_bytes())),
        slice_to_vec(&("invalid-voltage".as_bytes())),
        slice_to_vec(&("speed-error".as_bytes())),
        slice_to_vec(&("speed-unsafe".as_bytes())),
        slice_to_vec(&("unclutch".as_bytes())),
        slice_to_vec(&("sized".as_bytes())),
        ]);
    let request = encode_sub_cache_msg(param.0, filters, "SUB", MsgPolicy::Plain, None, msg_format).unwrap();
    param.1.send(&request, 0).unwrap();

    let resp = param.1.recv_bytes(0).unwrap();
    let cache_msg = decode_cache_msg(resp, msg_format).unwrap();
    let total_values = decode_u32_msg(cache_msg.msg, msg_format).unwrap();
    let mut value_size = 0;

    for _ in 0..total_values {
        let value = param.2.recv_bytes(0).unwrap();
        if value_size != 0 { assert_eq!(value_size, value.len()); }
        value_size = value.len();
        trace!("received value: {:?}", value);
    }

    debug!("Received {} values.", total_values);
    value_size
}
