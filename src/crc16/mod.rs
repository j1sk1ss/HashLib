/* CRC-16/IBM-3740 */
use crate::hash;

pub struct CRC16;

impl hash::Hasher for CRC16 {
    fn new() -> CRC16 {
        return CRC16 {};
    }

    fn hash(&self, data: &[u8]) -> hash::Hash {
        let mut crc: u16 = 0xFFFF;
        let poly: u16 = 0x1021;

        for &byte in data {
            crc ^= (byte as u16) << 8;
            for _ in 0..8 {
                if (crc & 0x8000) != 0 {
                    crc = (crc << 1) ^ poly;
                } 
                else {
                    crc <<= 1;
                }
            }
        }

        let mut result: [u8; 2] = [0u8; 2];
        result[0] = (crc >> 8) as u8;
        result[1] = (crc & 0xFF) as u8;
        return hash::Hash::from_array(&result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hash::Hasher;

    fn get_hasher() -> CRC16 {
        return CRC16::new();
    }

    fn get_hash_from_string(msg: &str) -> String {
        return get_hasher().hash(msg.as_bytes()).to_string();
    }
    
    fn get_hash_from_u128(data: u128) -> String {
        return get_hasher().hash(&data.to_le_bytes()).to_string();
    }
    
    fn get_hash_from_u8arr(data: &[u8]) -> String {
        return get_hasher().hash(data).to_string();
    }    

    #[test]
    fn string_tests() -> () {
        /* https://crccalc.com/?crc=Hello%20world&method=&datatype=ascii&outtype=hex  */
        assert_eq!(get_hash_from_string("Hello world"), "4591");
        assert_eq!(get_hash_from_string("Goodbye!"),    "017b");
        assert_eq!(get_hash_from_string("America8765"), "a185");
        assert_eq!(get_hash_from_string(" "),           "c592");
    }

    #[test]
    fn u128_test() -> () {
        assert_eq!(get_hash_from_u128(98234892934), ""); /* TODO */
        assert_eq!(get_hash_from_u128(94304995884), "");
        assert_eq!(get_hash_from_u128(0),           "");
    }

    #[test]
    fn u8arr_tests() -> () {
        assert_eq!(get_hash_from_u8arr(&[0, 1, 2, 98, 74]),  "");  /* TODO */
        assert_eq!(get_hash_from_u8arr(&[8, 92, 0xA]),       "");
        assert_eq!(get_hash_from_u8arr(&[0, 0, 0, 0, 0, 0]), "");
    }
}

