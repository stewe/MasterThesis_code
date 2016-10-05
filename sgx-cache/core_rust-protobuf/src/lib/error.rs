use io;
use collections::string::String;
// use core::error::Error;
use core::fmt;

pub type ProtobufResult<T> = Result<T, ProtobufError>;

#[derive(Debug)]
pub enum ProtobufError {
    IoError(io::Error),
    WireError(String),
    MessageNotInitialized { message: &'static str },
}

impl ProtobufError {
    pub fn message_not_initialized(message: &'static str) -> ProtobufError {
        ProtobufError::MessageNotInitialized {
            message: message
        }
    }
}

impl fmt::Display for ProtobufError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[allow(dead_code)]
impl ProtobufError {
     pub fn description(&self) -> &str {
         match self {
             // not sure that cause should be included in message
             &ProtobufError::IoError(_) => "IoError",//e.description(),
             &ProtobufError::WireError(ref e) => &e,
             &ProtobufError::MessageNotInitialized { .. } => "not all message fields set",
         }
     }
//
//     fn cause(&self) -> Option<&Error> {
//         match self {
//             &ProtobufError::IoError(ref e) => Some(e),
//             &ProtobufError::WireError(..) => None,
//             &ProtobufError::MessageNotInitialized { .. } => None,
//         }
//     }
 }
