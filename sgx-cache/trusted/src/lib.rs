#![feature(asm,collections,const_fn)]
#![no_std]

#![feature(iter_arith)]

#[macro_use] extern crate collections;
extern crate enclave;
extern crate spin;
extern crate interface;
extern crate core_collections;
extern crate core_crypto;
extern crate core_rustc_serialize;
extern crate core_protobuf;
#[macro_use] extern crate lazy_static;

mod msg_lib;
mod sub_cache;

use msg_lib::{authenticate, decode_cache_msg, decode_sub_cache_msg, encode_all_given, encode_cache_msg, encode_u32_msg, send_err_msg, CacheMsg, MsgFormat, MsgPolicy};
use sub_cache::*;
use collections::{String, Vec};
use collections::string::ToString;
use core::mem::transmute_copy;
use core_collections::HashMap;
use enclave::usercall::{do_usercall, UserBox, UserSlice};
use interface::ECall;
use spin::Mutex;

const CAPACITY: usize = 1000;
const EXPIRATION: u64 = 21000; // milliseconds
const KEY: [u8;16] = [0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8,
                        8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8, 15u8];
#[cfg(feature = "json")]
pub const MSG_FORMAT: MsgFormat = MsgFormat::Json;
#[cfg(not(feature = "json"))]
pub const MSG_FORMAT: MsgFormat = MsgFormat::Protobuf;

lazy_static!{
    static ref SUB_CACHE: Mutex<SubscriptionCache> = Mutex::new(SubscriptionCache::new(CAPACITY, EXPIRATION, KEY));
    static ref BENCHMARK_REQUEST_CTR: Mutex<Option<u32>> = Mutex::new(None);
    static ref BENCHMARK_START_TIME: Mutex<Option<u64>> = Mutex::new(None);
    static ref RESPONSE_MSGS: Mutex<Vec<UserSlice<u8>>> = Mutex::new(vec![]);
} 
static LAST_KNOWN_TIME: Mutex<u64> = Mutex::new(0u64);
pub static USER_HEAP_INITIATED: Mutex<bool> = Mutex::new(false);

#[no_mangle]
pub extern "C" fn entry(ecall: u64, p1: u64, p2: u64, _ignore:u64, p3: u64, time: u64) -> u64 {

    set_time(time);

    match ECall::from_u64(ecall).unwrap() {
        ECall::InitUserHeap => { unsafe{do_usercall(ecall, p1, p2, p3, time)}; init_user_heap(p1, p2); return 0 },
        ECall::HandleRequest => { let msg: Vec<u8> = unsafe{ (*(p1 as *const Vec<u8>)).clone() };
                                let response = ecall_handle_request(msg);
                                let mut response_msgs = RESPONSE_MSGS.lock();
                                response_msgs.clear();
                                for mut resp in response {
                                    let m_len = resp.len() as u16;
                                    let m_len_bytes: [u8;2] = unsafe{ transmute_copy(&m_len) };
//                                    unsafe{ do_usercall(ecall, 0, 0, 0, m_len as u64); }
                                    // insert the msg length at the first place
                                    resp.insert(0,m_len_bytes[1]);
                                    resp.insert(0,m_len_bytes[0]);
                                    (*response_msgs).push(UserSlice::clone_from(&resp));
                                }

                                let mut output_msgs: Vec<u64> = unsafe{ response_msgs.iter().map(|ref slice| slice.as_ptr() as u64).rev().collect() };
                                let out_len = output_msgs.len() as u64;
                                output_msgs.insert(0, out_len);
                                // Somehow the the first value becomes overwritten with '0'. Thus a placeholder is inserted.
                                output_msgs.insert(0, 0);  
                                let out_slice = UserSlice::clone_from(&output_msgs);
                                return unsafe{ out_slice.as_ptr() as u64 }
                               },
        ECall::HandleSubMsg => { let msg: Vec<u8> = unsafe{ (*(p1 as *const Vec<u8>)).clone() };
                                ecall_handle_sub_msg(msg);
                                // DEBUG stuff
//                                {
//                                   let cache = SUB_CACHE.lock();
//                                   let v = cache.get_size_per_entry();
//                                    let v_len = v.len();
//                                   let sum: u64 = v.into_iter().map(|(_, len)| { len as u64 }).sum();
//                                   unsafe{ do_usercall(ecall, 0, 0, v_len as u64, sum); };
//                                }
                                return 0
                                },
}

}


fn init_user_heap(heap_base: u64, heap_size: u64) {
    if !(*USER_HEAP_INITIATED.lock()) {
        use enclave::usercall::init_user_heap;
        enclave::usercall::init_user_heap(heap_base as *mut u8, heap_size as usize);
    }
}

fn ecall_handle_sub_msg(msg: Vec<u8>) {
    let msg_format;
//    unsafe { if !INITIALIZED { initialize(); }
    msg_format = MSG_FORMAT;
// }

    let msg_decoded = decode_cache_msg(msg, msg_format);
    match msg_decoded {
        Err(err) => {}, //{ warn!("{:?}", err.description()); } ,
        Ok(m) => {  //info!("Received request: {:?}", &m);
                    handle_sub_msg(m);
                    // info!("Response: {:?}", resp); }};
                },
        };
}


fn ecall_handle_request(msg: Vec<u8>) -> Vec<Vec<u8>> {
    let msg_format;
//    unsafe { if !INITIALIZED { initialize(); }
     msg_format = MSG_FORMAT;
// }

    let msg_decoded = decode_cache_msg(msg, msg_format);
    let result = match msg_decoded {
        Err(err) => {
            vec!(send_err_msg(err.description().to_string()))
        },
        Ok(m) => {  //trace!("Received request: {:?}", &m);
                    let resp = handle_request(m);
                    //trace!("Response: {:?}", resp);
                    resp }};
    result
}

fn handle_sub_msg(cache_msg: CacheMsg) {
    let time = match cache_msg.time {
        Some(t) => t,
        None => get_time_in_millis(),

    };
    // put into cache, either plain value (authenticated with timestamp=version by cache, or authenticated/encrypted msg (as received))
    // problem: when message is protected by sensor, cache needs the key for authentication (which is okay, since clients need it too.)
    // if key is not availabe to cache, it needs to protect the msg itself.
    // expecting the cache has the key.
    let mac = match cache_msg.mac {
        Some(m) => m,
        None => {
            // TODO authentication vs. encryption
            authenticate(cache_msg.msg_type.as_str(), time, &cache_msg.msg, &KEY)
        }
    };
    unsafe {
        let mut sub_cache = SUB_CACHE.lock();
        (*sub_cache).insert(cache_msg.msg_type.as_str(), time, cache_msg.msg, mac);
    }
}

fn handle_request(cache_msg: CacheMsg) -> Vec<Vec<u8>> {
    let msg_format = MSG_FORMAT;
    if cache_msg.mac.is_some() {
        // TODO
        // get SMK for cache_msg.enclave_id
        // verify, decrypt
    }

    let msg_type = cache_msg.msg_type.as_str();

    match msg_type {
        "SUB" => {
            // TODO if DHA, check validity of request
            unsafe {
                let mut ctr = BENCHMARK_REQUEST_CTR.lock();
                if ctr.is_some() {
                    match ctr.as_mut() {
                        Some(v) => *v = *v + 1,
                        None => {},
                    }
                }
            }

            let mut result = vec!();
            // TODO IMPORTANT SubCacheMsg
            let (number, subs) = match decode_sub_cache_msg(cache_msg.msg, msg_format) {
            // let subs = match decode_bytes_vec_msg(cache_msg.msg, msg_format) {
                Ok(v) => { (match v.0 { Some(n) => Some(n as usize), None => None }, v.1) },
                Err(e) => {
//                    warn!("Error at decoding subscription request: {:?}", e.description());
                    return vec![] },
            };
//            trace!("requested number, topics: {:?}, {:?}", number, subs);
            for topic in subs {
                match String::from_utf8(topic) {
                    Ok(t) => {
                        unsafe {
                            let topic_str = t.as_str();
                            let mut sub_cache = SUB_CACHE.lock();
                            let values = sub_cache.get(topic_str, number);
                            // build msgs
                            let mut msgs: Vec<Vec<u8>> = values.into_iter().map(|(time, msg, mac)|
                                encode_all_given(msg, topic_str, Some(mac), time, msg_format).unwrap() ).collect();
//                                trace!("{} values for topic {}", msgs.len(), t);
                            result.append(&mut msgs);
                        }
                    },
                    Err(e) => { }, //warn!("Error at decoding a topic of a subscription request: {:?}", e.description()); },
                }
            }
            let response_size = result.len() as u32;
            result.insert(0, encode_u32_msg(response_size, "SUBACK", MsgPolicy::Plain, None, msg_format).unwrap());
            return result
        },
        "Start" => {
            unsafe {
                *(BENCHMARK_REQUEST_CTR.lock()) = Some(0);
                *(BENCHMARK_START_TIME.lock()) = Some(get_time_in_millis())
            }
//            warn!("Started counter for measuring requests/second.");
            return vec![encode_cache_msg(vec![], "OK", MsgPolicy::Plain, None, get_time_in_millis(), msg_format).unwrap()]
        },
        "Stop" => {
            let stop_time = get_time_in_millis();
            let req_per_sec;
            let mut ctr = BENCHMARK_REQUEST_CTR.lock();
            let mut start_time = BENCHMARK_START_TIME.lock();
            unsafe {
                if ctr.is_none()
                    || start_time.is_none() {
                    return vec![send_err_msg("Benchmark wasn't started.".to_string())]
                }
                let dur = stop_time - start_time.unwrap();
                let dur_secs: f64 = dur as f64 / 1000f64;
                req_per_sec = (ctr.unwrap() as f64) / dur_secs;
//                warn!("BENCHMARK_REQUEST_CTR: {}, duration: {:?} = {}", BENCHMARK_REQUEST_CTR.unwrap(), dur, dur_float);

            }
//            warn!("Benchmark result: {} requests per second", req_per_sec as u32);
            unsafe {
                *ctr = None;
                *start_time = None;
            }
            return vec![encode_u32_msg(req_per_sec as u32, "Req/Sec", MsgPolicy::Authenticated, Some(KEY), msg_format).unwrap()]

        },
        _ => {},
    };

    vec![]

}

fn handle_sub_msg_test(msg: Vec<u8>, time: u64) -> u64 {
    use core::iter::IntoIterator;
    let mut sum: u8 = 0;
    
    for num in msg {
        unsafe{do_usercall(111, sum as u64, num as u64, 0, 0)};    
//        sum = sum + num; // TODO why doesn't this work???
    }
    unsafe{do_usercall(111, sum as u64, 0, 0, time)};

    sum as u64
    //0
}

fn handle_test() -> u64 {
    use msg_lib::{authenticate, decrypt, encrypt, validate, SubCacheMsg};

    
        1337

//    let output = UserSlice::clone_from(&encrypted);
//    let output_ptr = unsafe{ output.as_ptr() };
//    let res = unsafe{do_usercall(42, 42, short_mac, len, output_ptr as u64)};
}



fn set_time(new_time: u64) {
    let mut old_time = LAST_KNOWN_TIME.lock();
    if new_time > *old_time {
        *old_time = new_time;
    }
}

/// returns the time that was passed to the enclave at the last ECall.
pub fn get_time_in_millis() -> u64 {
    *LAST_KNOWN_TIME.lock()
}




extern {
    static ENCLAVE_SIZE: usize;
}

#[inline(always)]
fn image_base() -> u64 {
    let base;
    unsafe{asm!("lea IMAGE_BASE(%rip),$0":"=r"(base))};
    base
}

#[allow(dead_code)]
fn is_enclave_range<T>(p: *const T, len: usize) -> bool {
    let start=p as u64;
    let end=start+(len as u64);
    start >= image_base() && end <= image_base()+(ENCLAVE_SIZE as u64)
}

fn is_user_range<T>(p: *const T, len: usize) -> bool {
    let start=p as u64;
    let end=start+(len as u64);
    end <= image_base() || start >= image_base()+(ENCLAVE_SIZE as u64)
}
