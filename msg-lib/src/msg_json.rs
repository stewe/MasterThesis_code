use rustc_serialize::{ Encodable, Decodable};
use rustc_serialize::json;
use {BoolMsg, BytesMsg, BytesVecMsg, CacheMsg, DecodeError, EncodeError, MsgPolicy, SubCacheMsg,
    U32Msg, U8Msg};
use {get_time_in_millis, slice_to_vec};
use dh_attestation::*;
use std::error::Error;

pub fn to_json_all_given(msg: Vec<u8>, msg_type: &str, mac: Option<Vec<u8>>, time: u64) -> Result<Vec<u8>, EncodeError> {
    let result = CacheMsg { msg_type: msg_type.to_string(), msg: msg, client_id: None, mac: mac, time: Some(time) };
    Ok(json::encode(&result).unwrap().into_bytes())
}

pub fn to_json<M: Encodable>(msg_type: &str, msg: &M, client_id: Option<u32>, policy: MsgPolicy, key: Option<[u8;16]>, time: Option<u64>)
    -> Result<Vec<u8>, EncodeError> {
        let msg_bytes = json::encode(&msg).unwrap().into_bytes();
        to_json_from_bytes_msg(msg_type, msg_bytes, client_id, policy, key, time)
    }


pub fn to_json_from_bytes_msg(msg_type: &str, msg: Vec<u8>, client_id: Option<u32>, policy: MsgPolicy, key: Option<[u8;16]>, time: Option<u64>)
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
            if key.is_none() { return Err( EncodeError { description: "No key given for MAC calculation.".to_string() })}
            msg_body = msg;
            Some(super::authenticate(msg_type, time, &msg_body, &key.unwrap()))
        },
        MsgPolicy::Encrypted => {
            if key.is_none() { return Err( EncodeError { description: "No key given for encryption.".to_string() })}
            let plain_msg_body = msg;
            let (output, mac) = super::encrypt(msg_type, time, &plain_msg_body, &key.unwrap());
            msg_body = output;
            Some(slice_to_vec(&mac))
        },
    };

    let result = CacheMsg { msg_type: msg_type.to_string(), msg: msg_body, client_id: client_id, mac: mac, time: Some(time) };
    Ok(json::encode(&result).unwrap().into_bytes())
}

pub fn dha_session_request(enclave_id: u32, msg_type: &str, msg_policy: MsgPolicy, key: Option<[u8;16]>, time: Option<u64>) -> Vec<u8> {
    let session_request = DhaSessionRequest{};
    to_json(msg_type, &session_request, Some(enclave_id), msg_policy, key, time).unwrap()
}

pub fn dha_responder_gen_msg1(ga: Vec<u8>, targetinfo: Vec<u8>, msg_type: &str, msg_policy: MsgPolicy, key: Option<[u8;16]>, time: Option<u64>)
-> Vec<u8> {
    let msg1 = DhaMsg1{ga: ga, targetinfo: targetinfo};
    to_json(msg_type, &msg1, None, msg_policy, key, time).unwrap()
}

pub fn dha_msg2(gb: Vec<u8>, report: Report, report_mac: [u8;16], enclave_id: u32, msg_type: &str, msg_policy: MsgPolicy, key: Option<[u8;16]>, time: Option<u64>)
-> Vec<u8> {
    let msg2 = DhaMsg2 {
        gb: gb,
        report: report,
        report_mac: report_mac
    };
    to_json(msg_type, &msg2, Some(enclave_id), msg_policy, key, time).unwrap()
}

pub fn dha_msg3(report: Report, report_mac: [u8;16], msg_type: &str, msg_policy: MsgPolicy, key: Option<[u8;16]>, time: Option<u64>)
-> Vec<u8> {
    let msg3 = DhaMsg3 {
        report: report,
        report_mac: report_mac
    };
    to_json(msg_type, &msg3, None, msg_policy, key, time).unwrap()
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

pub fn bytes_vec_msg(val: Vec<Vec<u8>>, topic: &str, msg_policy: MsgPolicy, key: Option<[u8;16]>, time: Option<u64>)
-> Result<Vec<u8>, EncodeError> {
    let msg = BytesVecMsg { val: val };
    to_json(topic, &msg, None, msg_policy, key, time)
}

pub fn sub_cache_msg(number: Option<u32>, filters: Vec<Vec<u8>>, topic: &str, msg_policy: MsgPolicy, key: Option<[u8;16]>, time: Option<u64>)
-> Result<Vec<u8>, EncodeError> {
    let msg = SubCacheMsg { number: number, filters: filters};
    to_json(topic, &msg, None, msg_policy, key, time)
}

pub fn to_cache_msg(json: Vec<u8>) -> Result<CacheMsg, DecodeError> {
    let json_str = String::from_utf8(json);
    match json_str {
        Ok(s) => {  match json::decode::<CacheMsg>(s.as_str()) {
                        Ok(msg_decoded) => { Ok(msg_decoded) },
                        Err(err) => { Err(DecodeError { description: err.description().to_string() }) },
                    }
    },
        Err(err) => Err(DecodeError { description: err.description().to_string()})
    }
}

pub fn to_msg<T: Decodable>(json: Vec<u8>) -> Result<T, DecodeError> {
    let json_str = String::from_utf8(json);
    match json_str {
        Ok(s) => {  match json::decode::<T>(s.as_str()) {
                        Ok(msg_decoded) => { Ok(msg_decoded) },
                        Err(err) => { Err(DecodeError { description: err.description().to_string() }) },
                    }
    },
        Err(err) => Err(DecodeError { description: err.description().to_string()})
    }
}
