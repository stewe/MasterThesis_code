#[macro_use]
extern crate log;
// extern crate env_logger;
extern crate simple_logger;
extern crate crypto;
extern crate protobuf;
extern crate sgx_isa;

extern crate rand;
extern crate rustc_serialize;

pub mod dh_attestation;
pub mod rustcryptodhamsgs;  // TODO remove pub?

use crypto::curve25519::{curve25519, curve25519_base};
use crypto::aes::KeySize;
use crypto::aes_gcm::*;
use crypto::aead::{AeadEncryptor, AeadDecryptor};
use crypto::digest::Digest;
use crypto::sha2::Sha256;

use rustc_serialize::{ Encodable, Decoder, Encoder };

use std::error::Error;
use std::fmt;
use dh_attestation::*;
// use sgx_isa::{Targetinfo};

pub enum MsgPolicy {
    Plain,
    Authenticated,
    Encrypted
}


#[derive(Copy, Clone, Debug)]
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


#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct CacheMsg {
    pub client_id: Option<u32>,
    pub msg_type: String,
    pub msg: Vec<u8>,
    pub mac: Option<Vec<u8>>
}


#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct ErrorMsg {
    pub description: String,
}


#[derive(Copy, Clone, Debug)]
pub struct RustCryptoDHA {
    pub state: DhaState,
    /// n: ECDH private key
    pub n: [u8;32],
    // ga: ECDH public key of responder
    pub ga: [u8;32],
    // gb: ECDH public key of initiator
    pub gb: [u8;32],
    pub smk: [u8;16],
    pub msg_format: self::MsgFormat,
}

impl RustCryptoDHA {
    pub fn new(msg_format: MsgFormat, state: DhaState) -> RustCryptoDHA {
        RustCryptoDHA{
            state: state,
            n: [0;32],
            ga: [0;32],
            gb: [0;32],
            smk: [0;16],
            msg_format: msg_format,
        }
    }
    pub fn set_msg_format(&mut self, msg_format: MsgFormat) {
        self.msg_format = msg_format;
    }
}

impl DHAttestation<Vec<u8>> for RustCryptoDHA {

    fn dha_init_session(&mut self, role: DhaRole){
        self.n = rand::random::<[u8;32]>();
        match role {
            DhaRole::Initiator => { self.gb = curve25519_base(&self.n); },
            DhaRole::Responder => { self.ga = curve25519_base(&self.n); },
        }
    }

    fn dha_session_request(&mut self, enclave_id: u32) -> Vec<u8> {
        let p = (enclave_id, "REQ", MsgPolicy::Plain);
        match self.msg_format {
            MsgFormat::Json => json_::dha_session_request(p.0, p.1, p.2),
            MsgFormat::Protobuf => proto::dha_session_request_proto(p.0, p.1, p.2),
        }
    }

    fn dha_responder_gen_msg1(&mut self, session_request: Vec<u8>, targetinfo: Vec<u8>)
        -> Result<Vec<u8>, DecodeError> {
        // ga + targetinfo

        // 1. deserialize session_request and check it (json and protobuf)!
        // !!! session_request contains no information, thus no need to decode!
        // match from_json::<DhaSessionRequest>(session_request) {
        //     Ok(_) => { },
        //     Err(err) => {return Err(DecodeError { description: err.description().to_string()}) },
        // };

        // 2. produce msg1
        let ga = slice_to_vec(&self.ga);
        let p = (ga, targetinfo, "MS1", MsgPolicy::Plain);
        match self.msg_format {
            MsgFormat::Json => Ok(json_::dha_responder_gen_msg1(p.0, p.1, p.2, p.3)),
            MsgFormat::Protobuf => Ok(proto::dha_responder_gen_msg1(p.0, p.1, p.2, p.3)),
        }
    }


    fn dha_initiator_proc_msg1(&mut self, msg1: Vec<u8>, enclave_id: u32) -> Result<Vec<u8>, DecodeError> {
        let msg1_decoded: Result<DhaMsg1, DecodeError> = match self.msg_format {
            MsgFormat::Json => { json_::to_msg(msg1) },
            MsgFormat::Protobuf => {
                proto::to_dha_msg1(proto::to_msg::<self::rustcryptodhamsgs::DhaMsg1>(msg1)) },
        };

        let p = match msg1_decoded {
            Ok(m1) => {
                let ga = m1.ga;
                for (i, val) in ga.iter().enumerate() {
                    self.ga[i] = val.clone();
                }
                let smk = get_smk(&self.n, &ga);
                self.smk = smk;

                let gb = self.gb;

                let mut hasher = Sha256::new();
                let ga_gb: Vec<u8> = ga.iter().cloned().chain(gb.iter().cloned()).collect();
                hasher.input(&ga_gb);
                let h_ga_gb: &mut [u8] = &mut [0u8;32];
                hasher.result(h_ga_gb);

                let report = ereport(m1.targetinfo, Some(slice_to_vec(h_ga_gb))).unwrap();

                let aad: Vec<u8> = report_to_vec_u8(&report);

                let nonce = &gb[..12];    // TODO?
                let mut cipher = AesGcm::new(KeySize::KeySize128, &smk, nonce, &aad);
                let report_mac = &mut [0;16];
                cipher.encrypt(&[], &mut [], report_mac);

                (slice_to_vec(&gb), report, *report_mac, enclave_id , "MS2", MsgPolicy::Plain)
            },
            Err(err) => return Err(err),
        };

        match self.msg_format {
            MsgFormat::Json => Ok(json_::dha_msg2(p.0, p.1, p.2, p.3, p.4, p.5)),
            MsgFormat::Protobuf => Ok(proto::dha_msg2(p.0, p.1, p.2, p.3, p.4, p.5)),
        }
    }

    fn dha_responder_proc_msg2(&mut self, msg2: Vec<u8>) -> Result<Vec<u8>, DecodeError> {
        let msg2_decoded: Result<DhaMsg2, DecodeError> = match self.msg_format {
            MsgFormat::Json => { json_::to_msg(msg2) },
            MsgFormat::Protobuf => {
                proto::to_dha_msg2(proto::to_msg::<self::rustcryptodhamsgs::DhaMsg2>(msg2)) },
        };

        let p = match msg2_decoded {
            Ok(m2) => {
                let ga = self.ga;
                let gb = m2.gb;
                for (i, val) in gb.iter().enumerate() {
                    self.gb[i] = val.clone();
                }

                let smk = get_smk(&self.n, &gb);
                self.smk = smk;

                let initiator_report = m2.report;
                info!("report: {:?}", initiator_report);

                let aad: Vec<u8> = report_to_vec_u8(&initiator_report);
                info!("report/aad: {:?}", aad);
                let nonce = &gb[..12];    // TODO?
                let mut cipher = AesGcm::new(KeySize::KeySize128, &smk, nonce, &aad);

                // validate report mac
                if !cipher.decrypt(&[], &mut [], &m2.report_mac) {
                    return Err(DecodeError { description: "Report MAC invalid.".to_string()}) }

                // validate report
                if !verify_report(&initiator_report) {
                    return Err(DecodeError { description: "Report invalid.".to_string()}) }

                let mut hasher = Sha256::new();
                // 1. validate report_data ga_gb
                let ga_gb: Vec<u8> = ga.iter().cloned().chain(gb.iter().cloned()).collect();
                hasher.input(&ga_gb);
                let h_ga_gb: &mut [u8] = &mut [0u8;32];
                hasher.result(h_ga_gb);
                for (i, j) in initiator_report.report_data.iter().zip(h_ga_gb.iter()) {
                    if i != j {
                        return Err(DecodeError { description: "Report key hash invalid.".to_string()}) }
                }

                // 2.
                let gb_ga: Vec<u8> = gb.iter().cloned().chain(ga.iter().cloned()).collect();
                hasher.reset();
                hasher.input(&gb_ga);
                let h_gb_ga: &mut [u8] = &mut [0u8;32];
                hasher.result(h_gb_ga);

                let targetinfo = initiator_report.get_targetinfo();
                let report = ereport(targetinfo, Some(slice_to_vec(h_gb_ga))).unwrap();

                let aad: Vec<u8> = report_to_vec_u8(&report);
                let nonce = &gb[..12];
                let mut cipher = AesGcm::new(KeySize::KeySize128, &smk, nonce, &aad);
                let report_mac = &mut [0;16];
                cipher.encrypt(&[], &mut [], report_mac);

                (report, *report_mac, "MS3", MsgPolicy::Plain)
            },
            Err(err) => return Err(err),
        };

        match self.msg_format {
            MsgFormat::Json => Ok(json_::dha_msg3(p.0, p.1, p.2, p.3)),
            MsgFormat::Protobuf => Ok(proto::dha_msg3(p.0, p.1, p.2, p.3)),
        }
    }

    fn dha_initiator_proc_msg3(&mut self, msg3: Vec<u8>) -> Result<Vec<u8>, DecodeError> {
        let msg3_decoded: Result<DhaMsg3, DecodeError> = match self.msg_format {
            MsgFormat::Json => { json_::to_msg(msg3) },
            MsgFormat::Protobuf => {
                proto::to_dha_msg3(proto::to_msg::<self::rustcryptodhamsgs::DhaMsg3>(msg3)) },
        };

        match msg3_decoded {
            Ok(m3) => {
                let (ga, gb) = (self.ga, self.gb);

                let responder_report = m3.report;

                let aad: Vec<u8> = report_to_vec_u8(&responder_report);
                let nonce = &gb[..12];    // TODO?
                let mut cipher = AesGcm::new(KeySize::KeySize128, &self.smk, nonce, &aad);

                // validate report mac
                if !cipher.decrypt(&[], &mut [], &m3.report_mac) {
                    return Err(DecodeError { description: "Report MAC invalid.".to_string()}) }

                // validate report
                if !verify_report(&responder_report) {
                    return Err(DecodeError { description: "Report invalid.".to_string()}) }

                let mut hasher = Sha256::new();
                // 1. validate report_data ga_gb
                let gb_ga: Vec<u8> = gb.iter().cloned().chain(ga.iter().cloned()).collect();
                hasher.input(&gb_ga);
                let h_gb_ga: &mut [u8] = &mut [0u8;32];
                hasher.result(h_gb_ga);

                for (i, j) in responder_report.report_data.iter().zip(h_gb_ga.iter()) {
                    if i != j {
                        return Err(DecodeError { description: "Report key hash invalid.".to_string()}) }
                }
                Ok(vec!())
            },
            Err(err) => return Err(err),
        }

        // match self.msg_format {
        //     MsgFormat::Json => Ok(json_::dha_msg3(p.0, p.1, p.2, p.3)),
        //     MsgFormat::Protobuf => Ok(proto::dha_msg3(p.0, p.1, p.2, p.3)),
        // }

    }
}

// fn dha_responder_gen_msg1_json (session_request: Vec<u8>, targetinfo: Targetinfo,
//     msg_type: &str, msg_policy: MsgPolicy) -> (Vec<u8>, DhaSessionRequest){
//         let session_request_decoded = json::decode::<>(String::from_utf8(session_request).unwrap().as_str());
// }

pub mod json_ {
    use rustc_serialize::{ Encodable, Decodable};
    use rustc_serialize::json;
    use super::{CacheMsg, DecodeError, MsgPolicy};
    use super::dh_attestation::*;
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
}


pub mod proto {
    use protobuf::{ Message, MessageStatic, parse_from_bytes, ProtobufError };
    use super::{CacheMsg, DecodeError, MsgPolicy, };
    use super::dh_attestation::*;
    use super::rustcryptodhamsgs as pbmsgs;
    use std::error::Error;
    use super::slice_to_vec;
    use std::mem::transmute;

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
}

// SMK / aek [out]
// A pointer that points to instance of sgx_key_128bit_t. The aek is derived from the Diffie-Hell- man shared secret elliptic curve field element between the two enclaves:
// aek = AES-CMAC (0x00, gab x coordinate|| 0x01)
// The AES-CMAC key used in the AES-CMAC operation is 16 bytes of 0x00. The plain text used in the AES-CMAC calculation is the Diffie-Hellman shared secret elliptic curve field element in Little Endian format followed by one byte of 0x01.
pub fn get_smk(n: &[u8], q: &[u8]) -> [u8; 16] {
    let gab = curve25519(n, q);
    let aes_key = [0u8; 16];
    let mut aad = slice_to_vec(&gab);
    aad.push(1u8);
    let input = &[];
    let nonce = &[0u8;12];    // TODO?
    let mut cipher = AesGcm::new(KeySize::KeySize256, &aes_key, nonce, &aad);
    let tag = &mut [0;16];
    let output: &mut [u8] = &mut [];
    cipher.encrypt(input, output, tag);

    *tag
}

use std::clone;
pub fn slice_to_vec<T: clone::Clone>(slice: &[T]) -> Vec<T> {
    let mut v = vec!();
    v.extend_from_slice(slice);
    v
}

//TODO error msg generation and handling -> create defined msg type!
pub fn send_err_msg(err: String) -> Vec<u8> {
    // info!("Received unknown or invalid msg: {:?}", &err); // TODO msg and err?!
    let mut msg = vec!();
    for b in b"ERR".iter().cloned().chain(err.into_bytes().into_iter()) {
        msg.push(b);
    }
    msg
}

// impl From<Report> for Targetinfo {
// 	fn from(r: Report) -> Targetinfo {
// 		Targetinfo{
// 			measurement: r.mrenclave,
// 			attributes: r.attributes,
// 			miscselect: r.miscselect,
// 			..Targetinfo::default()
// 		}
// 	}
// }

// pub fn ereport(tinfo: &Targetinfo, rdata: &[u8; 64]) -> Report {
// 	ereport_internal(Some(tinfo),Some(rdata))
// }

/// Checks whether the report was generated on the same processor with this
/// enclave specified in the target.
// pub fn verify_report(report: &Report) -> bool {
// 	let req=Keyrequest{
// 		keyname: Keyname::Report as u16,
// 		keyid: report.keyid,
// 		..Default::default()
// 	};
// 	let key=egetkey(&req);
// 	let mac_data=unsafe{::core::slice::from_raw_parts(report as *const _ as *const u8,384)};
// 	aes::cmac_128(&key,mac_data)==report.mac
// }

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

pub fn verify_report(report: &Report) -> bool {
    true
}

pub fn report_to_vec_u8(report: &Report) -> Vec<u8> {
    let mut bytes = vec!();
    bytes.extend_from_slice(&report.report_data);
    bytes.extend_from_slice(&report.misc);
    bytes
}

// pub struct Report {
// 	pub cpusvn:     [u8; 16],
// 	pub miscselect: Miscselect,
// 	pub _reserved1: [u8; 28],
// 	pub attributes: Attributes,
// 	pub mrenclave:  [u8; 32],
// 	pub _reserved2: [u8; 32],
// 	pub mrsigner:   [u8; 32],
// 	pub _reserved3: [u8; 96],
// 	pub isvprodid:  u16,
// 	pub isvsvn:     u16,
// 	pub _reserved4: [u8; 60],
// 	pub reportdata: [u8; 64],
// 	pub keyid:      [u8; 32],
// 	pub mac:        [u8; 16],
// }


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
