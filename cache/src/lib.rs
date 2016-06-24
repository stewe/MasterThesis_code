#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate simple_logger;
extern crate time;
extern crate zmq;
extern crate zmq_sys;
extern crate msg_lib;
extern crate sgx_isa;

pub mod cache_enclave;

pub mod cache_ds; // TODO pub necessary???

use cache_ds::*;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use time::{Duration, now, Timespec};
use zmq::{Message};


static CAPACITY: usize = 10;  // TODO configure
static EXPIRATION: i64 = 6000; // TODO configure

// static mut CACHE_ENCLAVE: CacheEnclave = CacheEnclave{expiration: 0, cache_ds: None};

pub struct CacheEnclave {
    expiration: i64,
    #[allow(dead_code)] // TODO
    cache_ds: CacheDS<String, Vec<u8>>, // mutex? when using an dedicated thread for garbage collection
}

impl CacheEnclave {
    pub fn new(capacity: usize, expiration: i64) -> CacheEnclave {
        CacheEnclave {
            expiration: expiration,
            cache_ds: CacheDS::new(capacity),
        }
    }

    pub fn new_default() -> CacheEnclave {
        CacheEnclave {
            expiration: EXPIRATION,
            cache_ds: CacheDS::new(CAPACITY),
        }
    }

    // fn set_expiration(&mut self, expiration: i64) {
        // self.expiration = expiration;
    // }

    pub fn get_expiration(&self) -> i64 {
        self.expiration
    }

    pub fn handle_request(&mut self, msg: &Vec<u8>) -> String {
        // TODO debug only
        match String::from_utf8(msg.clone()) {
            Ok(m) => { debug!("Received request: {:?}", m); },
            Err(_) => { debug!("Received non-string request: {:?}", &msg); },
        }



        let msg_str = "msg.as_str().unwrap()"; // TODO

        // TODO parse msg appropriately
        let mut request = msg_str.split(' ');
        let op = request.next().unwrap_or("No operator!");

        let mut resp = String::new();
        match op {



            "set" => {  //let time_in_millis: i64 = 1462786673725;
                        //let expiry = Timespec::new(time_in_millis/1000, (time_in_millis%1000 * 1000) as i32);
                        let expiry = now() + Duration::seconds(10); // use parse and calculated value

                        let result: Result<(), &str>;
                        // result = self.cache_ds.insert_with_expiry(key, value, expiry);
                        // result = self.cache_ds.insert_with_ttl(key, value, ttl: i64);
                        // no ttl/expiry given: use default
                        // result = self.cache_ds.insert_with_ttl(key, value, self.expiration);
                        result = Err("foo bar!");

                        match result {
                            Ok(_) => { resp.push_str(""); },
                            Err(err) => { resp.push_str(err); },
                        };
                    },
            // add
            //replace

            "get" => {  let key = "key".to_string();
                        match self.cache_ds.get(&key) {
                            Some(v) => { match std::str::from_utf8(v) {
                                            Ok(val) => { resp = resp + val; },
                                            Err(err) => { error!("Error at {}", err);
                                                            resp = resp + "No valid value."; }
                                        }},
                            None => { resp = resp + "No valid value."; },
                        }
                    },

            "del" => {  let key = "key".to_string();
                        self.cache_ds.remove(&key);
                        resp.push_str("OK");
            }

            // next case
            _ => { info!("Received unknown request."); }
        }
        resp
    }


}


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
