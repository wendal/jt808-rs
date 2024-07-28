

#[cfg(feature = "no_std")]
extern crate alloc;
#[cfg(feature = "no_std")]
use alloc::vec::Vec;
// #[cfg(feature = "no_std")]
// use alloc::vec::Splice;
// #[cfg(not(feature = "no_std"))]
// use std::vec::Splice;

#[cfg(feature = "no_std")]
use core::convert::*;

#[cfg(not(feature = "no_std"))]
use std::convert::*;

use crate::protocol::{unwrap, wrap};

pub trait Message<T> {
    fn encode(&self, dst: &mut Vec<u8>) -> usize;
    fn decode(src: &Vec<u8>) -> T;
}

pub trait MsgInto {
    fn into2vec(&self, dst: &mut Vec<u8>) -> usize;
}

#[derive(Debug, Clone, Hash, PartialEq, Default, Eq, Ord, PartialOrd)]
pub struct MsgMeta {
    pub msgtp: u16, // 消息ID
    pub version: u8, // 版本号,固定为1
    pub mpkg: bool,   // 是否分包
    pub codec: u8, // 编码方式, 0 - 文本, 1 - RSA
}

#[derive(Debug, Clone, Hash, PartialEq, Default, Eq, Ord, PartialOrd)]
pub struct Base {
    pub meta: MsgMeta, // 消息元数据
    pub body: Vec<u8>, // 消息体
}

impl TryFrom<Vec<u8>> for Base {
    type Error = &'static str;

    fn try_from(src: Vec<u8>) -> Result<Self, Self::Error> {
        if src.len() < 4 {
            return Err("invalid base message, length less than 4")
        }
        // 协议层解包
        let mut dst: Vec<u8> = Vec::new();
        let size = unwrap(&src, &mut dst);
        // println!("解包后数据>>> {:?}", dst);
        if size == 0 {
            return Err("invalid base message, data invaild")
        }
        // 数据解析
        let src = dst;
        let mut msgtp: u16 = 0;
        msgtp |= (src[0] as u16) << 8;
        msgtp |= src[1] as u16;

        Ok(Base {
            meta: MsgMeta {
                msgtp: msgtp,
                version: ((src[2] >> 6) & 0xFF) as u8,
                mpkg: ((src[2] >> 5) & 0x01) == 1,
                codec: ((src[2] >> 2) & 0x07),
            },
            body: src[4..].to_vec(),
        })
    }
}


impl MsgInto for Base {
    fn into2vec(&self, dst: & mut Vec<u8>) -> usize {
        let mut tmp = dst;
        let mut dst: Vec<u8> = Vec::new();
        // 消息ID
        dst.push(((self.meta.msgtp >> 8) & 0xFF) as u8);
        dst.push((self.meta.msgtp & 0xFF) as u8);

        // 消息体属性
        let mut attr: u16 = self.body.len() as u16;
        attr |= ((if self.meta.version == 0 {0} else {1}) << 14) as u16;
        attr |= ((if self.meta.mpkg {1} else {0}) << 13) as u16;
        attr |= ((if self.meta.codec == 1 {1} else {0}) << 10) as u16;
        dst.push(((attr >> 8) & 0xFF) as u8);
        dst.push(((attr) & 0xFF) as u8);

        for tmp in self.body.iter() {
            dst.push(*tmp);
        }

        // 协议层打包
        wrap(&dst, &mut tmp);
        // println!("数据包内容>>> {:?}", tmp);
        tmp.len()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_model_base() {
        let base_body: Vec<u8> = vec![0x12, 0x34];
        let base = Base {
            meta: MsgMeta {
                msgtp: 0x0001,
                version: 1,
                mpkg: false,
                codec: 0,
            },
            body: base_body,
        };
        let mut dst: Vec<u8> = Vec::new();
        base.into2vec(&mut dst);
        // println!(">>> {:?}", dst);
        // println!(">>> {:?}", base.meta);
        assert_eq!(base.meta.msgtp, 0x0001);
        assert_eq!(base.meta.version, 1);
        assert_eq!(base.meta.mpkg, false);
        assert_eq!(base.meta.codec, 0);

        let base2 = Base::try_from(dst).unwrap();
        assert_eq!(base2.meta, base.meta);
        assert_eq!(base2.body, base.body);
    }
}
