/*
Blake2b hash function with 128bit size of output.
https://github.com/franziskuskiefer/Blake2b/
*/

use crate::hash;
mod misc;

#[repr(C)]
struct Blake2bParam {
    digest_length: u8,
    key_length: u8,
    fanout: u8,
    depth: u8,
    leaf_length: [u8; 4],
    node_offset: [u8; 8],
    node_depth: u8,
    inner_length: u8,
    reserved: [u8; 14],
    salt: [u8; 16],
    personal: [u8; 16],
}

impl Default for Blake2bParam {
    fn default() -> Blake2bParam {
        return Blake2bParam {
            digest_length: 0,
            key_length: 0,
            fanout: 1,
            depth: 1,
            leaf_length: [0; 4],
            node_offset: [0; 8],
            node_depth: 0,
            inner_length: 0,
            reserved: [0; 14],
            salt: [0; 16],
            personal: [0; 16],
        };
    }
}

struct Blake2bContext {
    h: [u64; 8],
    t: [u64; 2],
    f: [u64; 2],
    buffer: [u8; misc::BLAKE2B_BLOCKBYTES],
    buff_size: usize,
    out_size: usize
}

impl Blake2bContext {
    fn new() -> Blake2bContext {
        return Blake2bContext {
            h: [0u64; 8],
            t: [0u64; 2],
            f: [0u64; 2],
            buffer: [0u8; misc::BLAKE2B_BLOCKBYTES],
            buff_size: 0usize,
            out_size: 0usize
        };
    }

    fn increment_counter(&mut self, inc: u64) -> () {
        self.t[0] = self.t[0].wrapping_add(inc);
        self.t[1] = self.t[1].wrapping_add((self.t[0] < inc) as u64);
    }

    fn compress(&mut self, block: &[u8; 128]) -> () {
        let mut v: [u64; 16] = [0u64; 16];
        let mut m: [u64; 16] = [0u64; 16];
        for i in 0..16 {
            m[i] = misc::load64(&block[i * 8..]);
        }

        for i in 0..8 {
            v[i] = self.h[i];
            v[i + 8] = misc::BLAKE2B_IV[i];
        }

        v[12] ^= self.t[0];
        v[13] ^= self.t[1];
        v[14] ^= self.f[0];
        v[15] ^= self.f[1];

        for i in 0..12 {
            let s: [usize; 16] = misc::BLAKE2B_SIGMA[i];
            misc::G(&mut v, 0, 4, 8, 12, m[s[0]], m[s[1]]);
            misc::G(&mut v, 1, 5, 9, 13, m[s[2]], m[s[3]]);
            misc::G(&mut v, 2, 6, 10, 14, m[s[4]], m[s[5]]);
            misc::G(&mut v, 3, 7, 11, 15, m[s[6]], m[s[7]]);
            misc::G(&mut v, 0, 5, 10, 15, m[s[8]], m[s[9]]);
            misc::G(&mut v, 1, 6, 11, 12, m[s[10]], m[s[11]]);
            misc::G(&mut v, 2, 7, 8, 13, m[s[12]], m[s[13]]);
            misc::G(&mut v, 3, 4, 9, 14, m[s[14]], m[s[15]]);
        }

        for i in 0..8 {
            self.h[i] ^= v[i] ^ v[i + 8];
        }
    }

    fn init(&mut self, key: Option<&[u8]>) -> i32 {
        let mut param = Blake2bParam::default();
        param.digest_length = misc::BLAKE2B_OUTBYTES as u8;
        param.key_length = key.map_or(0, |k| k.len() as u8);
    
        let p_bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(
                (&param as *const Blake2bParam) as *const u8,
                std::mem::size_of::<Blake2bParam>(),
            )
        };
    
        self.h.copy_from_slice(&misc::BLAKE2B_IV);
        for i in 0..8 {
            let val = misc::load64(&p_bytes[i * 8..(i + 1) * 8]);
            self.h[i] ^= val;
        }
    
        self.out_size  = misc::BLAKE2B_OUTBYTES;
        self.buff_size = 0;
        self.t = [0; 2];
        self.f = [0; 2];
        self.buffer = [0; misc::BLAKE2B_BLOCKBYTES];
    
        if let Some(k) = key {
            if k.len() > misc::BLAKE2B_BLOCKBYTES {
                return 0;
            }

            let mut block = [0u8; misc::BLAKE2B_BLOCKBYTES];
            block[..k.len()].copy_from_slice(k);
            self.update(&block);
        }
    
        return 1;
    }

    fn update(&mut self, input: &[u8]) -> () {
        let mut inbuf: &[u8] = input;
        while inbuf.len() > misc::BLAKE2B_BLOCKBYTES {
            self.increment_counter(misc::BLAKE2B_BLOCKBYTES as u64);
            let block: &[u8; 128] = inbuf[0..misc::BLAKE2B_BLOCKBYTES].try_into().unwrap();
            self.compress(block);
            inbuf = &inbuf[misc::BLAKE2B_BLOCKBYTES..];
        }

        let len = inbuf.len();
        self.buffer[self.buff_size..self.buff_size + len].copy_from_slice(inbuf);
        self.buff_size += len;
    }

    fn finalize(&mut self) -> Vec<u8> {
        let mut buffer: [u8; misc::BLAKE2B_OUTBYTES] = [0u8; misc::BLAKE2B_OUTBYTES];

        self.increment_counter(self.buff_size as u64);
        self.f[0] = u64::MAX;

        for i in self.buff_size..misc::BLAKE2B_BLOCKBYTES {
            self.buffer[i] = 0;
        }

        let self_buffer = self.buffer;
        self.compress(&self_buffer);
        for (i, &w) in self.h.iter().enumerate() {
            misc::store64(&mut buffer[i * 8..(i + 1) * 8], w);
        }
        
        return buffer[..self.out_size].to_vec();
    }
}

pub struct Blake2B {
    key: String
}

impl Blake2B {
    pub fn set_key(&mut self, key: String) -> () {
        self.key = key;
    }
}

impl hash::Hasher for Blake2B {
    fn new() -> Blake2B {
        return Blake2B {
            key: "".to_string()
        };
    }

    fn hash(&self, data: &[u8]) -> hash::Hash {
        let mut context: Blake2bContext = Blake2bContext::new();
        context.init(Some(self.key.as_bytes()));
        context.update(data);
        return hash::Hash::from_vec(context.finalize());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hash::Hasher;

    fn get_hasher() -> Blake2B {
        let mut hasher: Blake2B = Blake2B::new();
        hasher.set_key("Key12345678".to_string());
        return hasher;
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
        /* https://emn178.github.io/online-tools/blake2b/ */
        assert_eq!(get_hash_from_string("Hello world"), "ff734a0b6c5d9e0f3900c2422d8cc5e1");
        assert_eq!(get_hash_from_string("Goodbye!"),    "b8bb05d29e50931c0cc70d5b152185d1");
        assert_eq!(get_hash_from_string("America8765"), "483d2661243edbadc5859ef272421f00");
        assert_eq!(get_hash_from_string(" "),           "aaa23663da8d05fecd3b0dd39ceaf26e");
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
