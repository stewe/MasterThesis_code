extern crate msg_lib;
#[macro_use]
extern crate log;
// extern crate env_logger;
extern crate simple_logger;
extern crate rand;
extern crate rustc_serialize;
extern crate zmq;

use std::env;
use std::fmt::{Display, Error, Formatter};
use std::str::FromStr;
use std::string::ToString;
use std::time::{Duration, Instant};
use std::thread::sleep;
use msg_lib::{encode_bool_msg, encode_bytes_msg, encode_u8_msg,
                MsgFormat, MsgPolicy};
use zmq::{Socket, Context};

#[derive(PartialEq)]
enum SensorType {
    Unclutch,
    InvalidVoltage,
    SpeedError,
    SpeedUnsafe,
    Clamp15,
    Sized,
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
                            &SensorType::Sized => "sized",
                            _ => "undefined", })
    }
}

fn main() {

    let mut sensor_type = SensorType::Undefined;
    let mut policy = MsgPolicy:: Plain;
    let mut format = MsgFormat::Protobuf;
    let mut port = String::new();
    let mut period = 100u64;
    let mut size = 0u32;

    let key  = [0u8;16];
    let mut key = Some((0..16).into_iter().fold(key, |mut acc, x| { acc[x] = x as u8; acc }));

    if env::args().len() == 1 {
        panic!("Use the following parameters:
                type=unclutch | invalid-voltage | speed-error | speed-unsafe | clamp15 | sized
                port=PORTNR
                policy=plain | mac | cipher (optional, default: plain)
                format=json | protobuf (optional, default: Protobuf)
                period=MILLISECONDS (optional, default: 100)
                log=y | n (optional, default: n)
                size=SIZE_IN_BYTES (required for sized)
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
                    "invalid-voltage" => SensorType::InvalidVoltage,
                    "speed-error" => SensorType::SpeedError,
                    "speed-unsafe" => SensorType::SpeedUnsafe,
                    "clamp15" => SensorType::Clamp15,
                    "sized" => SensorType::Sized,
                    _ => panic!("Unknown sensor type. Use one of the following:
                                type=unclutch | invalid-voltage | speed-error | speed-unsafe | clamp15 | sized"),
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
            "log" => match val {
                "yes" => { simple_logger::init_with_level(log::LogLevel::Info).unwrap(); },
                "debug" => { simple_logger::init_with_level(log::LogLevel::Debug).unwrap(); },
                "trace" => { simple_logger::init_with_level(log::LogLevel::Trace).unwrap(); },
                "no" => {},
                _ => panic!("Invalid logging mode. Use one of the following:
                            yes | no | debug | trace"),
            },
            "size" => {
                size = u32::from_str(val).expect("Invalid value for size. Use bytes as u32.");
            }
            _ => panic!("Invalid argument: {}", arg),
        }
    }

    if sensor_type == SensorType::Undefined {
        panic!("Specify one of the following sensor types:
                unclutch invalid-voltage speed-error speed-unsafe clamp15 sized") }
    if sensor_type == SensorType::Sized && size == 0 {
        panic!("Specify 'size' in bytes for sensor type Sized!.")
    }

    info!("Sensor {} started.", sensor_type);
    if sensor_type == SensorType::Sized {
        info!("value size: {}", size);
    }
    debug!("key: {:?}", key);


    let mut ctx = Context::new();
    let mut socket: Socket = ctx.socket(zmq::PUB).unwrap();
    socket.bind(format!("tcp://*:{}", port).as_str()).unwrap();


let period_duration = Duration::from_millis(period);
let start = Instant::now();
let mut wait_until = start + period_duration;

loop {
    let value = match sensor_type {
        SensorType::Sized => {
            let rnd: Vec<u8> = (0..size).fold(vec![], |mut acc, _| { acc.push(rand::random()); acc });
            encode_bytes_msg(rnd, "sized", policy.clone(), key, format).unwrap()
        },
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

    debug!("sent: ({} bytes) {:?}", value.len(), value);

    let now = Instant::now();
    if wait_until > now {
        sleep(wait_until - now);
    }
    wait_until = wait_until + period_duration;
}

}
