
extern crate alloc;
#[cfg(feature = "no_std")]
use alloc::vec::Vec;
// #[cfg(feature = "no_std")]
// use alloc::vec::Splice;
// #[cfg(not(feature = "no_std"))]
// use std::vec::Splice;

trait Message<T> {
    fn encode(&self, dst: &mut Vec<u8>) -> usize;
    fn decode(src: &Vec<u8>) -> T;
}

#[derive(Debug, Clone, Hash, PartialEq, Default, Eq, Ord, PartialOrd)]
pub struct MsgMeta {
    msgtp: u16, // 消息ID
    version: u8, // 版本号,固定为1
    mpkg: bool,   // 是否分包
    codec: u8, // 编码方式, 0 - 文本, 1 - RSA
}

#[derive(Debug, Clone, Hash, PartialEq, Default, Eq, Ord, PartialOrd)]
pub struct Base {
    meta: MsgMeta, // 消息元数据
    body: Vec<u8>, // 消息体
}

impl Message<Base> for Base {
    fn encode(&self, dst: & mut Vec<u8>) -> usize {
        // 消息ID
        dst.push(((self.meta.msgtp >> 8) & 0xFF) as u8);
        dst.push(((self.meta.msgtp >> 8) & 0xFF) as u8);

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
            body: src[3..].to_vec(),
        }
    }
}

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

#[derive(Debug, Clone, Hash, PartialEq, Default, Eq, Ord, PartialOrd)]
pub struct HeartBeat {
    meta: MsgMeta, // 消息头部
}

/// 注册
pub struct RegReq {
    meta: MsgMeta, // 消息头部
    province: u16, // 省域id
    city: u16, // 城市id
    manufacturer: [u8;11], // 厂商id
    model: [u8;30], // 设备型号
    device: [u8;30], // 设备编号
    color: u8,
    carid: String, // 车牌号
}

pub struct RegResp {
    meta: MsgMeta, // 消息头部
    msgid: u16, // 应答流水号
    result: u8, // 注册结果
    authcode: String, // 鉴权码
}

/// 注销
pub struct UnRegReq {
    meta: MsgMeta, // 消息头部

}
/// 鉴权
pub struct Auth {
    meta: MsgMeta, // 消息头部
    code: String, // 鉴权码
    imei: [u8;15], // imei
    version: [u8;20], // 版本号
}

/// 透传数据
pub struct RawData {
    meta: MsgMeta, // 消息头部
    data: String
}

/// 位置上报
pub struct LocationReport {
    meta: MsgMeta, // 消息头部

}

/// 定位数据批量上传
pub struct LocationBatchReport {
    meta: MsgMeta, // 消息头部

}
