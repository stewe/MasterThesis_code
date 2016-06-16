#[macro_use]
extern crate log;
// extern crate env_logger;
extern crate simple_logger;
extern crate crypto;
extern crate protobuf;
extern crate sgx_isa;

extern crate rand;
extern crate rustc_serialize;

mod msg_json;
mod msg_proto;
mod msg_proto_defs;
pub mod dh_attestation;
pub mod rust_crypto_dha;

use crypto::aes::KeySize;
use crypto::aes_gcm::*;
use crypto::aead::AeadEncryptor;
use crypto::curve25519::curve25519;

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

pub fn decode_cache_msg(msg: Vec<u8>, format: MsgFormat) -> Result<CacheMsg, DecodeError> {
    match format {
        MsgFormat::Json => { msg_json::to_msg(msg) },
        MsgFormat::Protobuf => { msg_proto::to_cache_msg(msg_proto::to_msg(msg)) }
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
