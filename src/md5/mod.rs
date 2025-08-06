/*
https://github.com/Zunawe/md5-c/blob/main/md5.c
*/

use crate::hash;
mod misc;

struct MD5Context {
    size: u64,
    buffer: [u32; 4],
    input: [u8; 64],
    digest: [u8; 16]
}

impl MD5Context {
    fn new() -> MD5Context {
        return MD5Context {
            size:   0,
            buffer: [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476],
            input:  [0u8; 64],
            digest: [0u8; 16]
        };
    }

    fn process_block(&mut self, block: &[u8]) -> () {
        let mut temp_input = [0u32; 16];
        for j in 0..16 {
            let k = j * 4;
            temp_input[j] = (block[k + 3] as u32) << 24
                          | (block[k + 2] as u32) << 16
                          | (block[k + 1] as u32) << 8
                          | (block[k] as u32);
        }

        misc::md5_step(&mut self.buffer, &temp_input);
    }

    fn update(&mut self, input: &[u8], len: usize) -> () {
        let mut i = 0;
        let mut index = (self.size % 64) as usize;
        self.size += len as u64;
    
        let part_len = 64 - index;
        if len >= part_len {
            self.input[index..index + part_len].copy_from_slice(&input[0..part_len]);
            let block: [u8; 64] = self.input;
            self.process_block(&block);
    
            i = part_len;
            while i + 63 < len {
                self.process_block(&input[i..i + 64]);
                i += 64;
            }
            index = 0;
        }
    
        self.input[index..index + len - i].copy_from_slice(&input[i..len]);
    }

    fn finalize(&mut self) -> () {
        let mut block: [u32; 16] = [0u32; 16];

        let offset = (self.size % 64) as usize;
        let padding_len = if offset < 56 {
            56 - offset
        } else {
            64 + 56 - offset
        };

        self.update(&misc::PADD[..padding_len], padding_len);
        self.size -= padding_len as u64;

        for j in 0..14 {
            let i = j * 4;
            block[j] = (self.input[i + 3] as u32) << 24
                     | (self.input[i + 2] as u32) << 16
                     | (self.input[i + 1] as u32) << 8
                     | (self.input[i]     as u32);
        }

        let size_bits = self.size * 8;
        block[14] = size_bits as u32;
        block[15] = (size_bits >> 32) as u32;

        misc::md5_step(&mut self.buffer, &block);
        for (i, val) in self.buffer.iter().enumerate() {
            self.digest[i * 4 + 0] = (val & 0x000000FF) as u8;
            self.digest[i * 4 + 1] = ((val >> 8) & 0x000000FF) as u8;
            self.digest[i * 4 + 2] = ((val >> 16) & 0x000000FF) as u8;
            self.digest[i * 4 + 3] = ((val >> 24) & 0x000000FF) as u8;
        }
    }
}

pub struct MD5;

impl hash::Hasher for MD5 {
    fn new() -> MD5 {
        return MD5 {};
    }

    fn hash(&self, data: &[u8]) -> hash::Hash {
        let mut context: MD5Context = MD5Context::new();
        context.update(data, data.len());
        context.finalize();
        return hash::Hash::from_array(&context.digest);
    }

    fn name(&self) -> String {
        return "md5".to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hash::Hasher;

    fn get_hasher() -> MD5 {
        return MD5::new();
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
        /* https://www.miraclesalad.com/webtools/md5.php */
        assert_eq!(get_hash_from_string("Hello world"), "3e25960a79dbc69b674cd4ec67a72c62");
        assert_eq!(get_hash_from_string("Goodbye!"),    "1f3f6dc2b268921e89d5d88b202e6ff0");
        assert_eq!(get_hash_from_string("America8765"), "3d6ff88ebb2e7bd6a62940ca4f8659c2");
        assert_eq!(get_hash_from_string(" "),           "7215ee9c7d9dc229d2921a40e899ec5f");
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
