use super::enclave_cache_lib::dh_attestation::*;
use super::enclave_cache_lib::*;
use std::error::Error;

static ENCLAVE_ID: u32 = 987654321u32;

// configure which format to use
// const MSG_FORMAT: MsgFormat = MsgFormat::Json;
const MSG_FORMAT: MsgFormat = MsgFormat::Protobuf;

static mut DHA: RustCryptoDHA = RustCryptoDHA{
    state: DhaState::Initator(DhaInitiatorState::Start),
    n: [0;32],
    ga: [0;32],
    gb: [0;32],
    smk: [0;16],
    msg_format: MSG_FORMAT,
};

static mut CACHE_INITIALIZED: bool = false;


// static MRENCLAVE: u32 = 123456789u32;
// static TARGETINFO: [u8; 512]; //TODO

pub fn foo() -> Vec<u8> {
    /*
        TODO...
        - create enclave mod for cache
        - session_request from client, response from cache with msg1!
        - state diagram for DHA
        - apply state!!!!!
    */

    unsafe {
        if !CACHE_INITIALIZED {
            info!("DEBUG DEBUG DEBUG DEBUG DEBUG DEBUG DEBUG DEBUG DEBUG DEBUG DEBUG DEBUG ");
            DHA.dha_init_session(DhaRole::Initiator);
            CACHE_INITIALIZED = true;
            let session_request = DHA.dha_session_request(ENCLAVE_ID);
            DHA.state = DhaState::Initator(DhaInitiatorState::Requested);
            return session_request
        }
    }

    // let mut session_request = vec!();
    // unsafe { session_request = DHA.dha_session_request(ENCLAVE_ID); }
    // unsafe { msg1 = DHA.dh_responder_gen_msg1( MRENCLAVE.clone()); }
    // info!("msg1: {:?}{:?}", &msg1[0..32], &msg1[32..]);

    // TODO remove this debug stuff
    // unsafe { assert_eq!(&msg1[0..32], DHA.q); }
    // use std::mem::transmute;
    // let mre: u32 = unsafe { transmute([msg1[32], msg1[33], msg1[34], msg1[35]]) };
    // assert_eq!(mre, MRENCLAVE);

    use std::thread::sleep;
    use std::time::Duration;
    sleep(Duration::new(2, 0));

    vec!()
}

pub fn ecall_handle_request(msg: Vec<u8>) -> Vec<u8> {
    // TODO apply protocol (state)
    let msg_decoded: Result<CacheMsg, DecodeError> = match MSG_FORMAT {
        MsgFormat::Json => { json_::to_msg(msg) },
        MsgFormat::Protobuf => { proto::to_cache_msg(proto::to_msg(msg)) }
    };
    let result = match msg_decoded {
        Err(err) => send_err_msg(format!("Invalid or unknown message: {}", err)),
        Ok(m) => {  info!("Received request: {:?}", &m);
                    handle_request(m) }};

    result
}


fn handle_request(cache_msg: CacheMsg) -> Vec<u8> {
    if cache_msg.mac.is_some() {
        // TODO
        // get SMK for cache_msg.enclave_id
        // verify, decrypt
    }

    // decode match MSG_FORMAT...
    match cache_msg.msg_type.as_str() {
        "MS1" =>  {
            unsafe {
                // check state
                // TODO initiate a new session request? cache has no valid session anymore
                if DHA.state != DhaState::Initator(DhaInitiatorState::Requested) {
                    return send_err_msg("No session requested.".to_string()) }

                let msg2 = match DHA.dha_initiator_proc_msg1(cache_msg.msg, ENCLAVE_ID) {
                    Ok(m2) => m2,
                    Err(err) => send_err_msg(err.description().to_string()),
                };
                DHA.state = DhaState::Initator(DhaInitiatorState::Msg2Sent);
                info!("DHA = {:?}", DHA);
                msg2
            }
        },
        "MS3" => unsafe {
            if DHA.state != DhaState::Initator(DhaInitiatorState::Msg2Sent){
                return send_err_msg("Unexpected MS3.".to_string()) }

            let resp = match DHA.dha_initiator_proc_msg3(cache_msg.msg) {
                Ok(m3) => m3,
                Err(err) => send_err_msg(err.description().to_string()),
            };
            DHA.state = DhaState::Responder(DhaResponderState::Active);
            resp
        },
        // "ERR" => {  },   // TODO
        _ => send_err_msg("Unknown message type.".to_string()),
    }

}
