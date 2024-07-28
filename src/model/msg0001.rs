
extern crate alloc;
#[cfg(feature = "no_std")]
use alloc::vec::Vec;

// use crate::message::*;
use crate::model::*;

#[derive(Debug, Clone, Hash, PartialEq, Default, Eq, Ord, PartialOrd)]
pub struct CommonResp {
    meta: MsgMeta, // 消息头部
    msgid: u16, // 应答流水号
    respid: u16, // 应答ID,对应平台消息的ID
    result: u8, // 0 - 成功, 其他失败
    body: Vec<u8>, // 消息体
}

impl Message<CommonResp> for CommonResp {
    fn encode(&self, dst: &mut Vec<u8>) -> usize {
        todo!()
    }

    fn decode(src: &Vec<u8>) -> CommonResp {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_model_common_resp() {
        
    }
}