

extern crate alloc;
use alloc::vec::Vec;

/// 数据封包
pub fn wrap(src: &Vec<u8>, dst: &mut Vec<u8>) -> usize {
    let mut result = 3;
    let mut crc = 0;
    dst.push(0x7E);
    for val in src.iter() { 
        let tmp: u8 = *val;
        if result == 3 {
            crc = tmp;
        }
        else {
            crc = crc ^ tmp;
        }
        // println!("tmp {:02X} crc {:02X}", tmp, crc);
        if tmp == 0x7E {
            dst.push(0x7D);
            dst.push(0x02);
            result += 2;
        }
        else if tmp == 0x7D {
            dst.push(0x7D);
            dst.push(0x01);
            result += 2;
        }
        else {
            dst.push(tmp);
            result += 1;
        }
    }
    dst.push(crc);
    dst.push(0x7E);
    result
}

/// 数据解包
pub fn unwrap(src: &Vec<u8>, dst: &mut Vec<u8>) -> usize {
    if src.len() < 4 {
        return 0;
    }
    if src[0] != 0x7E || src[src.len()-1] != 0x7E {
        return 0;
    }
    let mut ret = 0;
    let mut i = 1;
    let mut tmp:u8;
    let mut crc = 0;
    while i < src.len()-2 {
        tmp = src[i];
        if tmp == 0x7D {
            if src[i+1] == 0x02 {
                tmp = 0x7E;
                i+=1;
            }
            else if src[i+1] == 0x01 {
                tmp = 0x7D;
                i+=1;
            }
            else {
                return 0; // 非法数据
            }
        }
        dst.push(tmp);
        if i == 2 {
            crc = tmp;
        }
        else {
            crc = crc ^ tmp;
        }
        
        i+=1;
        ret+=1;
    }
    ret
}



#[cfg(test)]
mod tests {

    #[test]
    fn test_real_data() {
        let mut dst: Vec<u8> = vec![0; 128];
        dst.clear();
        let src: Vec<u8> = vec![0x7e, 0x81, 0x04, 0x00, 0x00, 0x33, 0x33, 0x33, 0x33, 0x30, 0x01, 0x00, 0x21, 0x95, 0x7e];
        let raw: Vec<u8> = vec![0x81, 0x04, 0x00, 0x00, 0x33, 0x33, 0x33, 0x33, 0x30, 0x01, 0x00, 0x21];

        // 封包测试
        let result = super::wrap(&raw, &mut dst);
        // println!(">>>>>>> {}", result);
        assert_eq!(result, src.len());
        // println!("crc {:02X}", dst[13]);
        
        for i in 0..src.len() {
            if dst[i] != src[i] {
                println!("不一致!!! 位置{} 期望{:02X} 实际{:02X}", i, src[i], dst[i]);
            }
            assert!(dst[i] == src[i]);
        }

        // 解包测试
        dst.clear();
        let result: usize = super::unwrap(&src, &mut dst);
        // println!(">>>>>>> {}", result);
        assert_eq!(result, raw.len());
        for i in 0..raw.len() {
            if dst[i] != raw[i] {
                println!("不一致!!! 位置{} 期望{:02X} 实际{:02X}", i, raw[i], dst[i]);
            }
            assert!(dst[i] == raw[i]);
        }

        // 再写一个测试
        dst.clear();
        let raw: Vec<u8> = vec![0x81, 0x7E, 0x7D, 0x21];
        let result = super::wrap(&raw, &mut dst);
        // println!(">>>>>>> {}", result);
        assert_eq!(result, 9);
        // println!(">>>>>>> {:02X}", dst[7]);
        assert!(dst[7] == 0xA3);

        let raw = vec![0x7E, 0x81, 0x7D, 0x02, 0x7D, 0x01, 0x21, 0xA3, 0x7E];
        for i in 0..raw.len() {
            if dst[i] != raw[i] {
                println!("不一致!!! 位置{} 期望{:02X} 实际{:02X}", i, raw[i], dst[i]);
            }
            assert!(dst[i] == raw[i]);
        }

    }
    
    #[test]
    fn test_wrap() {
        let mut dst: Vec<u8> = vec![0; 128];

        // 无需转义的情况
        let src: Vec<u8> = vec![0x12, 0x34, 0x56, 0x78];
        dst.clear();
        let result: usize = super::wrap(&src, &mut dst);
        assert_eq!(result, 7);
        assert_eq!(dst[0], 0x7E);
        assert_eq!(dst[1], 0x12);
        assert_eq!(dst[2], 0x34);
        assert_eq!(dst[3], 0x56);
        assert_eq!(dst[4], 0x78);
        assert_eq!(dst[6], 0x7E);

        
        // 需转义的情况, 有 0x7E
        let src: Vec<u8> = vec![0x12, 0x34, 0x56, 0x78, 0x7E, 0x12];
        dst.clear();
        let result: usize = super::wrap(&src, &mut dst);
        // println!(">>>>>>> {}", result);
        assert_eq!(result, 10);
        assert_eq!(dst[0], 0x7E);
        assert_eq!(dst[1], 0x12);
        assert_eq!(dst[2], 0x34);
        assert_eq!(dst[3], 0x56);
        assert_eq!(dst[4], 0x78);
        assert_eq!(dst[5], 0x7D);
        assert_eq!(dst[6], 0x02);
        assert_eq!(dst[7], 0x12);
        assert_eq!(dst[9], 0x7E);

        // 需转义的情况, 有 0x7D
        let src: Vec<u8> = vec![0x12, 0x34, 0x56, 0x78, 0x7D, 0x12];
        dst.clear();
        let result: usize = super::wrap(&src, &mut dst);
        // println!(">>>>>>> {}", result);
        assert_eq!(result, 10);
        assert_eq!(dst[0], 0x7E);
        assert_eq!(dst[1], 0x12);
        assert_eq!(dst[2], 0x34);
        assert_eq!(dst[3], 0x56);
        assert_eq!(dst[4], 0x78);
        assert_eq!(dst[5], 0x7D);
        assert_eq!(dst[6], 0x01);
        assert_eq!(dst[7], 0x12);
        assert_eq!(dst[9], 0x7E);

        // 7E 和 7D 同时出现的情况
        let src: Vec<u8> = vec![0x30, 0x7E, 0x08, 0x7D];
        dst.clear();
        let result: usize = super::wrap(&src, &mut dst);
        // println!(">>>>>>> {}", result);
        assert_eq!(result, 9);
        assert_eq!(dst[0], 0x7E);
        assert_eq!(dst[1], 0x30);
        assert_eq!(dst[2], 0x7D);
        assert_eq!(dst[3], 0x02);
        assert_eq!(dst[4], 0x08);
        assert_eq!(dst[5], 0x7D);
        assert_eq!(dst[6], 0x01);
        // println!(">>>>>>> {:02x}", dst[7]);
        assert_eq!(dst[8], 0x7E);

    }
}