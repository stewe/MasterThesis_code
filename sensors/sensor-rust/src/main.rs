extern crate msg_lib;
#[macro_use]
extern crate log;
// extern crate env_logger;
extern crate simple_logger;
extern crate rand;
extern crate rustc_serialize;
extern crate time;
extern crate zmq;

use std::env;
use std::fmt::{Display, Error, Formatter};
use std::str::FromStr;
use std::string::ToString;
use std::time::{Duration};
use std::thread::sleep;
use msg_lib::{encode_bool_msg, encode_u8_msg, MsgFormat, MsgPolicy};
use zmq::{Socket, Context};

#[derive(PartialEq)]
enum SensorType {
    Unclutch,
    InvalidVoltage,
    SpeedError,
    SpeedUnsafe,
    Clamp15,
    Undefined,
}

impl Display for SensorType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", match self {
                            &SensorType::Unclutch => "unclutch",
                            &SensorType::InvalidVoltage => "invalid-voltage",
                            &SensorType::SpeedError => "speed-error",
                            &SensorType::SpeedUnsafe => "speed-unsafe",
                            &SensorType::Clamp15 => "clamp15",
                            _ => "undefined", })
    }
}

fn main() {

    let mut sensor_type = SensorType::Undefined;
    let mut policy = MsgPolicy:: Plain;
    let mut format = MsgFormat::Protobuf;
    let mut port = String::new();
    let mut period = 100u64;
    let mut logging = false;

    let key  = [0u8;16];
    let mut key = Some((0..16).into_iter().fold(key, |mut acc, x| { acc[x] = x as u8; acc }));
    info!("key: {:?}", key);

    if env::args().len() == 1 {
        panic!("Use the following parameters:
                type=unclutch | voltage | speed-error | speed-unsafe | clamp15
                port=PORTNR
                policy=plain | mac | cipher (optional, default: plain)
                format=json | protobuf (optional, default: Protobuf)
                period=MILLISECONDS (optional, default: 100)
                log=y | n (optional, default: n)
                ");
    }

    for arg in env::args().skip(1) {
        if arg.len() < 2 { panic!("Invalid argument: {}", arg) }
        let mut splitted = arg.split('=');
        let (k, val) = (splitted.next().expect(format!("Invalid argument: {}", arg).as_str()), splitted.next().expect(format!("Invalid argument: {}", arg).as_str()));
        match k {
            "type" => {
                sensor_type = match val {
                    "unclutch" => SensorType::Unclutch,
                    "voltage" => SensorType::InvalidVoltage,
                    "speed-error" => SensorType::SpeedError,
                    "speed-unsafe" => SensorType::SpeedUnsafe,
                    "clamp15" => SensorType::Clamp15,
                    _ => panic!("Unknown sensor type. Use one of the following:\ntype=unclutch | voltage | speed-error | speed-unsafe | clamp15"),
                }
            },
            "policy" => {
                policy = match val {
                    "plain" => { key = None; MsgPolicy::Plain },
                    "mac" => MsgPolicy::Authenticated,
                    "cipher" => MsgPolicy::Encrypted,
                    _ => panic!("Unknown security policy, use one of the following: mac cipher plain")
                }
            },
            "format" => {
                format = match val {
                    "json" => MsgFormat::Json,
                    "protobuf" => MsgFormat::Protobuf,
                    _ => panic!("Unknown message format. Use one of the following:\njson protobuf"),
                }
            },
            "port" => {
                port = String::new() + val;
            },
            "period" => {
                period = u64::from_str(val).expect("Invalid value for period. Use milliseconds as u64.");
            },
            "log" => {
                match val {
                "y" => { logging = true; },
                "no" => {},
                _ => panic!("Invalid value for logging. Use y or n."),
                }
            },
            _ => panic!("Invalid argument: {}", arg),
        }
    }

    if sensor_type == SensorType::Undefined { panic!("Specify one of the following sensor types:\n unclutch voltage speed-error speed-unsafe clamp15") }

    if logging { simple_logger::init().unwrap(); }
    info!("Sensor {} started.", sensor_type);

    let mut ctx = Context::new();
    let mut socket: Socket = ctx.socket(zmq::PUB).unwrap();
    socket.bind(format!("tcp://*:{}", port).as_str()).unwrap();


let period_duration = Duration::from_millis(period);
// let start = time::SteadyTime::now();

loop {
// let num = 1000;
// for _ in 0..num {
    let value = match sensor_type {
        SensorType::Clamp15 => {
            let rnd = rand::random();
            encode_u8_msg(rnd, "clamp15", policy.clone(), key, format).unwrap()
        },
        _ => {
            let rnd = rand::random();
            encode_bool_msg(rnd, sensor_type.to_string().as_str(), policy.clone(), key, format).unwrap()
        },
    };
    socket.send(&value, 0).unwrap();

    // let mut v = value.clone();
    // v.split_off(16);
    // info!("sent: {:?}", String::from_utf8(v).unwrap());

    info!("sent: {:?}", value);

    sleep(period_duration); // TODO check, maybe transform period into (seconds, milliseconds)
}

// let duration = time::SteadyTime::now() - start;
// let duration_nanos = duration.num_nanoseconds().unwrap();
// println!("time for {} iterations: {:?}", num, duration);
// println!("i.e. overhead for serialization (, encryption) and sending per msg in nanos: {:?}", duration_nanos/(num as i64) - ((period*1000000) as i64));

// socket.close().unwrap();
// ctx.destroy().unwrap();



}
