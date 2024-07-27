

extern crate alloc;
use alloc::vec::Vec;


pub fn wrap_array(src: &[u8], dst: &mut [u8]) -> usize {
    let mut i = 0;
    dst[0] = 0x7E;
    i+=1;
    for val in src.iter() { 
        if *val == 0x7E {
            dst[i] = 0x7D;
            dst[i+1] = 0x02;
            i+=2;
        }
        else if *val == 0x7D {
            dst[i] = 0x7D;
            dst[i+1] = 0x01;
            i+=2;
        }
        else {
            dst[i] = *val;
            i += 1;
        }
    }
    dst[i] = 0x7E;
    i
}

pub fn unwrap_array(src: &[u8], dst: &mut [u8]) -> usize {
    let mut i = 0;
    if src[0] != 0x7E {
        return 0;
    }
    let mut mode = false;
    for val in src.iter() { 
        if *val == 0x7E {
            break;
        }
        if *val == 0x7D {
            mode = true;
            continue;
        }
        else if mode {
            if *val == 0x02 {
                dst[i] = 0x7E;
                mode = false;
            }
            else if *val == 0x01 {
                dst[i] = 0x7D;
                mode = false;
            }
            else {
                return 0; // 非法数据
            }
        }
        else {
            dst[i] = *val;
        }
        i+=1;
    }
    i
}


pub fn wrap(src: Vec<u8>, dst: &mut Vec<u8>) -> usize {
    let mut result = 2;
    dst.push(0x7E);
    for val in src.iter() { 
        if *val == 0x7E {
            dst.push(0x7D);
            dst.push(0x02);
            result += 2;
        }
        else if *val == 0x7D {
            dst.push(0x7D);
            dst.push(0x01);
            result += 2;
        }
        else {
            dst.push(val.clone());
            result += 1;
        }
    }
    dst.push(0x7E);
    result
}


pub fn unwrap(src: Vec<u8>, dst: &mut Vec<u8>) -> usize {
    if src.len() < 3 {
        return 0;
    }
    if src[0] != 0x7E || src[src.len()-1] != 0x7E {
        return 0;
    }
    let mut ret = 0;
    let mut i = 1;
    while i < src.len()-1 {
        let tmp = src[i];
        if tmp == 0x7D {
            if src[i+1] == 0x02 {
                dst.push(0x7E);
                i+=1;
            }
            else if src[i+1] == 0x01 {
                dst.push(0x7D);
                i+=1;
            }
            else {
                return 0; // 非法数据
            }
        }
        else {
            dst.push(tmp);
        }
        i+=1;
        ret+=1;
    }
    ret
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_unwrap() {
        let mut dst: Vec<u8> = vec![0; 128];

        // 无需转义的情况
        let src: Vec<u8> = vec![0x7E, 0x12, 0x34, 0x56, 0x78, 0x7E];
        dst.clear();
        let result: usize = super::unwrap(src, &mut dst);
        // println!(">>>>>>> {}", result);
        assert_eq!(result, 4);
        assert_eq!(dst[0], 0x12);
        assert_eq!(dst[1], 0x34);
        assert_eq!(dst[2], 0x56);
        assert_eq!(dst[3], 0x78);

        // 需转义的情况, 有 0x7E
        let src: Vec<u8> = vec![0x7E, 0x12, 0x7D, 0x02, 0x99, 0x78, 0x7E];
        dst.clear();
        let result: usize = super::unwrap(src, &mut dst);
        println!(">>>>>>> {}", result);
        assert_eq!(result, 4);
        assert_eq!(dst[0], 0x12);
        assert_eq!(dst[1], 0x7E);
        assert_eq!(dst[2], 0x99);
        assert_eq!(dst[3], 0x78);

        
        // 需转义的情况, 有 0x7D
        let src: Vec<u8> = vec![0x7E, 0x12, 0x7D, 0x01, 0x99, 0x78, 0x7E];
        dst.clear();
        let result: usize = super::unwrap(src, &mut dst);
        // println!(">>>>>>> {}", result);
        assert_eq!(result, 4);
        assert_eq!(dst[0], 0x12);
        assert_eq!(dst[1], 0x7D);
        assert_eq!(dst[2], 0x99);
        assert_eq!(dst[3], 0x78);

        // 7E 和 7D 同时出现的情况
        let src: Vec<u8> = vec![0x7E, 0x12, 0x7D, 0x01, 0x7D, 0x02, 0x78, 0x7E];
        dst.clear();
        let result: usize = super::unwrap(src, &mut dst);
        // println!(">>>>>>> {}", result);
        assert_eq!(result, 4);
        assert_eq!(dst[0], 0x12);
        assert_eq!(dst[1], 0x7D);
        assert_eq!(dst[2], 0x7E);
        assert_eq!(dst[3], 0x78);

    }

    
    #[test]
    fn test_wrap() {
        let mut dst: Vec<u8> = vec![0; 128];

        // 无需转义的情况
        let src: Vec<u8> = vec![0x12, 0x34, 0x56, 0x78];
        dst.clear();
        let result: usize = super::wrap(src, &mut dst);
        assert_eq!(result, 6);

        
        // 需转义的情况, 有 0x7E
        let src: Vec<u8> = vec![0x12, 0x34, 0x56, 0x78, 0x7E, 0x12];
        dst.clear();
        let result: usize = super::wrap(src, &mut dst);
        // println!(">>>>>>> {}", result);
        assert_eq!(result, 9);

        // 需转义的情况, 有 0x7D
        let src: Vec<u8> = vec![0x12, 0x34, 0x56, 0x78, 0x7D, 0x12];
        dst.clear();
        let result: usize = super::wrap(src, &mut dst);
        // println!(">>>>>>> {}", result);
        assert_eq!(result, 9);

        // 7E 和 7D 同时出现的情况
        let src: Vec<u8> = vec![0x7E, 0x34, 0x56, 0x78, 0x7D, 0x12];
        dst.clear();
        let result: usize = super::wrap(src, &mut dst);
        // println!(">>>>>>> {}", result);
        assert_eq!(result, 10);

    }
}