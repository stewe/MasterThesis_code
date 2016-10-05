#![no_std]

pub enum ECall {
    InitUserHeap = 1,
    HandleRequest = 2,
    HandleSubMsg = 3,
}

impl ECall {
    pub fn from_u64(num: u64) -> Option<ECall> {
        match num {
            1 => Some(ECall::InitUserHeap),
            2 => Some(ECall::HandleRequest),
            3 => Some(ECall::HandleSubMsg),
            _ => None,
        }
    }
}

