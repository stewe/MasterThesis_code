extern crate msg_lib;
extern crate sgx_isa;
#[macro_use]
extern crate log;
// extern crate env_logger;
extern crate simple_logger;

pub mod client_enclave;

use msg_lib::MsgFormat;

pub const MSG_FORMAT: MsgFormat = MsgFormat::Protobuf;
// const MSG_FORMAT: MsgFormat = MsgFormat::Json;
