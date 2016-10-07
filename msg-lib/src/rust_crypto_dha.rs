use dh_attestation::*;
use {msg_json, msg_proto, msg_proto_defs};
use {DecodeError, MsgFormat, MsgPolicy, ereport, get_smk, report_to_vec_u8, slice_to_vec, verify_report};

use rand;

use crypto::aes::KeySize;
use crypto::aes_gcm::*;
use crypto::aead::{AeadEncryptor, AeadDecryptor};
use crypto::curve25519::curve25519_base;
use crypto::digest::Digest;
use crypto::sha2::Sha256;


#[derive(Copy, Clone, Debug)]
pub struct RustCryptoDHA {
    pub state: DhaState,
    /// n: ECDH private key
    pub n: [u8;32],
    /// ga: ECDH public key of responder
    pub ga: [u8;32],
    /// gb: ECDH public key of initiator
    pub gb: [u8;32],
    pub smk: [u8;16],
    pub msg_format: super::MsgFormat,
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
        let p = (enclave_id, "REQ", MsgPolicy::Plain, None, None);
        match self.msg_format {
            MsgFormat::Json => msg_json::dha_session_request(p.0, p.1, p.2, p.3, p.4),
            MsgFormat::Protobuf => msg_proto::dha_session_request_proto(p.0, p.1, p.2, p.3, p.4),
        }
    }

    #[allow(unused_variables)]
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
        let p = (ga, targetinfo, "MS1", MsgPolicy::Plain, None, None);
        match self.msg_format {
            MsgFormat::Json => Ok(msg_json::dha_responder_gen_msg1(p.0, p.1, p.2, p.3, p.4, p.5)),
            MsgFormat::Protobuf => Ok(msg_proto::dha_responder_gen_msg1(p.0, p.1, p.2, p.3, p.4, p.5)),
        }
    }


    fn dha_initiator_proc_msg1(&mut self, msg1: Vec<u8>, enclave_id: u32) -> Result<Vec<u8>, DecodeError> {
        let msg1_decoded: Result<DhaMsg1, DecodeError> = match self.msg_format {
            MsgFormat::Json => { msg_json::to_msg(msg1) },
            MsgFormat::Protobuf => {
                msg_proto::to_dha_msg1(msg_proto::to_msg::<msg_proto_defs::DhaMsg1>(msg1)) },
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

                (slice_to_vec(&gb), report, *report_mac, enclave_id , "MS2", MsgPolicy::Plain, None, None)
            },
            Err(err) => return Err(err),
        };

        match self.msg_format {
            MsgFormat::Json => Ok(msg_json::dha_msg2(p.0, p.1, p.2, p.3, p.4, p.5, p.6, p.7)),
            MsgFormat::Protobuf => Ok(msg_proto::dha_msg2(p.0, p.1, p.2, p.3, p.4, p.5, p.6, p.7)),
        }
    }

    fn dha_responder_proc_msg2(&mut self, msg2: Vec<u8>) -> Result<Vec<u8>, DecodeError> {
        let msg2_decoded: Result<DhaMsg2, DecodeError> = match self.msg_format {
            MsgFormat::Json => { msg_json::to_msg(msg2) },
            MsgFormat::Protobuf => {
                msg_proto::to_dha_msg2(msg_proto::to_msg::<msg_proto_defs::DhaMsg2>(msg2)) },
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

                let aad: Vec<u8> = report_to_vec_u8(&initiator_report);
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

                (report, *report_mac, "MS3", MsgPolicy::Plain, None, None)
            },
            Err(err) => return Err(err),
        };

        match self.msg_format {
            MsgFormat::Json => Ok(msg_json::dha_msg3(p.0, p.1, p.2, p.3, p.4, p.5)),
            MsgFormat::Protobuf => Ok(msg_proto::dha_msg3(p.0, p.1, p.2, p.3, p.4, p.5)),
        }
    }

    fn dha_initiator_proc_msg3(&mut self, msg3: Vec<u8>) -> Result<Vec<u8>, DecodeError> {
        let msg3_decoded: Result<DhaMsg3, DecodeError> = match self.msg_format {
            MsgFormat::Json => { msg_json::to_msg(msg3) },
            MsgFormat::Protobuf => {
                msg_proto::to_dha_msg3(msg_proto::to_msg::<msg_proto_defs::DhaMsg3>(msg3)) },
        };

        match msg3_decoded {
            Ok(m3) => {
                info!("Received request: {:?}", &m3);
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
