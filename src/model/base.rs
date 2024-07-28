extern crate alloc;
#[cfg(feature = "no_std")]
use alloc::vec::Vec;
// #[cfg(feature = "no_std")]
// use alloc::vec::Splice;
// #[cfg(not(feature = "no_std"))]
// use std::vec::Splice;

pub trait Message<T> {
    fn encode(&self, dst: &mut Vec<u8>) -> usize;
    fn decode(src: &Vec<u8>) -> T;
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

impl Message<Base> for Base {
    fn encode(&self, dst: & mut Vec<u8>) -> usize {
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
        2 + self.body.len()
    }

    fn decode(src: &Vec<u8>) -> Base {
        let mut msgtp: u16 = 0;
        msgtp |= (src[0] as u16) << 8;
        msgtp |= src[1] as u16;

        Base {
            meta: MsgMeta {msgtp: msgtp,
                version: ((src[2] >> 6) & 0xFF) as u8,
                mpkg: ((src[2] >> 5) & 0x01) == 1,
                codec: ((src[2] >> 2) & 0x07),
            },
            body: src[4..].to_vec(),
        }
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
        let mut dst = Vec::new();
        base.encode(&mut dst);
        // println!(">>> {:?}", dst);
        println!(">>> {:?}", base.meta);
        assert_eq!(base.meta.msgtp, 0x0001);
        assert_eq!(base.meta.version, 1);
        assert_eq!(base.meta.mpkg, false);
        assert_eq!(base.meta.codec, 0);

        let base2 = Base::decode(&dst);
        assert_eq!(base2.meta, base.meta);
        assert_eq!(base2.body, base.body);
    }
}