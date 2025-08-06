use crate::hash;
mod misc;

pub struct CRC32;

impl hash::Hasher for CRC32 {
    fn new() -> CRC32 {
        return CRC32 {};
    }

    fn hash(&self, data: &[u8]) -> hash::Hash {
        let mut crc: u32 = 0xFFFFFFFF;
        for &byte in data {
            let index: usize = ((crc ^ byte as u32) & 0xFF) as usize;
            crc = (crc >> 8) ^ misc::CRC32_TABLE[index];
        }

        let final_crc: u32 = crc ^ 0xFFFFFFFF;
        let mut result: [u8; 4] = [0u8; 4];
        result[0] = ((final_crc >> 24) & 0xFF) as u8;
        result[1] = ((final_crc >> 16) & 0xFF) as u8;
        result[2] = ((final_crc >> 8) & 0xFF) as u8;
        result[3] = (final_crc & 0xFF) as u8;
        return hash::Hash::from_array(&result);
    }

    fn name(&self) -> String {
        return "crc32".to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hash::Hasher;

    fn get_hasher() -> CRC32 {
        return CRC32::new();
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
        assert_eq!(get_hash_from_string("Hello world"), "8bd69e52");
        assert_eq!(get_hash_from_string("Goodbye!"),    "3078a778");
        assert_eq!(get_hash_from_string("America8765"), "5fd1f637");
        assert_eq!(get_hash_from_string(" "),           "e96ccf45");
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
