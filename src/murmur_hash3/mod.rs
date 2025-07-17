// https://github.com/PeterScott/murmur3/blob/master/murmur3.c
use crate::hash;

pub struct MurMurHash3 {
    seed: u32
}

pub fn murmur3_x86_32(key: &[u8], seed: u32) -> u32 {
    #[inline(always)]
    fn rotl32(x: u32, r: i8) -> u32 {
        (x << r) | (x >> (32 - r))
    }

    #[inline(always)]
    fn fmix32(mut h: u32) -> u32 {
        h ^= h >> 16;
        h = h.wrapping_mul(0x85ebca6b);
        h ^= h >> 13;
        h = h.wrapping_mul(0xc2b2ae35);
        h ^= h >> 16;
        h
    }

    let c1: u32 = 0xcc9e2d51;
    let c2: u32 = 0x1b873593;

    let len = key.len() as u32;
    let nblocks = len / 4;

    let mut h1 = seed;

    for i in 0..nblocks {
        let i = (i * 4) as usize;
        let mut k1 = u32::from_le_bytes([key[i], key[i + 1], key[i + 2], key[i + 3]]);
        k1 = k1.wrapping_mul(c1);
        k1 = rotl32(k1, 15);
        k1 = k1.wrapping_mul(c2);

        h1 ^= k1;
        h1 = rotl32(h1, 13);
        h1 = h1.wrapping_mul(5).wrapping_add(0xe6546b64);
    }

    let tail = &key[(nblocks * 4) as usize..];
    let mut k1 = 0u32;

    match tail.len() {
        3 => {
            k1 ^= (tail[2] as u32) << 16;
            k1 ^= (tail[1] as u32) << 8;
            k1 ^= tail[0] as u32;
            k1 = k1.wrapping_mul(c1);
            k1 = rotl32(k1, 15);
            k1 = k1.wrapping_mul(c2);
            h1 ^= k1;
        }
        2 => {
            k1 ^= (tail[1] as u32) << 8;
            k1 ^= tail[0] as u32;
            k1 = k1.wrapping_mul(c1);
            k1 = rotl32(k1, 15);
            k1 = k1.wrapping_mul(c2);
            h1 ^= k1;
        }
        1 => {
            k1 ^= tail[0] as u32;
            k1 = k1.wrapping_mul(c1);
            k1 = rotl32(k1, 15);
            k1 = k1.wrapping_mul(c2);
            h1 ^= k1;
        }
        _ => {}
    }

    h1 ^= len;
    return fmix32(h1);
}

impl MurMurHash3 {
    fn set_seed(&mut self, seed: u32) -> () {
        self.seed = seed;
    }
}

impl hash::Hasher for MurMurHash3 {
    fn new() -> Self {
        return MurMurHash3 {
            seed: 0u32
        };
    }

    fn hash(&self, input: &[u8]) -> hash::Hash {
        let hashed = murmur3_x86_32(&input, self.seed);
        return hash::Hash::from_array(&hashed.to_be_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hash::Hasher;

    fn get_hasher() -> MurMurHash3 {
        let mut hasher = MurMurHash3::new();
        hasher.set_seed(0);
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
        /* https://emn178.github.io/online-tools/crc/ */
        assert_eq!(get_hash_from_string("Hello world"), "ad91570c");
        assert_eq!(get_hash_from_string("Goodbye!"),    "eadfaad3");
        assert_eq!(get_hash_from_string("America8765"), "41e6d73b");
        assert_eq!(get_hash_from_string(" "),           "7ef49b98");
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