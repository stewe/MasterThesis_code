extern crate sgx_isa;

// use sgx_isa::{Report, Targetinfo};
use super::DecodeError;
use super::rustc_serialize::{Decodable, Encodable, Decoder, Encoder};

pub enum DhaRole {
    Initiator,
    Responder,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DhaResponderState {
    Start,
    Msg1Sent,
    Active,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DhaInitiatorState {
    Start,
    Requested,
    Msg2Sent,
    Active,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DhaState {
    Initator(DhaInitiatorState),
    Responder(DhaResponderState),
}

pub trait DHAttestation<T> {
    fn dha_init_session(&mut self, role: DhaRole);
    fn dha_session_request(&mut self, enclave_id: u32) -> T;
    fn dha_responder_gen_msg1(&mut self, session_request: T, targetinfo: Vec<u8>)//[u8; 512] Targetinfo)
        -> Result<T, DecodeError>;
    fn dha_initiator_proc_msg1(&mut self, msg1: T, enclave_id: u32) -> Result<T, DecodeError>;
    fn dha_responder_proc_msg2(&mut self, msg2: T) -> Result<T, DecodeError>;
    fn dha_initiator_proc_msg3(&mut self, msg3: T) -> Result<T, DecodeError>;
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct DhaSessionRequest { }

#[derive(RustcDecodable, RustcEncodable)]
pub struct DhaMsg1 {
    pub ga: Vec<u8>,    //[u8; 32],
    pub targetinfo: Vec<u8>,    //TargetinfoWrapper, //[u8; 512],//Targetinfo,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct DhaMsg2 {
    pub gb: Vec<u8>,    //[u8; 32],
    pub report: Report,       //Report -> Decoder, Wrapper...
    pub report_mac: [u8; 16],
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Report {
    pub report_data: Vec<u8>,   //[u8; 64],
    pub misc: Vec<u8>,
}

impl Report {
    pub fn get_targetinfo(&self) -> Vec<u8> {
        let mut targetinfo = vec!();
        for i in 0..512 {
            targetinfo.push((i / 2) as u8);
        }
        targetinfo
    }
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct DhaMsg3 {
    pub report: Report,
    pub report_mac: [u8; 16],
}


pub struct TargetinfoWrapper {
    // pub targetinfo: Targetinfo,
    pub targetinfo: [u8; 512],
}

impl Encodable for TargetinfoWrapper {
    // impl<T:Encodable> Encodable for Vec<T> {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_seq(512, |s| {
            for (i, e) in self.targetinfo.iter().enumerate() {
                try!(s.emit_seq_elt(i, |s| e.encode(s)))
            }
            Ok(())
        })
    }
}

impl Decodable for TargetinfoWrapper {
    fn decode<D: Decoder>(d: &mut D) -> Result<TargetinfoWrapper, D::Error> {
       d.read_seq(|d, len| {
           let mut a = [0;512];
           for i in 0..len {
               a[i] = try!(d.read_seq_elt(i, |d| Decodable::decode(d)));
           }
           Ok(TargetinfoWrapper{targetinfo: a})
       })
   }
}
