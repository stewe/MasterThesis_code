extern crate chan_signal;
extern crate enclave_interface;
extern crate interface;
#[macro_use] extern crate log;
extern crate simple_logger;
extern crate sgxs;
extern crate sgx_isa;
extern crate zmq;

use chan_signal::Signal;
use interface::ECall;
use sgx_isa::Sigstruct;
use std::fs::File;
use std::io::{Error as IoError,self};
use std::mem::transmute;
use std::ops::Deref;
use std::slice::from_raw_parts;
use std::sync::Arc;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use zmq::{Socket, Context, DONTWAIT, poll, POLLIN};

fn next_arg(args: &mut std::env::ArgsOs) -> Result<std::ffi::OsString,IoError> {
    args.next().ok_or(IoError::new(io::ErrorKind::InvalidInput,"missing argument"))
}

fn parse_args() -> Result<(File,Sigstruct,File,Sigstruct),(&'static str,IoError)> {
    use enclave_interface::util::read_sigstruct;

    let mut args=std::env::args_os();
    args.next(); // skip arg[0]
    let file=try!(next_arg(&mut args).and_then(File::open).map_err(|err|("file",err)));
    let sig=try!(next_arg(&mut args).and_then(File::open).and_then(|mut f|read_sigstruct(&mut f)).map_err(|err|("sig",err)));
    let le_file=try!(next_arg(&mut args).and_then(File::open).map_err(|err|("le_file",err)));
    let le_sig=try!(next_arg(&mut args).and_then(File::open).and_then(|mut f|read_sigstruct(&mut f)).map_err(|err|("le_sig",err)));
    if let Some(arg) = args.next() {
        let arg = arg.into_string().unwrap();
        let mut splitted = arg.split('=');
        let (k, val) = (splitted.next().expect(format!("Invalid argument: {}", arg).as_str()), splitted.next().expect(format!("Invalid argument: {}", arg).as_str()));
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
                _ => panic!("Unknown argument! Arguments: [log=yes|no|debug|trace]")
            }
        }

    Ok((file,sig,le_file,le_sig))
}


fn main() {
    use sgxs::loader::{Load,Map,OptionalEinittoken as OptTok};
    use enclave_interface::tcs;

    let (mut file,sig,mut le_file,le_sig) = match parse_args() {
        Ok(res) => res,
        Err((arg,err)) => {
            error!("Usage: sgx-first-steps-untrusted <file> <sig> <le_file> <le_sig> log=debug");
            error!("\nError with argument `{}': {}",arg,err);
            std::process::exit(1);
        }
    };


   let dev = match sgxs::isgx::Device::open("/dev/isgx") {
       Ok(d) => d,
       Err(_) => {
           sgxs::isgx::Device::open("/dev/sgx").unwrap()
       },
   };
    let mut mapping = dev.load_with_launch_enclave(&mut file,&sig,OptTok::None(None),&mut le_file,&le_sig).unwrap();

    if log_enabled!(log::LogLevel::Debug) {
        let _ = enclave_interface::debug::install_segv_signal_handler(&mut mapping.tcss()[0]);
        debug!("Enabled debugging features for the enclave.")
    }

    // init user heap for enclave
    let heap_size: usize = 4096*1024*1*128; // TODO size sufficient? max value size * capacity * nr of sensors * overhead
    let heap_base = Arc::new(unsafe{  memalign(4096, heap_size) as u64 });
    info!("INFO! heap_base: {:?}", heap_base);
    let _ = tcs::enter(&mut mapping.tcss()[0],
                                        &ocall_void,
                                        ECall::InitUserHeap as u64, *heap_base.deref(), heap_size as u64, 0, 0);

    // Listen to Signals 'INT' and 'TERM' in a dedicated thread and free memory before exit.
    let signal = chan_signal::notify(&[Signal::INT, Signal::TERM]);
    let heap_base = heap_base.clone();
    thread::spawn(move || {
        match signal.recv() {
            Some(signal) => {
                info!("Received signal: {:?}", signal);
                let addr = (*heap_base.deref()) as *mut u8;
                unsafe{ free(addr) };
                info!("Freed allocated memory.");
            },
            _ => info!("Receive returned None"),
        };
        std::process::exit(0);
    });

    let mut ctx = Context::new();
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

    info!("Cache started.");

    loop {
        // read message from zmq socket

        // ZeroMQ:
        // If none of the requested events have occurred on any zmq_pollitem_t item,
        // zmq_poll() shall wait timeout milliseconds for an event to occur on any of
        // the requested items. If the value of timeout is 0, zmq_poll() shall return immediately.
        // If the value of timeout is -1, zmq_poll() shall block indefinitely until
        // a requested event has occurred on at least one zmq_pollitem_t.

        // OSX man poll:
        // If timeout is greater than zero, it specifies a maximum interval (in mil-
        // liseconds) to wait for any file descriptor to become ready.  If timeout
        // is zero, then poll() will return without blocking. If the value of
        // timeout is -1, the poll blocks indefinitely.

        match poll(&mut [subscriber.as_poll_item(POLLIN), responder.as_poll_item(POLLIN)], -1) {
            Ok(n) => {
                if n > 0 {
                    loop {
                        match responder.recv_bytes(DONTWAIT) {
                            Ok(msg) =>  {
                                debug!("msg: {:?}", &msg);

                                let input = Box::new(msg);
                                let input_addr = std::boxed::Box::into_raw(input) as u64;
                                let time = get_time_in_millis();
                                let ret = tcs::enter(&mut mapping.tcss()[0],
                                                        &ocall_void,
                                                        ECall::HandleRequest as u64, input_addr, 0, 0, time);

                                let resp_ptr = unsafe{ (ret as *const u64).offset(1) };
                                let resp_len = unsafe{ *(resp_ptr) };
                                let mut resp_msg_addrs = unsafe{ from_raw_parts(resp_ptr.offset(1), resp_len as usize).to_vec() };
//                                debug!("responses: {:?}", &resp_msg_addrs);
                                // resolve msgs from addresses
                                let mut responses = vec![];
                                for _ in 0..resp_msg_addrs.len() {
                                    let r_ptr = resp_msg_addrs.pop().unwrap() as *const u8;
                                    let mut r_len_b = [0u8, 0u8];
                                    r_len_b[0] = unsafe{ *(r_ptr.offset(0)) };
                                    r_len_b[1] = unsafe{ *(r_ptr.offset(1)) };
                                    let r_len: u16 = unsafe{ transmute(r_len_b) };
                                    let resp = unsafe{ from_raw_parts(r_ptr.offset(2), r_len as usize).to_vec() };
                                    debug!("response (len): ({}) {:?}", r_len, resp);
                                    responses.push(resp);
                                }

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
                                let input = Box::new(msg);
                                let input_addr = std::boxed::Box::into_raw(input) as u64;
                                let time = get_time_in_millis();
                                let _ = tcs::enter(&mut mapping.tcss()[0],
                                                        &ocall_void,
                                                        ECall::HandleSubMsg as u64, input_addr, 0, 0, time);
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

fn ocall_void(p1: u64, p2: u64, p3: u64, p4: u64, p5: u64) -> u64 {
    info!("USERCALL: ocall_void; params: {}, {}, {}, {}, {}", p1, p2, p3, p4, p5);
    0
}

extern "C" {
        /// We need this to allocate aligned memory for our heap.
        fn memalign(alignment: usize, size: usize) -> *mut u8;

        // Release our memory.
        fn free(ptr: *mut u8);
}

fn get_time_in_millis() -> u64 {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    (now.as_secs() * 1000) + (now.subsec_nanos() / 1000000) as u64
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

    // socket.set_subscribe(&[]).unwrap(); // every message

    socket
}
