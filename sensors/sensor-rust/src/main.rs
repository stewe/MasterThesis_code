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
use std::string::ToString;
use std::time::Duration;
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
    simple_logger::init().unwrap();
    info!("Sensor started.");

    let mut sensor_type = SensorType::Undefined;
    let mut policy = MsgPolicy:: Plain;
    let mut format = MsgFormat::Protobuf;

    let key  = [0u8;16];
    let mut key = Some((0..16).into_iter().fold(key, |mut acc, x| { acc[x] = x as u8; acc }));
    info!("key: {:?}", key);

    for arg in env::args().skip(1) {
        if arg.len() < 2 { panic!("Invalid argument: {}", arg) }
        let (k, val) = arg.split_at(2);
        match k {
            "t=" => {
                sensor_type = match val {
                    "unclutch" => SensorType::Unclutch,
                    "voltage" => SensorType::InvalidVoltage,
                    "speed-error" => SensorType::SpeedError,
                    "speed-unsafe" => SensorType::SpeedUnsafe,
                    "clamp15" => SensorType::Clamp15,
                    _ => panic!("Unknwon sensor type. Use one of the following:\nunclutch voltage speed-error speed-unsafe clamp15"),
                }
            },
            "p=" => {
                policy = match val {
                    "plain" => { key = None; MsgPolicy::Plain },
                    "mac" => MsgPolicy::Authenticated,
                    "cipher" => MsgPolicy::Encrypted,
                    _ => panic!("Unknown security policy, use one of the following: mac cipher plain")
                }
            },
            "f=" => {
                format = match val {
                    "json" => MsgFormat::Json,
                    "protobuf" => MsgFormat::Protobuf,
                    _ => panic!("Unknwon message format. Use one of the following:\njson protobuf"),
                }
            },
            _ => panic!("Invalid argument: {}", arg),
        }
    }

    if sensor_type == SensorType::Undefined { panic!("Specify one of the following sensor types:\n unclutch voltage speed-error speed-unsafe clamp15") }

    let mut ctx = Context::new();
    let mut socket: Socket = ctx.socket(zmq::PUB).unwrap();
    socket.bind("tcp://*:5551").unwrap();

loop {
    for _ in 0..5 {
        let value = match sensor_type {
            SensorType::Clamp15 => {
                let rnd = rand::random();
                encode_u8_msg(rnd, "pub/clamp15", policy.clone(), key, format).unwrap()
            },
            _ => {
                let rnd = rand::random();
                encode_bool_msg(rnd, format!("pub/{}", sensor_type.to_string()).as_str(), policy.clone(), key, format).unwrap()
            },
        };
        socket.send(&value, 0).unwrap();

        let mut v = value.clone();
        v.split_off(16);
        info!("sent: {:?}", String::from_utf8(v).unwrap());

        sleep(Duration::new(1,0));
    }

    sleep(Duration::new(10,0));
}

}
