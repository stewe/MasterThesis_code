// simple dog clutch checker

extern crate msg_lib;
extern crate rustc_serialize;
extern crate zmq;

use std::fmt;
use std::env;
use msg_lib::{decode_cache_msg, decode_bool_msg, decode_u8_msg, encode_sub_cache_msg,
                slice_to_vec, validate_cache_msg,
                MsgFormat, MsgPolicy};
use zmq::{Socket, Context};


struct InputT {
    i6_unclutch: bool,
    i5_invalid_voltage: bool,
    i4_speed_error: bool,
    i3_speed_unsafe: bool,
    i2_clamp15: u8
}

struct InputTBuffer {
    i6_unclutch: Vec<bool>,
    i5_invalid_voltage: Vec<bool>,
    i4_speed_error: Vec<bool>,
    i3_speed_unsafe: Vec<bool>,
    i2_clamp15: Vec<u8>
}

impl InputTBuffer {
    fn new ()  -> Self {
        InputTBuffer {
            i6_unclutch: vec![],
            i5_invalid_voltage: vec![],
            i4_speed_error: vec![],
            i3_speed_unsafe: vec![],
            i2_clamp15: vec![],
        }
    }

    fn add_i6_unclutch(&mut self, val: bool) {
        self.i6_unclutch.push(val);
    }

    fn add_i5_invalid_voltage(&mut self, val: bool) {
        self.i5_invalid_voltage.push(val);
    }

    fn add_i4_speed_error(&mut self, val: bool) {
        self.i4_speed_error.push(val);
    }
    fn add_i3_speed_unsafe(&mut self, val: bool) {
        self.i3_speed_unsafe.push(val);
    }
    fn add_i2_clamp15(&mut self, val: u8) {
        self.i2_clamp15.push(val);
    }

    fn build(&mut self) -> Option<InputT> {
        if self.i6_unclutch.is_empty()
            || self.i5_invalid_voltage.is_empty()
            || self.i4_speed_error.is_empty()
            || self.i3_speed_unsafe.is_empty()
            || self.i2_clamp15.is_empty()
        { None } else {
            Some(
                InputT {
                    i6_unclutch: self.i6_unclutch.pop().unwrap(),
                    i5_invalid_voltage: self.i5_invalid_voltage.pop().unwrap(),
                    i4_speed_error: self.i4_speed_error.pop().unwrap(),
                    i3_speed_unsafe: self.i3_speed_unsafe.pop().unwrap(),
                    i2_clamp15: self.i2_clamp15.pop().unwrap(),
                })
            }
    }
}

struct OutputT {
    o1_safe_state: bool,
    o2_state_reset: bool
}

struct ConfigT {
    c1_debounce1: u8,
    c2_debounce2: u8,
    c3_rising_edge: bool    // true = rising edge
}

struct StateT {
    curr_err: u16,
    curr_qerr: u16,
    clamp15: bool
}



impl fmt::Debug for InputT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!{f, "unclutch: {}, invalid voltage: {}, speed error: {}, speed unsafe: {}, clamp15: {}", self.i6_unclutch, self.i5_invalid_voltage, self.i4_speed_error, self.i3_speed_unsafe, self.i2_clamp15}
    }
}

impl fmt::Display for InputT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl fmt::Debug for OutputT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "safe state: {}, state reset: {}", self.o1_safe_state, self.o2_state_reset)
    }
}

impl fmt::Display for OutputT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}



impl ConfigT {
    fn new() -> ConfigT {
        ConfigT {
            c1_debounce1: 5, // 5 times
            c2_debounce2: 5, // 5 times
            c3_rising_edge: true
        }
    }
}

impl fmt::Debug for ConfigT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "debounce1: {}, debounce2: {}, rising edge: {}", self.c1_debounce1, self.c2_debounce2, self.c3_rising_edge)
    }
}

impl fmt::Display for ConfigT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}



impl StateT {
    fn new(config: &ConfigT) -> StateT {
        StateT {
            curr_err: 0,
            curr_qerr: 0,
            clamp15: config.c3_rising_edge
        }
    }
}

impl fmt::Debug for StateT {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "curr_err: {}, curr_qerr: {}, clamp15: {})", self.curr_err, self.curr_qerr, self.clamp15)
    }
}

impl fmt::Display for StateT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}


fn safety_check(inp: &InputT, conf: &ConfigT, state: &mut StateT) -> OutputT {
    let mut safe_state = false;
    let unsafe_speed = inp.i4_speed_error || inp.i3_speed_unsafe;
    if inp.i6_unclutch && unsafe_speed {
        if state.curr_err+1 >= conf.c1_debounce1 as u16 {
           safe_state = true;
        } else {
            state.curr_err = state.curr_err + 1;
        }
    } else {
        state.curr_err = 0;
    }

    if inp.i5_invalid_voltage && unsafe_speed {
        if state.curr_qerr+1 >= conf.c2_debounce2 as u16 {
           safe_state = true;
        } else {
            state.curr_qerr = state.curr_qerr + 1;
        }
    } else {
        state.curr_qerr = 0;
    }

    let mut state_reset = false;
    let mut clamp15 = false;
    if inp.i2_clamp15 != 0 {
        clamp15 = true;
    }

    if clamp15 != state.clamp15 {   // rising edge?
        if conf.c3_rising_edge && clamp15 { // configured rising edge && rising edge?
            state_reset = true;
        } else if !conf.c3_rising_edge && !clamp15 { // configured falling edge & falling edge ?
            state_reset = true;
        }
    }

    state.clamp15 = inp.i2_clamp15.ne(&0u8);

    OutputT {
        o1_safe_state: safe_state,
        o2_state_reset: state_reset
    }
}

pub fn main() {
    if env::args().len() > 1 {
        with_cache();
    } else {
        without_cache();
    }
}

fn with_cache() {

    let msg_format = MsgFormat::Protobuf;
    let key  = [0u8;16];
    let key = (0..16).into_iter().fold(key, |mut acc, x| { acc[x] = x as u8; acc });

    let config = ConfigT::new();
    let mut state = StateT::new(&config);
    let mut input_buffer = InputTBuffer::new();
    let mut inputs = vec![];

    let mut ctx = Context::new();
    let mut requester: Socket = ctx.socket(zmq::REQ).unwrap();
    requester.connect("tcp://localhost:5550").unwrap();
    let mut subscriber = ctx.socket(zmq::SUB).unwrap();
    subscriber.connect("tcp://localhost:5560").unwrap();
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

    let filters = slice_to_vec(&[
        slice_to_vec(&("clamp15".as_bytes())),
        slice_to_vec(&("invalid-voltage".as_bytes())),
        slice_to_vec(&("speed-error".as_bytes())),
        slice_to_vec(&("speed-unsafe".as_bytes())),
        slice_to_vec(&("unclutch".as_bytes())),
        ]);
    let request = encode_sub_cache_msg(Some(5), filters, "SUB", MsgPolicy::Plain, None, msg_format).unwrap();

    requester.send(&request, 0).unwrap();
    let _ = requester.recv_bytes(0);

    loop {
        let msg = subscriber.recv_bytes(0).unwrap();
        let msg_decoded = decode_cache_msg(msg, msg_format).unwrap();
        if !validate_cache_msg(&msg_decoded, key) {panic!("Received corrupted messsage.")};
        match msg_decoded.msg_type.as_str() {
            "clamp15" => { input_buffer.add_i2_clamp15(decode_u8_msg(msg_decoded.msg, msg_format).unwrap()); },
            "invalid-voltage" => { input_buffer.add_i5_invalid_voltage(decode_bool_msg(msg_decoded.msg, msg_format).unwrap()); },
            "unclutch" => { input_buffer.add_i6_unclutch(decode_bool_msg(msg_decoded.msg, msg_format).unwrap()); },
            "speed-error" => { input_buffer.add_i4_speed_error(decode_bool_msg(msg_decoded.msg, msg_format).unwrap()); },
            "speed-unsafe" => { input_buffer.add_i3_speed_unsafe(decode_bool_msg(msg_decoded.msg, msg_format).unwrap()); },
            _ => panic!("Received unknown message type."),
        };
        if let Some(input) = input_buffer.build() {
            inputs.push(input);
        }
        if inputs.len() >= 5 { break; }
    }

    for input in inputs {
        // println!("state: {:?}", state);
        safety_check(&input, &config, &mut state);
    }
    // println!("final state: {:?}", state)
}

fn without_cache() {
    let msg_format = MsgFormat::Protobuf;
    let key  = [0u8;16];
    let key = (0..16).into_iter().fold(key, |mut acc, x| { acc[x] = x as u8; acc });

    let mut ctx = Context::new();
    let mut socket = init_subscriber_socket(&mut ctx);

    let config = ConfigT::new();
    let mut state = StateT::new(&config);
    let mut input_buffer = InputTBuffer::new();
    let mut inputs = vec![];

    loop {
        let msg = socket.recv_bytes(0).unwrap();
        let msg_decoded = decode_cache_msg(msg, msg_format).unwrap();
        if !validate_cache_msg(&msg_decoded, key) {panic!("Received corrupted messsage.")};
        match msg_decoded.msg_type.as_str() {
            "clamp15" => { input_buffer.add_i2_clamp15(decode_u8_msg(msg_decoded.msg, msg_format).unwrap()); },
            "invalid-voltage" => { input_buffer.add_i5_invalid_voltage(decode_bool_msg(msg_decoded.msg, msg_format).unwrap()); },
            "unclutch" => { input_buffer.add_i6_unclutch(decode_bool_msg(msg_decoded.msg, msg_format).unwrap()); },
            "speed-error" => { input_buffer.add_i4_speed_error(decode_bool_msg(msg_decoded.msg, msg_format).unwrap()); },
            "speed-unsafe" => { input_buffer.add_i3_speed_unsafe(decode_bool_msg(msg_decoded.msg, msg_format).unwrap()); },
            _ => panic!("Received unknown message type."),
        };
        if let Some(input) = input_buffer.build() {
            inputs.push(input);
        }
        if inputs.len() >= 5 { break; }
    }

    for input in inputs {
        safety_check(&input, &config, &mut state);
    }
}

fn init_subscriber_socket(ctx: &mut Context) -> Socket {
    let mut socket: Socket = ctx.socket(zmq::SUB).unwrap();
    socket.connect("tcp://localhost:5551").unwrap();    // unclutch
    socket.connect("tcp://localhost:5552").unwrap();    // invalid-voltage
    socket.connect("tcp://localhost:5553").unwrap();    // speed-error
    socket.connect("tcp://localhost:5554").unwrap();    // speed-unsafe
    socket.connect("tcp://localhost:5555").unwrap();    // clamp15
    socket.connect("tcp://localhost:5559").unwrap();    // sized

    let filters: [(&str,&[u8]);5] = [
        ("clamp15", &[10, 7, 99, 108, 97, 109, 112, 49, 53]),
        ("invalid-voltage", &[10, 15, 105, 110, 118, 97, 108, 105, 100]),
        ("unclutch", &[10, 8, 117, 110, 99, 108, 117, 116, 99, 104]),
        ("speed-error", &[10, 11, 115, 112, 101, 101, 100, 45, 101, 114, 114, 111, 114]),
        ("speed-unsafe", &[10, 12, 115, 112, 101, 101, 100, 45, 117, 110, 115, 97, 102, 101]),
    ];

    for f in filters.iter() {
        socket.set_subscribe(format!("{}{}", "{\"msg_type\":\"", f.0).as_bytes()).unwrap();
        socket.set_subscribe(f.1).unwrap();
    }
    socket
}
