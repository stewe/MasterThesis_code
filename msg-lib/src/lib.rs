#[macro_use] extern crate log;
extern crate simple_logger;
extern crate crypto;
extern crate protobuf;

extern crate rand;
extern crate rustc_serialize;

mod msg_json;
mod msg_proto;
mod msg_proto_defs;
pub mod dh_attestation;
pub mod rust_crypto_dha;

use crypto::aes::KeySize;
use crypto::aes_gcm::*;
use crypto::aead::{AeadEncryptor, AeadDecryptor};
use crypto::curve25519::curve25519;

use std::error::Error;
use std::fmt;
use std::iter::repeat;
use std::time::{SystemTime, UNIX_EPOCH};
use dh_attestation::*;

#[derive(Clone)]
pub enum MsgPolicy {
    Plain,
    Authenticated,
    Encrypted
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MsgFormat {
    Json,
    Protobuf
}

#[derive (Debug)]
pub struct DecodeError {
    description: String,
}


impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Decode error: {}", self.description)
    }
}

impl Error for DecodeError {
    fn description(&self) -> &str {
        &self.description
    }
}

#[derive (Debug)]
pub struct EncodeError {
    description: String,
}

impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Decode error: {}", self.description)
    }
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct CacheMsg {
    pub msg_type: String,
    pub msg: Vec<u8>,
    pub client_id: Option<u32>,
    pub mac: Option<Vec<u8>>,
    pub time: Option<u64>,
}


#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct ErrorMsg {
    pub description: String,
}

pub fn encode_cache_msg(msg: Vec<u8>, topic: &str, policy: MsgPolicy, key: Option<[u8;16]>,
                        time: u64, msg_format: MsgFormat)
-> Result<Vec<u8>, EncodeError> {
    let p = (topic, msg, None, policy, key, Some(time));
    match msg_format {
        MsgFormat::Json => msg_json::to_json_from_bytes_msg(p.0, p.1, p.2, p.3, p.4, p.5),
        MsgFormat::Protobuf => msg_proto::to_proto_from_bytes_msg(p.0, p.1, p.2, p.3, p.4, p.5),
    }
}

pub fn encode_all_given(msg: Vec<u8>, msg_type: &str, mac: Option<Vec<u8>>, time: u64,
                        msg_format: MsgFormat)
-> Result<Vec<u8>, EncodeError> {
    let p = (msg, msg_type, mac, time);
    match msg_format {
        MsgFormat::Json => msg_json::to_json_all_given(p.0, p.1, p.2, p.3),
        MsgFormat::Protobuf => msg_proto::to_proto_all_given(p.0, p.1, p.2, p.3),
    }
}

pub fn decode_cache_msg(msg: Vec<u8>, format: MsgFormat) -> Result<CacheMsg, DecodeError> {
    match format {
        MsgFormat::Json => { msg_json::to_msg(msg) },
        MsgFormat::Protobuf => { msg_proto::to_cache_msg(msg_proto::to_msg(msg)) }
    }
}


#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct BoolMsg {
    pub val: bool,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct U8Msg {
    pub val: u8,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct BytesMsg {
    pub val: Vec<u8>,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct BytesVecMsg {
    pub val: Vec<Vec<u8>>,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct U32Msg {
    pub val: u32,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct SubCacheMsg {
    pub number: Option<u32>,
    pub filters: Vec<Vec<u8>>,
}



pub fn encode_bool_msg(val: bool, topic: &str, policy: MsgPolicy,
                        key: Option<[u8;16]>, msg_format: MsgFormat)
-> Result<Vec<u8>, EncodeError> {
    let time = get_time_in_millis();
    let p = (val, topic, policy, key, Some(time));
    match msg_format {
        MsgFormat::Json => msg_json::bool_msg(p.0, p.1, p.2, p.3, p.4),
        MsgFormat::Protobuf => msg_proto::bool_msg(p.0, p.1, p.2, p.3, p.4),
    }
}

pub fn encode_u8_msg(val: u8, topic: &str, policy: MsgPolicy,
                    key: Option<[u8;16]>, msg_format: MsgFormat)
-> Result<Vec<u8>, EncodeError> {
    let time = get_time_in_millis();
    let p = (val, topic, policy, key, Some(time));
    match msg_format {
        MsgFormat::Json => msg_json::u8_msg(p.0, p.1, p.2, p.3, p.4),
        MsgFormat::Protobuf => msg_proto::u8_msg(p.0, p.1, p.2, p.3, p.4),
    }
}

pub fn encode_u32_msg(val: u32, topic: &str, policy: MsgPolicy,
                        key: Option<[u8;16]>, msg_format: MsgFormat)
-> Result<Vec<u8>, EncodeError> {
    let time = get_time_in_millis();
    let p = (val, topic, policy, key, Some(time));
    match msg_format {
        MsgFormat::Json => msg_json::u32_msg(p.0, p.1, p.2, p.3, p.4),
        MsgFormat::Protobuf => msg_proto::u32_msg(p.0, p.1, p.2, p.3, p.4),
    }
}

pub fn encode_bytes_msg(val: Vec<u8>, topic: &str, policy: MsgPolicy,
                        key: Option<[u8;16]>, msg_format: MsgFormat)
-> Result<Vec<u8>, EncodeError> {
    let time = get_time_in_millis();
    let p = (val, topic, policy, key, Some(time));
    match msg_format {
        MsgFormat::Json => msg_json::bytes_msg(p.0, p.1, p.2, p.3, p.4),
        MsgFormat::Protobuf => msg_proto::bytes_msg(p.0, p.1, p.2, p.3, p.4),
    }
}

pub fn encode_bytes_vec_msg(val: Vec<Vec<u8>>, topic: &str, policy: MsgPolicy,
                            key: Option<[u8;16]>, msg_format: MsgFormat)
-> Result<Vec<u8>, EncodeError> {
    let time = get_time_in_millis();
    let p = (val, topic, policy, key, Some(time));
    match msg_format {
        MsgFormat::Json => msg_json::bytes_vec_msg(p.0, p.1, p.2, p.3, p.4),
        MsgFormat::Protobuf => msg_proto::bytes_vec_msg(p.0, p.1, p.2, p.3, p.4),
    }
}

pub fn encode_sub_cache_msg(number: Option<u32>, filters: Vec<Vec<u8>>, topic: &str,
                            policy: MsgPolicy, key: Option<[u8;16]>, msg_format: MsgFormat)
-> Result<Vec<u8>, EncodeError> {
    let time = get_time_in_millis();
    let p = (number, filters, topic, policy, key, Some(time));
    match msg_format {
        MsgFormat::Json => msg_json::sub_cache_msg(p.0, p.1, p.2, p.3, p.4, p.5),
        MsgFormat::Protobuf => msg_proto::sub_cache_msg(p.0, p.1, p.2, p.3, p.4, p.5),
    }
}

pub fn authenticate(msg_type: &str, time: u64, msg: &Vec<u8>, key: &[u8]) -> Vec<u8> {
    let nonce = produce_nonce(time, msg_type);

    //  aad: includes msg_type, msg and timestamp
    let mut aad: Vec<u8> = vec!();
    aad.extend_from_slice(msg_type.as_bytes());
    aad.extend_from_slice(&msg);
    aad.extend_from_slice(&nonce[0..8]); // = time
    let mut cipher = AesGcm::new(KeySize::KeySize128, key, &nonce, &aad);
    let mac = &mut [0;16];
    cipher.encrypt(&[], &mut [], mac);
    slice_to_vec(mac)
}

pub fn encrypt(msg_type: &str, time: u64, msg: &Vec<u8>, key: &[u8]) -> (Vec<u8>, [u8;16]) {
    let nonce = produce_nonce(time, msg_type);
    //  aad: includes msg_type and timestamp
    let mut aad: Vec<u8> = vec!();
    aad.extend_from_slice(msg_type.as_bytes());
    aad.extend_from_slice(&nonce[0..8]); // = time
    let mut cipher = AesGcm::new(KeySize::KeySize128, &key, &nonce, &aad);
    let mac = &mut [0;16];
    let mut output: Vec<u8> = repeat(0).take(msg.len()).collect();
    cipher.encrypt(&msg, output.as_mut_slice(), mac);

    (output, *mac)
}


pub fn validate_cache_msg(cache_msg: &CacheMsg, key: [u8;16]) -> bool {
    if cache_msg.mac.is_none() || cache_msg.time.is_none() { return false }
    validate(&cache_msg.mac.clone().unwrap(), cache_msg.time.unwrap(),
                &cache_msg.msg_type, &cache_msg.msg, key)
}

pub fn validate(mac: &Vec<u8>, time: u64, msg_type: &String, msg: &Vec<u8>, key: [u8;16]) -> bool {
    let nonce = produce_nonce(time, msg_type.as_str());
    //  aad: includes msg_type, msg and timestamp
    let mut aad: Vec<u8> = vec!();
    aad.extend_from_slice(msg_type.as_bytes());
    // only for validation
    aad.extend_from_slice(&msg);
    aad.extend_from_slice(&nonce[0..8]); // = time
    let mut cipher = AesGcm::new(KeySize::KeySize128, &key, &nonce, &aad);
    cipher.decrypt(&[], &mut [], &mac)
}

pub fn decrypt_cache_msg(cache_msg: &CacheMsg, key: [u8;16]) -> (bool, Vec<u8>) {
    if cache_msg.mac.is_none() || cache_msg.time.is_none() { return (false, vec!()) }
    decrypt(&cache_msg.mac.clone().unwrap(), cache_msg.time.unwrap(),
            &cache_msg.msg_type, &cache_msg.msg, key)
}

pub fn decrypt(mac: &Vec<u8>, time: u64, msg_type: &String, msg: &Vec<u8>, key: [u8;16])
-> (bool, Vec<u8>) {
    let nonce = produce_nonce(time, msg_type.as_str());
    //  aad: includes msg_type and timestamp
    let mut aad: Vec<u8> = vec!();
    aad.extend_from_slice(msg_type.as_bytes());
    aad.extend_from_slice(&nonce[0..8]); // = time
    let mut cipher = AesGcm::new(KeySize::KeySize128, &key, &nonce, &aad);
    let mut output: Vec<u8> = repeat(0).take(msg.len()).collect();
    (cipher.decrypt(&msg, output.as_mut_slice(), &mac), output)
}

pub fn decode_bytes_vec_msg(msg: Vec<u8>, msg_format: MsgFormat)
-> Result<Vec<Vec<u8>>, DecodeError> {
    let bytes_vec_decoded: Result<BytesVecMsg, DecodeError> = match msg_format {
        MsgFormat::Json => { msg_json::to_msg(msg) },
        MsgFormat::Protobuf => {
            msg_proto::to_bytes_vec_msg(msg_proto::to_msg::<msg_proto_defs::BytesVecMsg>(msg)) },
    };
    match bytes_vec_decoded {
        Ok(v) => Ok(v.val),
        Err(e) => Err(e)
    }
}

pub fn decode_bytes_msg(msg: Vec<u8>, msg_format: MsgFormat)
-> Result<Vec<u8> ,DecodeError> {
    let bytes_decoded: Result<BytesMsg, DecodeError> = match msg_format {
        MsgFormat::Json => { msg_json::to_msg(msg) },
        MsgFormat::Protobuf => {
            msg_proto::to_bytes_msg(msg_proto::to_msg::<msg_proto_defs::BytesMsg>(msg)) },
    };
    match bytes_decoded {
        Ok(v) => Ok(v.val),
        Err(e) => Err(e)
    }
}

pub fn decode_sub_cache_msg(msg: Vec<u8>, msg_format: MsgFormat)
-> Result<(Option<u32>, Vec<Vec<u8>>), DecodeError> {
    let sub_cache_msg_decoded: Result<SubCacheMsg, DecodeError> = match msg_format {
        MsgFormat::Json => { msg_json::to_msg(msg) },
        MsgFormat::Protobuf => {
            msg_proto::to_sub_cache_msg(msg_proto::to_msg::<msg_proto_defs::SubCacheMsg>(msg)) },
    };
    match sub_cache_msg_decoded {
        Ok(v) => Ok((v.number, v.filters)),
        Err(e) => Err(e)
    }
}

pub fn decode_bool_msg(msg: Vec<u8>, msg_format: MsgFormat) -> Result<bool, DecodeError> {
    let bool_decoded: Result<BoolMsg, DecodeError> = match msg_format {
        MsgFormat::Json => { msg_json::to_msg(msg) },
        MsgFormat::Protobuf => {
            msg_proto::to_bool_msg(msg_proto::to_msg::<msg_proto_defs::BoolMsg>(msg)) },
    };
    match bool_decoded {
        Ok(v) => Ok(v.val),
        Err(e) => Err(e)
    }
}

pub fn decode_u8_msg(msg: Vec<u8>, msg_format: MsgFormat) -> Result<u8, DecodeError> {
    let u8_decoded: Result<U8Msg, DecodeError> = match msg_format {
        MsgFormat::Json => { msg_json::to_msg(msg) },
        MsgFormat::Protobuf => {
            msg_proto::to_u8_msg(msg_proto::to_msg::<msg_proto_defs::U8Msg>(msg)) },
    };
    match u8_decoded {
        Ok(v) => Ok(v.val),
        Err(e) => Err(e)
    }
}

pub fn decode_u32_msg(msg: Vec<u8>, msg_format: MsgFormat) -> Result<u32, DecodeError> {
    let u32_decoded: Result<U32Msg, DecodeError> = match msg_format {
        MsgFormat::Json => { msg_json::to_msg(msg) },
        MsgFormat::Protobuf => {
            msg_proto::to_u32_msg(msg_proto::to_msg::<msg_proto_defs::U32Msg>(msg)) },
    };
    match u32_decoded {
        Ok(v) => Ok(v.val),
        Err(e) => Err(e)
    }
}

pub fn get_time_in_millis() -> u64 {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    (now.as_secs() * 1000) + (now.subsec_nanos() / 1000000) as u64
}

pub fn produce_nonce(time: u64, msg_type: &str) -> [u8;12] {
    let mut nonce: [u8;12] = [0u8;12];
    let mut num = time;
    for i in 0..8 {
        nonce[i] = (num & 0b11111111) as u8;
        num = num >> 8;
    }
    let mut rem = msg_type.len();
    if rem > 4 {rem = 4;}
    let tmp = msg_type.as_bytes();
    for i in 0..rem {
        nonce[8+i] = tmp[i];
    }
    nonce
}


use std::clone;
pub fn slice_to_vec<T: clone::Clone>(slice: &[T]) -> Vec<T> {
    let mut v = vec!();
    v.extend_from_slice(slice);
    v
}

pub fn send_err_msg(err: String) -> Vec<u8> {
    debug!("Received unknown or invalid msg: {:?}", &err);
    let mut msg = vec!();
    for b in b"ERR".iter().cloned().chain(err.into_bytes().into_iter()) {
        msg.push(b);
    }
    msg
}

// required by dha
pub fn get_smk(n: &[u8], q: &[u8]) -> [u8; 16] {
    let gab = curve25519(n, q);
    let aes_key = [0u8; 16];
    let mut aad = slice_to_vec(&gab);
    aad.push(1u8);
    let input = &[];
    let nonce = &[0u8;12];
    let mut cipher = AesGcm::new(KeySize::KeySize256, &aes_key, nonce, &aad);
    let tag = &mut [0;16];
    let output: &mut [u8] = &mut [];
    cipher.encrypt(input, output, tag);

    *tag
}

#[allow(unused_variables)]
pub fn ereport(targetinfo: Vec<u8>, report_data: Option<Vec<u8>>) -> Result<Report, String> {
    let data = match report_data {
        Some(mut d) => {
            match d.len() {
                l @ 0...63 => { for _ in l..64 { d.push(0u8) }; d },
                64 => { d },
                _ => { return Err("Report data can contain at least 64 bytes.".to_string()) },
            }
        },
        None => slice_to_vec(&[0;64]),
    };
    Ok( Report{ report_data: data,
            misc: slice_to_vec(&[0;368]) })
}

#[allow(unused_variables)]
pub fn verify_report(report: &Report) -> bool {
    true
}

pub fn report_to_vec_u8(report: &Report) -> Vec<u8> {
    let mut bytes = vec!();
    bytes.extend_from_slice(&report.report_data);
    bytes.extend_from_slice(&report.misc);
    bytes
}
