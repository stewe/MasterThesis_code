#[macro_use]
extern crate log;
extern crate simple_logger;
extern crate msg_lib;
extern crate zmq;
extern crate enclave_cache;

use std::env;
use enclave_cache::cache_enclave;
use msg_lib::MsgFormat;
use zmq::{Socket, Context, DONTWAIT, poll, POLLIN};


// ------------------------------------------------------------------------------------
fn main() {

    let args = env::args();
    if args.len() > 1 {
        for arg in env::args().skip(1) {
            let mut splitted = arg.split('=');
            let (k, val) = (splitted.next().expect(format!("Invalid argument: {}", arg).as_str()),
                            splitted.next().expect(format!("Invalid argument: {}", arg).as_str()));
            match k {
                "log" => {
                    match val {
                        "yes" => { simple_logger::init_with_level(log::LogLevel::Info).unwrap(); },
                        "debug" => { simple_logger::init_with_level(log::LogLevel::Debug).unwrap(); },
                        "trace" => { simple_logger::init_with_level(log::LogLevel::Trace).unwrap(); },
                        "no" => {},
                        _ => panic!("Invalid logging mode. Use one of the following:
                                    yes | no | debug | trace"),
                    }
                },
                "format" => {
                    match val {
                        "json" => unsafe { cache_enclave::MSG_FORMAT = MsgFormat::Json; },
                        "protobuf" => unsafe { cache_enclave::MSG_FORMAT = MsgFormat::Protobuf; },
                        _ => panic!("Invalid message format. Use one of the following:
                                    protobuf | json"),
                    }
                },
                _ => panic!("Unknown argument! Arguments: [log=yes|no|debug|trace] [format=json|protobuf]")
            }
        }
    }

    info!("Cache started.");

    let mut ctx = Context::new();
    // REP socket for subscription requests
    let mut responder: Socket = ctx.socket(zmq::REP).unwrap();
    responder.bind("tcp://*:5550").unwrap();

    let mut publisher: Socket = ctx.socket(zmq::PUB).unwrap();
    publisher.set_sndhwm(5000i32).unwrap();
    publisher.bind("tcp://*:5560").unwrap();
    // the first published message will not arrive -> send some dummy message(s)
    publisher.send("Cache Service initialized.".as_bytes(), 0).unwrap();    // will not arrive

    debug!("publisher: maxmsgsize={:?}\tsndhwm={:?}",
        publisher.get_maxmsgsize().unwrap(), publisher.get_sndhwm().unwrap());

    let mut subscriber = init_subscriber_socket(&mut ctx);

    loop {
        // read message from zmq socket
        match poll(&mut [subscriber.as_poll_item(POLLIN), responder.as_poll_item(POLLIN)], -1) {
            Ok(n) => {
                if n > 0 {
                    loop {
                        match responder.recv_bytes(DONTWAIT) {
                            Ok(msg) =>  {
                                debug!("msg: {:?}", &msg);
                                let responses = cache_enclave::ecall_handle_request(msg);
                                match responses.len() {
                                    0 => {responder.send(&[], 0).unwrap(); },
                                    1 => { responder.send(&responses.first().unwrap(), 0).unwrap(); },
                                    _ => {
                                        responder.send(&responses.first().unwrap(), 0).unwrap();
                                        let mut i  = 0;
                                        for resp in responses.iter().skip(1) {
                                            publisher.send(&resp, 0).unwrap();
                                            i = i + 1;
                                        }
                                        debug!("returned {} msgs.", i);
                                    },
                                }
                            },
                            _  => { break }
                        }
                    }
                    loop {
                        match subscriber.recv_bytes(DONTWAIT) {
                            Ok(msg) =>  {
                                trace!("msg: {:?}", &msg);
                                let _ = cache_enclave::ecall_handle_sub_msg(msg);
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

pub fn init_subscriber_socket(ctx: &mut Context) -> Socket {
    let mut socket: Socket = ctx.socket(zmq::SUB).unwrap();
    socket.connect("tcp://localhost:5551").unwrap();    // unclutch
    socket.connect("tcp://localhost:5552").unwrap();    // invalid-voltage
    socket.connect("tcp://localhost:5553").unwrap();    // speed-error
    socket.connect("tcp://localhost:5554").unwrap();    // speed-unsafe
    socket.connect("tcp://localhost:5555").unwrap();    // clamp15
    socket.connect("tcp://localhost:5559").unwrap();    // sized

    let filters: [(&str,&[u8]);6] = [
        ("clamp15", &[10, 7, 99, 108, 97, 109, 112, 49, 53]),
        ("invalid-voltage", &[10, 15, 105, 110, 118, 97, 108, 105, 100]),
        ("unclutch", &[10, 8, 117, 110, 99, 108, 117, 116, 99, 104]),
        ("speed-error", &[10, 11, 115, 112, 101, 101, 100, 45, 101, 114, 114, 111, 114]),
        ("speed-unsafe", &[10, 12, 115, 112, 101, 101, 100, 45, 117, 110, 115, 97, 102, 101]),
        ("sized", &[10, 5, 115, 105, 122, 101, 100]),
    ];

    for f in filters.iter() {
        socket.set_subscribe(format!("{}{}", "{\"msg_type\":\"", f.0).as_bytes()).unwrap();
        socket.set_subscribe(f.1).unwrap();
    }

    socket
}
