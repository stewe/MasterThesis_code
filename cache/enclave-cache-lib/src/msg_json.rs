use rustc_serialize::{ Encodable, Decodable};
use rustc_serialize::json;
use {CacheMsg, DecodeError, MsgPolicy};
use dh_attestation::*;
use std::error::Error;

// no pure json! msg_type+json
fn to_json<M: Encodable>(msg_type: &str, msg: &M, client_id: Option<u32>, policy: MsgPolicy)
    -> Vec<u8> {
    let msg_body;
    let mac = match policy {
        MsgPolicy::Plain => {   msg_body = json::encode(&msg).unwrap().into_bytes(); // TODO error handling! ???
                                None },
        MsgPolicy::Authenticated => {   msg_body = vec!();
                                        None }, // TODO
        MsgPolicy::Encrypted => {   msg_body = vec!();
                                    None },     // TODO
    };

    let result = CacheMsg { client_id: client_id, msg_type: msg_type.to_string(), msg: msg_body, mac: mac };
    json::encode(&result).unwrap().into_bytes()
}

pub fn dha_session_request(enclave_id: u32, msg_type: &str, msg_policy: MsgPolicy) -> Vec<u8> {
    let session_request = DhaSessionRequest{};
    to_json(msg_type, &session_request, Some(enclave_id), msg_policy)
}

pub fn dha_responder_gen_msg1(ga: Vec<u8>, targetinfo: Vec<u8>, msg_type: &str, msg_policy: MsgPolicy) -> Vec<u8> {
    let msg1 = DhaMsg1{ga: ga, targetinfo: targetinfo};
    to_json(msg_type, &msg1, None, msg_policy)
}

pub fn dha_msg2(gb: Vec<u8>, report: Report, report_mac: [u8;16], enclave_id: u32, msg_type: &str, msg_policy: MsgPolicy) -> Vec<u8> {
    let msg2 = DhaMsg2 {
        gb: gb,
        report: report,
        report_mac: report_mac
    };
    to_json(msg_type, &msg2, Some(enclave_id), msg_policy)
}

pub fn dha_msg3(report: Report, report_mac: [u8;16], msg_type: &str, msg_policy: MsgPolicy) -> Vec<u8> {
    let msg3 = DhaMsg3 {
        report: report,
        report_mac: report_mac
    };
    to_json(msg_type, &msg3, None, msg_policy)
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
