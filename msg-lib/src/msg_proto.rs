    use protobuf::{ Message, MessageStatic, parse_from_bytes, ProtobufError, RepeatedField };
    use {BoolMsg, BytesMsg, BytesVecMsg, CacheMsg, DecodeError, EncodeError, MsgPolicy,
        SubCacheMsg, U32Msg, U8Msg};
    use {get_time_in_millis, slice_to_vec};
    use dh_attestation::*;
    use msg_proto_defs as pbmsgs;
    use std::error::Error;

    pub fn dha_session_request_proto(enclave_id: u32, msg_type: &str, msg_policy: MsgPolicy, key: Option<[u8;16]>, time: Option<u64>) -> Vec<u8> {
        let session_request = pbmsgs::DhaSessionRequest::new();
        to_proto(msg_type, &session_request, Some(enclave_id), msg_policy, key, time).unwrap()
    }

    pub fn dha_responder_gen_msg1(ga: Vec<u8>, targetinfo: Vec<u8>, msg_type: &str, msg_policy: MsgPolicy, key: Option<[u8;16]>, time: Option<u64>) -> Vec<u8> {
        let mut msg1 = pbmsgs::DhaMsg1::new();
        msg1.set_ga(slice_to_vec(&ga));
        msg1.set_targetinfo(slice_to_vec(&targetinfo));
        to_proto(msg_type, &msg1, None, msg_policy, key, time).unwrap()
    }


    pub fn dha_msg2(gb: Vec<u8>, report: Report, report_mac: [u8;16], enclave_id: u32, msg_type: &str, msg_policy: MsgPolicy, key: Option<[u8;16]>, time: Option<u64>) -> Vec<u8> {
        let mut msg2 = pbmsgs::DhaMsg2::new();
        msg2.set_gb(slice_to_vec(&gb));
        let mut report_proto = pbmsgs::Report::new();
        report_proto.set_report_data(report.report_data);
        report_proto.set_misc(report.misc);
        msg2.set_report(report_proto);
        msg2.set_report_mac(slice_to_vec(&report_mac));

        to_proto(msg_type, &msg2, Some(enclave_id), msg_policy, key, time).unwrap()
    }

    pub fn dha_msg3(report: Report, report_mac: [u8;16], msg_type: &str, msg_policy: MsgPolicy, key: Option<[u8;16]>, time: Option<u64>) -> Vec<u8> {
        let mut msg3 = pbmsgs::DhaMsg3::new();
        let mut report_proto = pbmsgs::Report::new();
        report_proto.set_report_data(report.report_data);
        report_proto.set_misc(report.misc);
        msg3.set_report(report_proto);
        msg3.set_report_mac(slice_to_vec(&report_mac));

        to_proto(msg_type, &msg3, None, msg_policy, key, time).unwrap()
    }

    pub fn bool_msg(val: bool, topic: &str, msg_policy: MsgPolicy, key: Option<[u8;16]>, time: Option<u64>)
    -> Result<Vec<u8>, EncodeError> {
        let mut bool_msg = pbmsgs::BoolMsg::new();
        bool_msg.set_val(val);
        to_proto(topic, &bool_msg, None, msg_policy, key, time)
    }

    pub fn u8_msg(val: u8, topic: &str, msg_policy: MsgPolicy, key: Option<[u8;16]>, time: Option<u64>)
    -> Result<Vec<u8>, EncodeError> {
        let mut u8_msg = pbmsgs::U8Msg::new();
        u8_msg.set_val(val as u32);
        to_proto(topic, &u8_msg, None, msg_policy, key, time)
    }

    pub fn u32_msg(val: u32, topic: &str, msg_policy: MsgPolicy, key: Option<[u8;16]>, time: Option<u64>)
    -> Result<Vec<u8>, EncodeError> {
        let mut u32_msg = pbmsgs::U32Msg::new();
        u32_msg.set_val(val);
        to_proto(topic, &u32_msg, None, msg_policy, key, time)
    }

    pub fn bytes_msg(val: Vec<u8>, topic: &str, msg_policy: MsgPolicy, key: Option<[u8;16]>, time: Option<u64>)
    -> Result<Vec<u8>, EncodeError> {
        let mut bytes_msg = pbmsgs::BytesMsg::new();
        bytes_msg.set_val(val);
        to_proto(topic, &bytes_msg, None, msg_policy, key, time)
    }

    pub fn bytes_vec_msg(val: Vec<Vec<u8>>, topic: &str, msg_policy: MsgPolicy, key: Option<[u8;16]>, time: Option<u64>)
    -> Result<Vec<u8>, EncodeError> {
        let mut bytes_vec_msg = pbmsgs::BytesVecMsg::new();
        let repeated = RepeatedField::from_vec(val);
        bytes_vec_msg.set_val(repeated);
        to_proto(topic, &bytes_vec_msg, None, msg_policy, key, time)
    }

    pub fn sub_cache_msg(number: Option<u32>, filters: Vec<Vec<u8>>, topic: &str, msg_policy: MsgPolicy, key: Option<[u8;16]>, time: Option<u64>)
    -> Result<Vec<u8>, EncodeError> {
        let mut sub_cache_msg = pbmsgs::SubCacheMsg::new();
        if let Some(n) = number { sub_cache_msg.set_number(n) };
        let repeated = RepeatedField::from_vec(filters);
        sub_cache_msg.set_filters(repeated);
        to_proto(topic, &sub_cache_msg, None, msg_policy, key, time)
    }


    pub fn to_proto_all_given(msg: Vec<u8>, msg_type: &str, mac: Option<Vec<u8>>, time: u64) -> Result<Vec<u8>, EncodeError> {
        let mut result = pbmsgs::CacheMsg::new();
        result.set_msg_type(msg_type.to_string());
        result.set_msg(msg);
        if let Some(m) = mac { result.set_mac(m); };
        result.set_time(time);
        Ok(result.write_to_bytes().unwrap())
    }


    pub fn to_proto<M : Message + MessageStatic>(msg_type: &str, msg: &M, client_id: Option<u32>, policy: MsgPolicy, key: Option<[u8;16]>, time: Option<u64>)
    -> Result<Vec<u8>, EncodeError> {
        let msg_bytes = msg.write_to_bytes().unwrap();
        to_proto_from_bytes_msg(msg_type, msg_bytes, client_id, policy, key, time)
    }

    pub fn to_proto_from_bytes_msg(msg_type: &str, msg: Vec<u8>, client_id: Option<u32>, policy: MsgPolicy, key: Option<[u8;16]>, time: Option<u64>)
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

    pub fn to_dha_msg1(from: Result<pbmsgs::DhaMsg1, DecodeError>) -> Result<DhaMsg1, DecodeError> {
            match from {
                Ok(msg1) => { Ok(DhaMsg1{
                    ga: slice_to_vec(msg1.get_ga()),
                    targetinfo: slice_to_vec(msg1.get_targetinfo()),
                }) },
                Err(err) => Err(err),
            }
    }

    pub fn to_dha_msg2(from: Result<pbmsgs::DhaMsg2, DecodeError>) -> Result<DhaMsg2, DecodeError> {
        match from {
                Ok(msg2) => {
                    let mut report_mac: [u8; 16] = [0;16];
                    for (i, val) in msg2.get_report_mac().iter().cloned().enumerate() {
                        report_mac[i] = val;
                    }
                    Ok(DhaMsg2{
                    gb: slice_to_vec(msg2.get_gb()),
                    report: Report{report_data: slice_to_vec(msg2.get_report().get_report_data()),
                                    misc: slice_to_vec(msg2.get_report().get_misc()) },
                    report_mac: report_mac,
                }) },
                Err(err) => Err(err),
            }
    }

    pub fn to_dha_msg3(from: Result<pbmsgs::DhaMsg3, DecodeError>) -> Result<DhaMsg3, DecodeError> {
        match from {
            Ok(msg3) => {
                let mut report_mac: [u8; 16] = [0;16];
                for (i, val) in msg3.get_report_mac().iter().cloned().enumerate() {
                    report_mac[i] = val;
                }
                Ok(DhaMsg3{
                    report: Report{report_data: slice_to_vec(msg3.get_report().get_report_data()),
                                    misc: slice_to_vec(msg3.get_report().get_misc()) },
                    report_mac: report_mac,
            })},
            Err(err) => Err(err),
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

    pub fn to_bytes_msg(from: Result<pbmsgs::BytesMsg, DecodeError>) -> Result<BytesMsg, DecodeError> {
        match from {
            Ok(m) => Ok(BytesMsg { val: slice_to_vec(m.get_val()) }),
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
