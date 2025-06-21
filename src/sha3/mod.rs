/*
SHA-3, 256bit hash function.
*/

use crate::hash;
mod misc;

struct Sha3ctx {
    state: [[u64; 5]; 5],
    buffer: [u8; 136],
    buff_size: usize
}

impl Sha3ctx {
    fn new() -> Sha3ctx {
        return Sha3ctx {
            state: [[0u64; 5]; 5],
            buffer: [0u8; 136],
            buff_size: 0
        }
    }

    fn absorb_block(&mut self) {
        for i in 0..(136 / 8) {
            let mut word_bytes = [0u8; 8];
            word_bytes.copy_from_slice(&self.buffer[i * 8..(i + 1) * 8]);
            let word = u64::from_le_bytes(word_bytes);
            let x = i % 5;
            let y = i / 5;
            self.state[x][y] ^= word;
        }
    }

    fn absorb(&mut self, mut data: &[u8]) {
        while !data.is_empty() {
            let to_copy = usize::min(136 - self.buff_size, data.len());
            self.buffer[self.buff_size..self.buff_size + to_copy].copy_from_slice(&data[..to_copy]);

            self.buff_size += to_copy;
            data = &data[to_copy..];

            if self.buff_size == 136 {
                self.absorb_block();
                misc::keccak_f(&mut self.state);
                self.buff_size = 0;
            }
        }
    }

    fn finalize(&mut self) -> [u8; 32] {
        self.buffer[self.buff_size] = 0x06;
        for i in (self.buff_size + 1)..135 {
            self.buffer[i] = 0;
        }

        self.buffer[135] |= 0x80;
        self.absorb_block();
        misc::keccak_f(&mut self.state);

        let mut hash: [u8; 32] = [0u8; 32];
        for i in 0..4 {
            hash[i * 8..(i + 1) * 8].copy_from_slice(&self.state[i % 5][i / 5].to_be_bytes());
        }        

        return hash;
    }
}

pub struct SHA3;

impl hash::Hasher for SHA3 {
    fn new() -> SHA3 {
        return SHA3 {};
    }

    fn hash(&self, data: &[u8]) -> hash::Hash {
        let mut ctx: Sha3ctx = Sha3ctx::new();
        ctx.absorb(data);
        return hash::Hash::from_array(&ctx.finalize());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hash::Hasher;

    fn get_hasher() -> SHA3 {
        return SHA3::new();
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
        assert_eq!(get_hash_from_string("Hello world"), "369183d3786773cef4e56c7b849e7ef5f742867510b676d6b38f8e38a222d8a2");
        assert_eq!(get_hash_from_string("Goodbye!"),    "db10b44f8d065b9675bc2f672999e3dbbd6fd1e0bca19bb441258391f2b67e6a");
        assert_eq!(get_hash_from_string("America8765"), "6712b740c03481dfa38d6193fb27896508e1fced868abddaa9ccecbc0f068aaa");
        assert_eq!(get_hash_from_string(" "),           "60e893e6d54d8526e55a81f98bfac5da236bb203e84ed5967a8f527d5bf3d4a4");
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
