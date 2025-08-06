use crate::hash;
mod misc;

pub struct CRC64;

impl hash::Hasher for CRC64 {
    fn new() -> CRC64 {
        return CRC64 {};
    }

    fn hash(&self, data: &[u8]) -> hash::Hash {
        let mut crc: u64 = 0x0000000000000000;
        for &byte in data {
            let index = ((crc >> 56) as u8 ^ byte) as usize;
            crc = (crc << 8) ^ misc::CRC64_TABLE[index];
        }
    
        let mut result: [u8; 8] = [0u8; 8];
        result[0] = ((crc >> 56) & 0xFF) as u8;
        result[1] = ((crc >> 48) & 0xFF) as u8;
        result[2] = ((crc >> 40) & 0xFF) as u8;
        result[3] = ((crc >> 32) & 0xFF) as u8;
        result[4] = ((crc >> 24) & 0xFF) as u8;
        result[5] = ((crc >> 16) & 0xFF) as u8;
        result[6] = ((crc >> 8) & 0xFF) as u8;
        result[7] = (crc & 0xFF) as u8;
    
        return hash::Hash::from_array(&result);
    }

    fn name(&self) -> String {
        return "crc64".to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hash::Hasher;

    fn get_hasher() -> CRC64 {
        return CRC64::new();
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
        /* https://emn178.github.io/online-tools/crc/ */
        assert_eq!(get_hash_from_string("Hello world"), "2893ce68fbb8374e");
        assert_eq!(get_hash_from_string("Goodbye!"),    "e34844cf2b002839");
        assert_eq!(get_hash_from_string("America8765"), "2869693c107abc49");
        assert_eq!(get_hash_from_string(" "),           "cc7af1ff21c30bde");
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
