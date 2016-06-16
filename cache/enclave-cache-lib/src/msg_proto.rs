    use protobuf::{ Message, MessageStatic, parse_from_bytes, ProtobufError };
    use {CacheMsg, DecodeError, MsgPolicy, };
    use dh_attestation::*;
    use msg_proto_defs as pbmsgs;
    use std::error::Error;
    use slice_to_vec;

    pub fn dha_session_request_proto(enclave_id: u32, msg_type: &str, msg_policy: MsgPolicy) -> Vec<u8> {
        let session_request = pbmsgs::DhaSessionRequest::new();
        to_proto(msg_type, &session_request, Some(enclave_id), msg_policy)
    }

    pub fn dha_responder_gen_msg1(ga: Vec<u8>, targetinfo: Vec<u8>, msg_type: &str, msg_policy: MsgPolicy) -> Vec<u8> {
        let mut msg1 = pbmsgs::DhaMsg1::new();
        msg1.set_ga(slice_to_vec(&ga));
        msg1.set_targetinfo(slice_to_vec(&targetinfo));
        to_proto(msg_type, &msg1, None, msg_policy)
    }


    pub fn dha_msg2(gb: Vec<u8>, report: Report, report_mac: [u8;16], enclave_id: u32, msg_type: &str, msg_policy: MsgPolicy) -> Vec<u8> {
        let mut msg2 = pbmsgs::DhaMsg2::new();
        msg2.set_gb(slice_to_vec(&gb));
        let mut report_proto = pbmsgs::Report::new();
        report_proto.set_report_data(report.report_data);
        report_proto.set_misc(report.misc);
        msg2.set_report(report_proto);
        msg2.set_report_mac(slice_to_vec(&report_mac));

        to_proto(msg_type, &msg2, Some(enclave_id), msg_policy)
    }

    pub fn dha_msg3(report: Report, report_mac: [u8;16], msg_type: &str, msg_policy: MsgPolicy) -> Vec<u8> {
        let mut msg3 = pbmsgs::DhaMsg3::new();
        let mut report_proto = pbmsgs::Report::new();
        report_proto.set_report_data(report.report_data);
        report_proto.set_misc(report.misc);
        msg3.set_report(report_proto);
        msg3.set_report_mac(slice_to_vec(&report_mac));

        to_proto(msg_type, &msg3, None, msg_policy)
    }

    pub fn to_proto<M : Message + MessageStatic>(msg_type: &str, msg: &M, client_id: Option<u32>, policy: MsgPolicy)
        -> Vec<u8> {
        let mut result = pbmsgs::CacheMsg::new();
        result.set_msg_type(msg_type.to_string());
        match client_id{
            Some(id) => { result.set_client_id(id); }
            None => {}
        }

        match policy {
            MsgPolicy::Plain => {   let msg_body = msg.write_to_bytes().unwrap();
                                    result.set_msg(msg_body);   },
            MsgPolicy::Authenticated => {}, // TODO
            MsgPolicy::Encrypted => {},     // TODO
        }

        result.write_to_bytes().unwrap()
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
            Ok(c) => { Ok(CacheMsg{client_id: if c.has_client_id() { Some(c.get_client_id()) } else { None },
                                    msg_type: c.get_msg_type().to_string(),
                                    msg: { slice_to_vec(c.get_msg()) },
                                    mac: if c.has_mac() { Some(slice_to_vec(c.get_mac())) } else { None },
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
