use super::msg_proto_defs as pbmsgs;
use super::{BoolMsg, BytesVecMsg, CacheMsg, DecodeError, EncodeError, MsgPolicy,
    SubCacheMsg, U32Msg, U8Msg};
use get_time_in_millis;
use super::slice_to_vec;
use collections::string::ToString;
use collections::vec::Vec;
use core_protobuf::{ Message, MessageStatic, parse_from_bytes, ProtobufError, RepeatedField };


pub fn bool_msg(val: bool, topic: &str, msg_policy: MsgPolicy,
                key: Option<[u8;16]>, time: Option<u64>)
-> Result<Vec<u8>, EncodeError> {
    let mut bool_msg = pbmsgs::BoolMsg::new();
    bool_msg.set_val(val);
    to_proto(topic, &bool_msg, None, msg_policy, key, time)
}

pub fn u8_msg(val: u8, topic: &str, msg_policy: MsgPolicy,
                key: Option<[u8;16]>, time: Option<u64>)
-> Result<Vec<u8>, EncodeError> {
    let mut u8_msg = pbmsgs::U8Msg::new();
    u8_msg.set_val(val as u32);
    to_proto(topic, &u8_msg, None, msg_policy, key, time)
}

pub fn u32_msg(val: u32, topic: &str, msg_policy: MsgPolicy,
                key: Option<[u8;16]>, time: Option<u64>)
-> Result<Vec<u8>, EncodeError> {
    let mut u32_msg = pbmsgs::U32Msg::new();
    u32_msg.set_val(val);
    to_proto(topic, &u32_msg, None, msg_policy, key, time)
}

pub fn bytes_msg(val: Vec<u8>, topic: &str, msg_policy: MsgPolicy,
                key: Option<[u8;16]>, time: Option<u64>)
-> Result<Vec<u8>, EncodeError> {
    let mut bytes_msg = pbmsgs::BytesMsg::new();
    bytes_msg.set_val(val);
    to_proto(topic, &bytes_msg, None, msg_policy, key, time)
}

pub fn bytes_vec_msg(val: Vec<Vec<u8>>, topic: &str, msg_policy: MsgPolicy,
                    key: Option<[u8;16]>, time: Option<u64>)
-> Result<Vec<u8>, EncodeError> {
    let mut bytes_vec_msg = pbmsgs::BytesVecMsg::new();
    let repeated = RepeatedField::from_vec(val);
    bytes_vec_msg.set_val(repeated);
    to_proto(topic, &bytes_vec_msg, None, msg_policy, key, time)
}

pub fn sub_cache_msg(number: Option<u32>, filters: Vec<Vec<u8>>, topic: &str,
                    msg_policy: MsgPolicy, key: Option<[u8;16]>, time: Option<u64>)
-> Result<Vec<u8>, EncodeError> {
    let mut sub_cache_msg = pbmsgs::SubCacheMsg::new();
    if let Some(n) = number { sub_cache_msg.set_number(n) };
    let repeated = RepeatedField::from_vec(filters);
    sub_cache_msg.set_filters(repeated);
    to_proto(topic, &sub_cache_msg, None, msg_policy, key, time)
}

pub fn to_proto_all_given(msg: Vec<u8>, msg_type: &str, mac: Option<Vec<u8>>, time: u64)
-> Result<Vec<u8>, EncodeError> {
    let mut result = pbmsgs::CacheMsg::new();
    result.set_msg_type(msg_type.to_string());
    result.set_msg(msg);
    if let Some(m) = mac { result.set_mac(m); };
    result.set_time(time);
    Ok(result.write_to_bytes().unwrap())
}


pub fn to_proto<M : Message + MessageStatic>(msg_type: &str, msg: &M, client_id: Option<u32>,
                                        policy: MsgPolicy, key: Option<[u8;16]>, time: Option<u64>)
-> Result<Vec<u8>, EncodeError> {
    let msg_bytes = msg.write_to_bytes().unwrap();
    to_proto_from_bytes_msg(msg_type, msg_bytes, client_id, policy, key, time)
}

pub fn to_proto_from_bytes_msg(msg_type: &str, msg: Vec<u8>, client_id: Option<u32>,
                                policy: MsgPolicy, key: Option<[u8;16]>, time: Option<u64>)
    -> Result<Vec<u8>, EncodeError> {
    let mut result = pbmsgs::CacheMsg::new();
    result.set_msg_type(msg_type.to_string());
    if let Some(id) = client_id { result.set_client_id(id); };
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
            if key.is_none() { return Err(EncodeError { description:
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

    result.set_msg(msg_body);
    if mac.is_some() {
        result.set_mac(mac.unwrap());
        result.set_time(time)
    }
    Ok(result.write_to_bytes().unwrap())
}


pub fn to_msg<T: Message + MessageStatic>(protobuf: Vec<u8>) -> Result<T, DecodeError> {
    let msg_decoded: Result<T, ProtobufError> = parse_from_bytes(&protobuf);
    match msg_decoded {
        Ok(msg) => Ok(msg),
        Err(err) =>  Err(DecodeError { description: err.description().to_string()}),
    }
}

pub fn to_cache_msg(from: Result<pbmsgs::CacheMsg, DecodeError>) -> Result<CacheMsg, DecodeError> {
    match from {
        Ok(c) => { Ok(CacheMsg{ msg_type: c.get_msg_type().to_string(),
                                msg: { slice_to_vec(c.get_msg()) },
                                client_id: if c.has_client_id() { Some(c.get_client_id()) } else { None },
                                mac: if c.has_mac() { Some(slice_to_vec(c.get_mac())) } else { None },
                                time: if c.has_time() { Some(c.get_time()) } else { None },
                            }) },
        Err(err) => Err(DecodeError { description: err.description().to_string()}),
    }
}

pub fn to_bool_msg(from: pbmsgs::BoolMsg) -> BoolMsg {
    BoolMsg { val: from.get_val() }
}

pub fn to_u8_msg(from: pbmsgs::U8Msg) -> U8Msg {
    U8Msg { val: from.get_val() as u8 }
}

pub fn to_u32_msg(from: Result<pbmsgs::U32Msg, DecodeError>) -> Result<U32Msg, DecodeError> {
    match from {
        Ok(m) => Ok(U32Msg { val: m.get_val() }),
        Err(e) => Err(e),
    }
}

pub fn to_bytes_vec_msg(from: Result<pbmsgs::BytesVecMsg, DecodeError>)
    -> Result<BytesVecMsg, DecodeError> {
        match from {
            Ok(m) => Ok(BytesVecMsg { val: slice_to_vec(m.get_val()) }),
            Err(e) => Err(e),
        }
}

pub fn to_sub_cache_msg(from: Result<pbmsgs::SubCacheMsg, DecodeError>)
-> Result<SubCacheMsg, DecodeError> {
    match from {
        Ok(m) => { Ok(SubCacheMsg {number: if m.has_number() { Some(m.get_number()) } else { None } ,
                                    filters: slice_to_vec(&m.get_filters())}) },
        Err(e) => Err(e),
    }
}
