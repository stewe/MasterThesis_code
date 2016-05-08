#[macro_use]
extern crate log;
extern crate simple_logger;
extern crate time;
extern crate zmq;
extern crate zmq_sys;

pub mod cache_ds; // TODO pub necessary???

use cache_ds::*;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use time::{Timespec,};
use zmq::{Message};


pub struct CacheEnclave {
    expiration: i64,
    #[allow(dead_code)] // TODO
    cache_ds: CacheDS<String, String>, // mutex? when using an dedicated thread for garbage collection
}

impl CacheEnclave {
    pub fn new(capacity: usize, expiration: i64) -> CacheEnclave {
        CacheEnclave {
            expiration: expiration,
            cache_ds: CacheDS::new(capacity),
        }
    }

    // fn set_expiration(&mut self, expiration: i64) {
    //     self.expiration = expiration;
    // }

    pub fn get_expiration(&self) -> i64 {
        self.expiration
    }

    pub fn handle_request(&mut self, msg: &mut Message) -> Result<Message, zmq::Error> {
        let msg_str = msg.as_str().unwrap();
        let mut request = msg_str.split(' ');
        let op = request.next().unwrap_or("No operator!");

        let mut resp = String::new();
        let resp_msg = Message::new();
        match op {
            // set
            // add
            //replace

            //get

            //delete

            // next case
            _ => { info!("Received unknown request."); }
        }
        resp_msg
    }

    // pub fn set_with_expiry(&mut self, key: K, value: V, expiry: Timespec) {
    //     self.cache_ds
    // }

    // set
    // add
    //replace

    //get

    //delete


}

// static mut CACHE_ENCLAVE: CacheEnclave = CacheEnclave{expiration: 0, cache_ds: None};

fn print_tuple(id: String, t: (String, Timespec)) {
    let (msg, time) = t;
    print!("id: {}; msg: {}, time: {}.{}", id, msg, time.sec, time.nsec);
}

pub fn print_arc_queues(queues: &HashMap<String, Arc<Mutex<VecDeque<(String, Timespec)>>>>) {
    for (id, q) in queues.iter() {
        let q = q.lock().unwrap();
        let size = q.len();
        println!("{} contains: ", id);
        for i in 0..size {
            let a = q.get(i).unwrap();
            print_tuple(id.clone(), a.clone());
            println!("");
        }
        println!("");
    }
}
