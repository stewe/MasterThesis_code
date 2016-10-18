use core_rustc_serialize::{ Encodable, Decodable};
use core_rustc_serialize::json;
use super::{BoolMsg, BytesMsg, BytesVecMsg, CacheMsg, DecodeError, EncodeError, MsgPolicy, SubCacheMsg,
    U32Msg, U8Msg};
use super::slice_to_vec;
use get_time_in_millis;
use collections::string::{String, ToString};
use collections::vec::Vec;

pub fn to_json_all_given(msg: Vec<u8>, msg_type: &str, mac: Option<Vec<u8>>, time: u64)
-> Result<Vec<u8>, EncodeError> {
    let result = CacheMsg { msg_type: msg_type.to_string(), msg: msg, client_id: None,
                            mac: mac, time: Some(time) };
    Ok(json::encode(&result).unwrap().into_bytes())
}

pub fn to_json<M: Encodable>(msg_type: &str, msg: &M, client_id: Option<u32>,
                            policy: MsgPolicy, key: Option<[u8;16]>, time: Option<u64>)
    -> Result<Vec<u8>, EncodeError> {
        let msg_bytes = json::encode(&msg).unwrap().into_bytes();
        to_json_from_bytes_msg(msg_type, msg_bytes, client_id, policy, key, time)
}

pub fn to_json_from_bytes_msg(msg_type: &str, msg: Vec<u8>, client_id: Option<u32>,
                            policy: MsgPolicy, key: Option<[u8;16]>, time: Option<u64>)
    -> Result<Vec<u8>, EncodeError> {
    let msg_body;

    let time = match time {
        Some(t) => t,
        None => get_time_in_millis(),
    };

    let mac = match policy {
        MsgPolicy::Plain => {
            msg_body = msg;
            None
        },
        MsgPolicy::Authenticated => {
            if key.is_none() { return Err( EncodeError { description:
                                            "No key given for MAC calculation.".to_string() })}
            msg_body = msg;
            Some(super::authenticate(msg_type, time, &msg_body, &key.unwrap()))
        },
        MsgPolicy::Encrypted => {
            if key.is_none() { return Err( EncodeError { description:
                                                "No key given for encryption.".to_string() })}
            let plain_msg_body = msg;
            let (output, mac) = super::encrypt(msg_type, time, &plain_msg_body, &key.unwrap());
            msg_body = output;
            Some(slice_to_vec(&mac))
        },
    };

    let result = CacheMsg { msg_type: msg_type.to_string(), msg: msg_body,
                            client_id: client_id, mac: mac, time: Some(time) };
    Ok(json::encode(&result).unwrap().into_bytes())
}

pub fn bool_msg(val: bool, topic: &str, msg_policy: MsgPolicy, key: Option<[u8;16]>, time: Option<u64>)
-> Result<Vec<u8>, EncodeError> {
    let msg = BoolMsg { val: val };
    to_json(topic, &msg, None, msg_policy, key, time)
}

pub fn u8_msg(val: u8, topic: &str, msg_policy: MsgPolicy, key: Option<[u8;16]>, time: Option<u64>)
-> Result<Vec<u8>, EncodeError> {
    let msg = U8Msg { val: val };
    to_json(topic, &msg, None, msg_policy, key, time)
}

pub fn u32_msg(val: u32, topic: &str, msg_policy: MsgPolicy, key: Option<[u8;16]>, time: Option<u64>)
-> Result<Vec<u8>, EncodeError> {
    let msg = U32Msg { val: val };
    to_json(topic, &msg, None, msg_policy, key, time)
}

pub fn bytes_msg(val: Vec<u8>, topic: &str, msg_policy: MsgPolicy, key: Option<[u8;16]>, time: Option<u64>)
-> Result<Vec<u8>, EncodeError> {
    let msg = BytesMsg { val: val };
    to_json(topic, &msg, None, msg_policy, key, time)
}

pub fn bytes_vec_msg(val: Vec<Vec<u8>>, topic: &str, msg_policy: MsgPolicy,
                        key: Option<[u8;16]>, time: Option<u64>)
-> Result<Vec<u8>, EncodeError> {
    let msg = BytesVecMsg { val: val };
    to_json(topic, &msg, None, msg_policy, key, time)
}

pub fn sub_cache_msg(number: Option<u32>, filters: Vec<Vec<u8>>, topic: &str,
                        msg_policy: MsgPolicy, key: Option<[u8;16]>, time: Option<u64>)
-> Result<Vec<u8>, EncodeError> {
    let msg = SubCacheMsg { number: number, filters: filters};
    to_json(topic, &msg, None, msg_policy, key, time)
}

pub fn to_cache_msg(json: Vec<u8>) -> Result<CacheMsg, DecodeError> {
    let json_str = String::from_utf8(json);
    match json_str {
        Ok(s) => {  match json::decode::<CacheMsg>(s.as_str()) {
                        Ok(msg_decoded) => { Ok(msg_decoded) },
                        Err(_) => { Err(DecodeError { description:
                                                "Failed parsing a valid JSON.".to_string() }) },
                    }
    },
        Err(_) => Err(DecodeError { description: "Failed casting into a valid UTF8 string.".to_string()})
    }
}

pub fn to_msg<T: Decodable>(json: Vec<u8>) -> Result<T, DecodeError> {
    let json_str = String::from_utf8(json);
    match json_str {
        Ok(s) => {  match json::decode::<T>(s.as_str()) {
                        Ok(msg_decoded) => { Ok(msg_decoded) },
                        Err(_) => { Err(DecodeError { description:
                                                "Failed parsing a valid JSON.".to_string() }) },
                    }
    },
        Err(_) => Err(DecodeError { description: "Failed casting into a valid UTF8 string.".to_string()})
    }
}
