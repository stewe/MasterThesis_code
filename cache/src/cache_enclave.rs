use super::*;

use msg_lib::*;
use msg_lib::dh_attestation::*;
use msg_lib::rust_crypto_dha::*;
//{CacheMsg, from_json_to_cache_msg, from_protobuf_to_cache_msg, MsgFormat, RustCryptoDHA};

// use super::lazy_static;
// use super::sgx_isa::Targetinfo;
use std::collections::HashMap;
use std::error::Error;

// configure which format to use
const MSG_FORMAT: MsgFormat = MsgFormat::Json;
// const MSG_FORMAT: MsgFormat = MsgFormat::Protobuf;

const CAPACITY: usize = 10000;
const EXPIRATION: i64 = 10000;

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

// use std::mem;
// info!("size of (u32, RustCryptoDHA): {}\nsize of clients: {}",
//     mem::size_of::<(u32, RustCryptoDHA)>(),
//     mem::size_of::<[(u32, RustCryptoDHA); MAX_CLIENTS]>());

static mut INITIALIZED: bool = false;
static mut CLIENTS: *mut HashMap<u32, RustCryptoDHA>
                    = 0 as *mut HashMap<u32, RustCryptoDHA>;

static mut SUB_CACHE: *mut SubscriptionCache = 0 as *mut SubscriptionCache;


pub fn ecall_handle_request(msg: Vec<u8>) -> Vec<u8> {

    unsafe { if !INITIALIZED { initialize(); } }

    let msg_decoded = decode_cache_msg(msg, MSG_FORMAT);
    let result = match msg_decoded {
        Err(err) => send_err_msg(err.description().to_string()),
        Ok(m) => {  //info!("Received request: {:?}", &m);
                    let resp = handle_request(m);
                    // info!("Response: {:?}", resp);
                    resp }};
    result
}

unsafe fn initialize() {
    let clients = Box::new(HashMap::<u32, RustCryptoDHA>::new());
    CLIENTS = Box::into_raw(clients);

    let sub_cache = Box::new(SubscriptionCache::new(CAPACITY, EXPIRATION));
    SUB_CACHE = Box::into_raw(sub_cache);

    INITIALIZED = true;
}

fn handle_request(cache_msg: CacheMsg) -> Vec<u8> {
    info!("Received request: {:?}", &cache_msg);
    if cache_msg.mac.is_some() {
        // TODO
        // get SMK for cache_msg.enclave_id
        // verify, decrypt
    }

    // decode match MSG_FORMAT...
    if cache_msg.msg_type.starts_with("pub/") {

        let mut key  = [0u8;16];
        key = (0..16).into_iter().fold(key, |mut acc, x| { acc[x] = x as u8; acc });
        info!("key: {:?}", key);

        // put match into a lib function or write
        let msg_type = cache_msg.msg_type.clone();
        let topic = msg_type.split_at(4).1;
        match topic {
            "unclutch" | "voltage" | "speed-error" | "speed-unsafe" => {
                info!("SCHUBIDU!");
                // dependent on mac, if no mac: cache ensures confidentiality
                // test MAC validation
                // info!("MAC valididation successful: {}", validate(&cache_msg, key));
                let (valid, decrypted) = decrypt_cache_msg(&cache_msg, key);
                info!("MAC encryption valid: {}, result: {:?}", valid, decrypted );
            },
            "clamp15" => {
                // test MAC validation
                info!("MAC valididation successful: {}", validate_cache_msg(&cache_msg, key));
                // let (valid, decrypted) = decrypt(&cache_msg, key);
                // info!("MAC encryption valid: {}, result: {:?}", valid, decrypted );
            },
            _ => {
                // should not match, since the filters are defined precisely
                warn!("FAIL! ...unknown /published sensor message.");
                return vec!()
            },
        };

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
                authenticate(cache_msg.msg_type.as_str(), time, &cache_msg.msg, &key)
            }
        };

        unsafe {
            (*SUB_CACHE).insert(msg_type.as_str(), time, cache_msg.msg, mac);
        }

        return vec!()
    }

    match cache_msg.msg_type.as_str() {
        "REQ" =>  {
            // 1st
            let mut dha = RustCryptoDHA::new(MSG_FORMAT, DhaState::Responder(DhaResponderState::Start));
            dha.dha_init_session(DhaRole::Responder);
            dha.state = DhaState::Responder(DhaResponderState::Msg1Sent);
            let targetinfo;
            unsafe {
                targetinfo = get_targetinfo();
                (*CLIENTS).insert(cache_msg.client_id.unwrap(), dha);
                info!("CLIENTS size: {:?}", (*CLIENTS).len());
            }
            match dha.dha_responder_gen_msg1(cache_msg.msg, targetinfo) {
                Ok(msg1) => { msg1 },
                Err(err) => send_err_msg(err.description().to_string()),
            }
        },
        "MS2" => {
            let client_id = match cache_msg.client_id {
                Some(id) => id,
                None => { return send_err_msg("Client_id is missing.".to_string()) }
            };
            unsafe {
                // check state
                // TODO initiate a new session request? cache has no valid session anymore

                match (*CLIENTS).get_mut(&client_id) {
                    None => { return send_err_msg("Not waiting for MS2.".to_string()) },
                    Some(dha) => {
                        if dha.state != DhaState::Responder(DhaResponderState::Msg1Sent) {
                            return send_err_msg("Not waiting for MS2.".to_string())
                        } else {
                            let msg3 = match dha.dha_responder_proc_msg2(cache_msg.msg) {
                                Ok(ms3) => { ms3 },
                                Err(err) => send_err_msg(err.description().to_string()),
                            };
                            dha.state = DhaState::Responder(DhaResponderState::Active);
                            info!("DHA = {:?}", dha);
                            msg3
                        }}}}
        },
        // "ERR" => {  },   // TODO
        _ => send_err_msg("Unknown message type.".to_string()),
    }

}

unsafe fn get_targetinfo() -> Vec<u8> {   //Targetinfo {
    let mut raw_dummy: [u8; 512] = [0; 512];
    for i in 0..511 {
        raw_dummy[i] = i as u8;
    }
    slice_to_vec(&raw_dummy)
}
