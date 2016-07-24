use super::*;

use msg_lib::*;
use msg_lib::dh_attestation::*;
use msg_lib::rust_crypto_dha::*;

// use super::lazy_static;
// use super::sgx_isa::Targetinfo;
use std::collections::HashMap;
use std::error::Error;
use std::time::{Instant};

pub static mut MSG_FORMAT: MsgFormat = MsgFormat::Protobuf;

const CAPACITY: usize = 1000;
const EXPIRATION: u64 = 21000; // milliseconds

// static ENCLAVE_ID: u32 = 123456789u32;

// static mut DHA: RustCryptoDHA = RustCryptoDHA{
//     state: DhaState::Responder(DhaResponderState::Start),
//     n: [0;32],
//     q: [0;32],
//     msg_format: MSG_FORMAT,
// };

// const MAX_CLIENTS: usize = 128;

// static mut clients: [(u32, RustCryptoDHA); MAX_CLIENTS] = [(0u32, RustCryptoDHA{
//     state: DhaState::Responder(DhaResponderState::Start),
//     n: [0;32],
//     q: [0;32],
//     msg_format: MSG_FORMAT,
// }); MAX_CLIENTS];


static mut INITIALIZED: bool = false;
static mut CLIENTS: *mut HashMap<u32, RustCryptoDHA>
                    = 0 as *mut HashMap<u32, RustCryptoDHA>;

static mut SUB_CACHE: *mut SubscriptionCache<Vec<u8>> = 0 as *mut SubscriptionCache<Vec<u8>>;

static mut BENCHMARK_REQUEST_CTR: Option<u32> = None;
static mut BENCHMARK_START_TIME: Option<Instant> = None;

const KEY: [u8;16] = [0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8,
                        8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8, 15u8];

pub fn ecall_handle_sub_msg(msg: Vec<u8>) {
    let msg_format;
    unsafe { if !INITIALIZED { initialize(); }
            msg_format = MSG_FORMAT; }

    let msg_decoded = decode_cache_msg(msg, msg_format);
    match msg_decoded {
        Err(err) => { warn!("{:?}", err.description()); } ,
        Ok(m) => {  //info!("Received request: {:?}", &m);
                    handle_sub_msg(m);
                    // info!("Response: {:?}", resp); }};
                },
        };
}

fn handle_sub_msg(cache_msg: CacheMsg) {

    // match cache_msg.msg_type.as_str() {
    //     "unclutch" | "invalid-voltage" | "speed-error" | "speed-unsafe" => {
    //         // dependent on mac, if no mac: cache ensures confidentiality
    //         // test MAC validation
    //         debug!("MAC valididation successful: {}", validate_cache_msg(&cache_msg, KEY));
    //         // let (valid, decrypted) = decrypt_cache_msg(&cache_msg, KEY);
    //         // debug!("MAC encryption valid: {}, result: {:?}", valid, decrypted );
    //     },
    //     "clamp15" => {
    //         // test MAC validation
    //         debug!("MAC valididation successful: {}", validate_cache_msg(&cache_msg, KEY));
    //         // let (valid, decrypted) = decrypt(&cache_msg, key);
    //         // debug!("MAC encryption valid: {}, result: {:?}", valid, decrypted );
    //     },
    //     _ => {
    //         // should not match, since the filters are defined precisely
    //         warn!("FAIL! ...unknown /published sensor message.");
    //     },
    // };

    // dependent on mac, if no mac: cache ensures confidentiality
    // test MAC validation
    trace!("MAC valididation successful: {}", validate_cache_msg(&cache_msg, KEY));
    // let (valid, decrypted) = decrypt_cache_msg(&cache_msg, KEY);
    // debug!("MAC encryption valid: {}, result: {:?}", valid, decrypted );


    let time = match cache_msg.time {
        Some(t) => t,
        None => get_time_in_millis(),

    };
    // put into cache, either plain value (authenticated with timestamp=version by cache, or authenticated/encrypted msg (as received))
    // problem: when message is protected by sensor, cache needs the key for authentication (which is okay, since clients need it too.)
    // if key is not availabe to cache, it needs to protect the msg itself.
    // so, sha or aes-gcm?

    // expecting the cache has the key.
    let mac = match cache_msg.mac {
        Some(m) => m,
        None => {
            // TODO authentication vs. encryption
            authenticate(cache_msg.msg_type.as_str(), time, &cache_msg.msg, &KEY)
        }
    };

    unsafe {
        (*SUB_CACHE).insert(cache_msg.msg_type.as_str(), time, cache_msg.msg, mac);
    }

}


pub fn ecall_handle_request(msg: Vec<u8>) -> Vec<Vec<u8>> {
    let msg_format;
    unsafe { if !INITIALIZED { initialize(); }
            msg_format = MSG_FORMAT; }

    let msg_decoded = decode_cache_msg(msg, msg_format);
    let result = match msg_decoded {
        Err(err) => {
            vec!(send_err_msg(err.description().to_string()))
        },
        Ok(m) => {  trace!("Received request: {:?}", &m);
                    let resp = handle_request(m);
                    trace!("Response: {:?}", resp);
                    resp }};
    result
}

unsafe fn initialize() {
    let clients = Box::new(HashMap::<u32, RustCryptoDHA>::new());
    CLIENTS = Box::into_raw(clients);

    let sub_cache = Box::new(SubscriptionCache::new(CAPACITY, EXPIRATION, KEY));
    SUB_CACHE = Box::into_raw(sub_cache);

    INITIALIZED = true;
}

fn handle_request(cache_msg: CacheMsg) -> Vec<Vec<u8>> {
    let msg_format;
    unsafe { msg_format = MSG_FORMAT; }
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
                if BENCHMARK_REQUEST_CTR.is_some() {
                    match BENCHMARK_REQUEST_CTR.as_mut() {
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
                    warn!("Error at decoding subscription request: {:?}", e.description());
                    return vec![] },
            };
            trace!("requested number, topics: {:?}, {:?}", number, subs);
            for topic in subs {
                match String::from_utf8(topic) {
                    Ok(t) => {
                        unsafe {
                            let topic_str = t.as_str();
                            let values = (*SUB_CACHE).get(topic_str, number);
                            // build msgs
                            let mut msgs: Vec<Vec<u8>> = values.into_iter().map(|(time, msg, mac)|
                                encode_all_given(msg, topic_str, Some(mac), time, msg_format).unwrap() ).collect();
                                trace!("{} values for topic {}", msgs.len(), t);
                            result.append(&mut msgs);
                        }
                    },
                    Err(e) => { warn!("Error at decoding a topic of a subscription request: {:?}", e.description()); },
                }
            }
            let response_size = result.len() as u32;
            result.insert(0, encode_u32_msg(response_size, "SUBACK", MsgPolicy::Plain, None, msg_format).unwrap());
            return result
        },
        "Start" => {
            unsafe {
                BENCHMARK_REQUEST_CTR = Some(0);
                BENCHMARK_START_TIME = Some(Instant::now())
            }
            warn!("Started counter for measuring requests/second.");
            return vec![encode_cache_msg(vec![], "OK", MsgPolicy::Plain, None, get_time_in_millis(), msg_format).unwrap()]
        },
        "Stop" => {
            let stop_time = Instant::now();
            let req_per_sec;
            unsafe {
                if BENCHMARK_REQUEST_CTR.is_none()
                    // || BENCHMARK_REQUEST_CTR.unwrap() == 0
                    || BENCHMARK_START_TIME.is_none() {
                    return vec![send_err_msg("Benchmark wasn't started.".to_string())]
                }
                let dur = stop_time.duration_since(BENCHMARK_START_TIME.unwrap());
                let dur_float = (dur.as_secs() as f64) + (dur.subsec_nanos() as f64 / 1000000000f64);
                req_per_sec = (BENCHMARK_REQUEST_CTR.unwrap() as f64) / dur_float;
                warn!("BENCHMARK_REQUEST_CTR: {}, duration: {:?} = {}", BENCHMARK_REQUEST_CTR.unwrap(), dur, dur_float);

            }
            warn!("Benchmark result: {} requests per second", req_per_sec as u32);
            unsafe {
                BENCHMARK_REQUEST_CTR = None;
                BENCHMARK_START_TIME = None;
            }
            return vec![encode_u32_msg(req_per_sec as u32, "Req/Sec", MsgPolicy::Authenticated, Some(KEY), msg_format).unwrap()]

        },
        _ => {},
    };

    // DHA attestation
    vec![match msg_type {
        "REQ" =>  {
            // 1st
            let mut dha = RustCryptoDHA::new(msg_format, DhaState::Responder(DhaResponderState::Start));
            dha.dha_init_session(DhaRole::Responder);
            dha.state = DhaState::Responder(DhaResponderState::Msg1Sent);
            let targetinfo;
            unsafe {
                targetinfo = get_targetinfo();
                (*CLIENTS).insert(cache_msg.client_id.unwrap(), dha);
                debug!("CLIENTS size: {:?}", (*CLIENTS).len());
            }
            match dha.dha_responder_gen_msg1(cache_msg.msg, targetinfo) {
                Ok(msg1) => { msg1 },
                Err(err) => send_err_msg(err.description().to_string()),
            }
        },
        "MS2" => {
            let client_id = match cache_msg.client_id {
                Some(id) => id,
                None => { return vec!(send_err_msg("Client_id is missing.".to_string())) }
            };
            unsafe {
                // check state
                // TODO initiate a new session request? cache has no valid session anymore

                match (*CLIENTS).get_mut(&client_id) {
                    None => { return vec!(send_err_msg("Not waiting for MS2.".to_string())) },
                    Some(dha) => {
                        if dha.state != DhaState::Responder(DhaResponderState::Msg1Sent) {
                            return vec!(send_err_msg("Not waiting for MS2.".to_string()))
                        } else {
                            let msg3 = match dha.dha_responder_proc_msg2(cache_msg.msg) {
                                Ok(ms3) => { ms3 },
                                Err(err) => send_err_msg(err.description().to_string()),
                            };
                            dha.state = DhaState::Responder(DhaResponderState::Active);
                            debug!("DHA = {:?}", dha);
                            msg3
                        }}}}
        },
        // "ERR" => {  },   // TODO
        "SUB" => unreachable!(),
        _ => send_err_msg("Unknown message type.".to_string()),
    }]

}

unsafe fn get_targetinfo() -> Vec<u8> {   //Targetinfo {
    let mut raw_dummy: [u8; 512] = [0; 512];
    for i in 0..511 {
        raw_dummy[i] = i as u8;
    }
    slice_to_vec(&raw_dummy)
}
