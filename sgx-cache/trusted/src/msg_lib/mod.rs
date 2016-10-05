use collections::{String, Vec};
use core_rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
use core::fmt;
use core::iter::repeat;
use core_crypto::aes::KeySize;
use core_crypto::aes_gcm::*;
use core_crypto::aead::{AeadEncryptor, AeadDecryptor};
use get_time_in_millis;

mod msg_json;
mod msg_proto;
mod msg_proto_defs;

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

impl DecodeError {
    pub fn description(&self) -> &str {
        &self.description
    }
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Decode error: {}", self.description)
    }
}

#[derive (Debug)]
pub struct EncodeError {
    description: String,
}

impl EncodeError {
    pub fn description(&self) -> &str {
        &self.description
    }
}

impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Decode error: {}", self.description)
    }
}

// Rust blocks the automatic derivation because of 'no_std'
// !#[derive(RustcEncodable)]
pub struct CacheMsg {
    pub msg_type: String,
    pub msg: Vec<u8>,
    pub client_id: Option<u32>,
    pub mac: Option<Vec<u8>>,
    pub time: Option<u64>,
}

impl Encodable for CacheMsg {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error>{
        s.emit_struct("CacheMsg", 5, |s| {
            try!(s.emit_struct_field("msg_type", 0, |s| { self.msg_type.encode(s) }));
            try!(s.emit_struct_field("msg", 1, |s| { self.msg.encode(s) }));
            try!(s.emit_struct_field("client_id", 2, |s| { self.client_id.encode(s) }));
            try!(s.emit_struct_field("mac", 3, |s| { self.mac.encode(s) }));
            try!(s.emit_struct_field("time", 4, |s| { self.time.encode(s) }));
            Ok(())
        })
    }
}

impl Decodable for CacheMsg {
    fn decode<D: Decoder>(d: &mut D) -> Result<CacheMsg, D::Error> {
        d.read_struct("CacheMsg", 5, |d| {
            let msg_type = try!(d.read_struct_field("msg_type", 0, |d| { String::decode(d) } ));
            let msg: Vec<u8> = try!(d.read_struct_field("msg", 1, |d| { Vec::decode(d) }));
            let client_id = try!(d.read_struct_field("client_id", 2, |d| { Option::decode(d) } ));
            let mac = try!(d.read_struct_field("mac", 3, |d| { Option::decode(d) } ));
            let time = try!(d.read_struct_field("time", 4, |d| { Option::decode(d) } ));
            Ok(CacheMsg{ msg_type: msg_type, msg: msg, client_id: client_id, mac: mac, time: time })
        })
    }
}


pub struct ErrorMsg {
    pub description: String,
}


impl Encodable for ErrorMsg {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error>{
        s.emit_struct("ErrorMsg", 1, |s| {
            try!(s.emit_struct_field("description", 0, |s| { s.emit_str(&self.description) }));
            Ok(())
        })
    }
}

impl Decodable for ErrorMsg {
    fn decode<D: Decoder>(d: &mut D) -> Result<ErrorMsg, D::Error> {
        d.read_struct("ErrorMsg", 1, |d| {
            let description = try!(d.read_struct_field("description", 0, |d| { d.read_str() } ));
            Ok(ErrorMsg{ description: description })
        })
    }
}


pub struct BoolMsg {
    pub val: bool,
}

impl Encodable for BoolMsg {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error>{
        s.emit_struct("BoolMsg", 1, |s| {
            try!(s.emit_struct_field("val", 0, |s| { s.emit_bool(self.val) }));
            Ok(())
        })
    }
}

impl Decodable for BoolMsg {
    fn decode<D: Decoder>(d: &mut D) -> Result<BoolMsg, D::Error> {
        d.read_struct("BoolMsg", 1, |d| {
            let val = try!(d.read_struct_field("val", 0, |d| { d.read_bool() } ));
            Ok(BoolMsg{ val: val })
        })
    }
}

pub struct U8Msg {
    pub val: u8,
}

impl Encodable for U8Msg {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error>{
        s.emit_struct("U8Msg", 1, |s| {
            try!(s.emit_struct_field("val", 0, |s| { s.emit_u8(self.val) }));
            Ok(())
        })
    }
}

impl Decodable for U8Msg {
    fn decode<D: Decoder>(d: &mut D) -> Result<U8Msg, D::Error> {
        d.read_struct("U8Msg", 1, |d| {
            let val = try!(d.read_struct_field("val", 0, |d| { d.read_u8() } ));
            Ok(U8Msg{ val: val })
        })
    }
}

pub struct BytesMsg {
    pub val: Vec<u8>,
}

impl Encodable for BytesMsg {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error>{
        s.emit_struct("BytesMsg", 1, |s| {
            try!(s.emit_struct_field("val", 0, |s| { self.val.encode(s) }));
            Ok(())
        })
    }
}

impl Decodable for BytesMsg {
    fn decode<D: Decoder>(d: &mut D) -> Result<BytesMsg, D::Error> {
        d.read_struct("BytesMsg", 1, |d| {
            let val = try!(d.read_struct_field("val", 0, |d| { Vec::decode(d) } ));
            Ok(BytesMsg{ val: val })
        })
    }
}

pub struct BytesVecMsg {
    pub val: Vec<Vec<u8>>,
}

impl Encodable for BytesVecMsg {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error>{
        s.emit_struct("BytesVecMsg", 1, |s| {
            try!(s.emit_struct_field("val", 0, |s| { self.val.encode(s) }));
            Ok(())
        })
    }
}

impl Decodable for BytesVecMsg {
    fn decode<D: Decoder>(d: &mut D) -> Result<BytesVecMsg, D::Error> {
        d.read_struct("BytesVecMsg", 1, |d| {
            let val = try!(d.read_struct_field("val", 0, |d| { Vec::decode(d) } ));
            Ok(BytesVecMsg{ val: val })
        })
    }
}

pub struct U32Msg {
    pub val: u32,
}

impl Encodable for U32Msg {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error>{
        s.emit_struct("U32Msg", 1, |s| {
            try!(s.emit_struct_field("val", 0, |s| { s.emit_u32(self.val) }));
            Ok(())
        })
    }
}

impl Decodable for U32Msg {
    fn decode<D: Decoder>(d: &mut D) -> Result<U32Msg, D::Error> {
        d.read_struct("U32Msg", 1, |d| {
            let val = try!(d.read_struct_field("val", 0, |d| { d.read_u32() } ));
            Ok(U32Msg{ val: val })
        })
    }
}

pub struct SubCacheMsg {
    pub number: Option<u32>,
    pub filters: Vec<Vec<u8>>,
}

impl Encodable for SubCacheMsg {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error>{
        s.emit_struct("SubCacheMsg", 2, |s| {
            try!(s.emit_struct_field("number", 0, |s| { self.number.encode(s) }));
            try!(s.emit_struct_field("filters", 1, |s| {self.filters.encode(s)} ));
            Ok(())
        })
    }
}

impl Decodable for SubCacheMsg {
    fn decode<D: Decoder>(d: &mut D) -> Result<SubCacheMsg, D::Error> {
        d.read_struct("SubCacheMsg", 2, |d| {
            let number = try!(d.read_struct_field("number", 0, |d| { Option::decode(d) } ));
            let filters = try!(d.read_struct_field("filters", 1, |d| {Vec::decode(d)} ));
            Ok(SubCacheMsg{ number: number, filters: filters })
        })
    }
}


pub fn encode_cache_msg(msg: Vec<u8>, topic: &str, policy: MsgPolicy, key: Option<[u8;16]>, time: u64, msg_format: MsgFormat) -> Result<Vec<u8>, EncodeError> {
    let p = (topic, msg, None, policy, key, Some(time));
    match msg_format {
        MsgFormat::Json => msg_json::to_json_from_bytes_msg(p.0, p.1, p.2, p.3, p.4, p.5),
        MsgFormat::Protobuf => msg_proto::to_proto_from_bytes_msg(p.0, p.1, p.2, p.3, p.4, p.5),
    }
}

pub fn encode_all_given(msg: Vec<u8>, msg_type: &str, mac: Option<Vec<u8>>, time: u64, msg_format: MsgFormat) -> Result<Vec<u8>, EncodeError> {
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

pub fn encode_bool_msg(val: bool, topic: &str, policy: MsgPolicy, key: Option<[u8;16]>, msg_format: MsgFormat)
-> Result<Vec<u8>, EncodeError> {
    let time = get_time_in_millis();
    let p = (val, topic, policy, key, Some(time));
    match msg_format {
        MsgFormat::Json => msg_json::bool_msg(p.0, p.1, p.2, p.3, p.4),
        MsgFormat::Protobuf => msg_proto::bool_msg(p.0, p.1, p.2, p.3, p.4),
    }
}

pub fn encode_u8_msg(val: u8, topic: &str, policy: MsgPolicy, key: Option<[u8;16]>, msg_format: MsgFormat)
-> Result<Vec<u8>, EncodeError> {
    let time = get_time_in_millis();
    let p = (val, topic, policy, key, Some(time));
    match msg_format {
        MsgFormat::Json => msg_json::u8_msg(p.0, p.1, p.2, p.3, p.4),
        MsgFormat::Protobuf => msg_proto::u8_msg(p.0, p.1, p.2, p.3, p.4),
    }
}

pub fn encode_u32_msg(val: u32, topic: &str, policy: MsgPolicy, key: Option<[u8;16]>, msg_format: MsgFormat)
-> Result<Vec<u8>, EncodeError> {
    let time = get_time_in_millis();
    let p = (val, topic, policy, key, Some(time));
    match msg_format {
        MsgFormat::Json => msg_json::u32_msg(p.0, p.1, p.2, p.3, p.4),
        MsgFormat::Protobuf => msg_proto::u32_msg(p.0, p.1, p.2, p.3, p.4),
    }
}

pub fn encode_bytes_msg(val: Vec<u8>, topic: &str, policy: MsgPolicy, key: Option<[u8;16]>, msg_format: MsgFormat)
-> Result<Vec<u8>, EncodeError> {
    let time = get_time_in_millis();
    let p = (val, topic, policy, key, Some(time));
    match msg_format {
        MsgFormat::Json => msg_json::bytes_msg(p.0, p.1, p.2, p.3, p.4),
        MsgFormat::Protobuf => msg_proto::bytes_msg(p.0, p.1, p.2, p.3, p.4),
    }
}

pub fn encode_bytes_vec_msg(val: Vec<Vec<u8>>, topic: &str, policy: MsgPolicy, key: Option<[u8;16]>, msg_format: MsgFormat)
-> Result<Vec<u8>, EncodeError> {
    let time = get_time_in_millis();
    let p = (val, topic, policy, key, Some(time));
    match msg_format {
        MsgFormat::Json => msg_json::bytes_vec_msg(p.0, p.1, p.2, p.3, p.4),
        MsgFormat::Protobuf => msg_proto::bytes_vec_msg(p.0, p.1, p.2, p.3, p.4),
    }
}

pub fn encode_sub_cache_msg(number: Option<u32>, filters: Vec<Vec<u8>>, topic: &str, policy: MsgPolicy, key: Option<[u8;16]>, msg_format: MsgFormat)
-> Result<Vec<u8>, EncodeError> {
    let time = get_time_in_millis();
    let p = (number, filters, topic, policy, key, Some(time));
    match msg_format {
        MsgFormat::Json => msg_json::sub_cache_msg(p.0, p.1, p.2, p.3, p.4, p.5),
        MsgFormat::Protobuf => msg_proto::sub_cache_msg(p.0, p.1, p.2, p.3, p.4, p.5),
    }
}

/// encrypt with modified crate crypto
pub fn encrypt(msg_type: &str, time: u64, msg: &Vec<u8>, key: &[u8]) -> (Vec<u8>, [u8;16]){
    let nonce = produce_nonce(time, msg_type);
    //  aad: includes msg_type and timestamp
    let mut aad: Vec<u8> = vec!();
    aad.extend_from_slice(msg_type.as_bytes());
    aad.extend_from_slice(&nonce[0..8]); // = time
    let mut cipher = AesGcm::new(KeySize::KeySize128, &key, &nonce, &aad);
    let mac = &mut [0u8;16];
    let mut output: Vec<u8> = repeat(0).take(msg.len()).collect();
    cipher.encrypt(&msg, output.as_mut_slice(), mac);
    (output, *mac)
}

/// decrypt with modified crate crypto
 pub fn decrypt(mac: &Vec<u8>, time: u64, msg_type: &String, msg: &Vec<u8>, key: [u8;16]) -> (bool, Vec<u8>) {
    let nonce = produce_nonce(time, msg_type.as_str());
    //  aad: includes msg_type and timestamp
    let mut aad: Vec<u8> = vec!();
    aad.extend_from_slice(msg_type.as_bytes());
    aad.extend_from_slice(&nonce[0..8]); // = time
    let mut cipher = AesGcm::new(KeySize::KeySize128, &key, &nonce, &aad);
    let mut output: Vec<u8> = repeat(0).take(msg.len()).collect();
    (cipher.decrypt(&msg, output.as_mut_slice(), &mac), output)
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

pub fn validate_cache_msg(cache_msg: &CacheMsg, key: [u8;16]) -> bool {
    if cache_msg.mac.is_none() || cache_msg.time.is_none() { return false }
    validate(&cache_msg.mac.clone().unwrap(), cache_msg.time.unwrap(), &cache_msg.msg_type, &cache_msg.msg, key)
}

pub fn decrypt_cache_msg(cache_msg: &CacheMsg, key: [u8;16]) -> (bool, Vec<u8>) {
    if cache_msg.mac.is_none() || cache_msg.time.is_none() { return (false, vec!()) }
    decrypt(&cache_msg.mac.clone().unwrap(), cache_msg.time.unwrap(), &cache_msg.msg_type, &cache_msg.msg, key)
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

pub fn send_err_msg(err: String) -> Vec<u8> {
    // info!("Received unknown or invalid msg: {:?}", &err); // TODO msg and err?!
    let mut msg = vec!();
    for b in b"ERR".iter().cloned().chain(err.into_bytes().into_iter()) {
        msg.push(b);
    }
    msg
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

use core::clone;
pub fn slice_to_vec<T: clone::Clone>(slice: &[T]) -> Vec<T> {
    let mut v = vec!();
    v.extend_from_slice(slice);
    v
}

fn hex_to_num(ascii: u8) -> u8 {
        match ascii {
            b'0' ... b'9' => ascii-b'0',
            b'A' ... b'F' => ascii-b'A'+10,
            b'a' ... b'f' => ascii-b'a'+10,
            _ => panic!("Not hex!")
        }
    }

    fn hex_to_bytes(raw_hex: &str) -> Vec<u8> {
        raw_hex.as_bytes().chunks(2).map(|b|(hex_to_num(b[0])<<4) + hex_to_num(b[1])).collect()
    }


unsafe fn rust_crypto_aesni_setup_working_key_128(key: *const u8, round_key: *const u8) {
    asm!(
        "\
            movdqu ($1), %xmm1; \
            movdqu %xmm1, (%rax); \
            add $$0x10, %rax; \
            \
            aeskeygenassist $$0x01, %xmm1, %xmm2; \
            call 1f; \
            jmp 2f; \
            \
            1: \
            pshufd $$0xff, %xmm2, %xmm2; \
            vpslldq $$0x04, %xmm2, %xmm3; \
            ret; \
            pxor %xmm3, %xmm1; \
            vpslldq $$0x4, %xmm1, %xmm3; \
            pxor %xmm3, %xmm1; \
            vpslldq $$0x04, %xmm1, %xmm3; \
            pxor %xmm3, %xmm1; \
            pxor %xmm2, %xmm1; \
            movdqu %xmm1, (%rax); \
            add $$0x10, %rax; \
            ret; \
            \
            2: \
        "
        : "=*m{rax}" (round_key)                    // output
        : "r" (key)                                 // input
        : "{xmm1}", "{xmm2}", "{xmm3}", "memory"    // clobbers
        : "volatile"                                // options
    );
}
