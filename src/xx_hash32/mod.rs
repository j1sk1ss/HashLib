// https://github.com/easyaspi314/xxhash-clean/blob/master/xxhash32-ref.c
use crate::hash;

pub struct XXHash32 {
    seed: u32
}

pub fn xxh32(input: &[u8], seed: u32) -> u32 {
    const PRIME32_1: u32 = 0x9E3779B1u32;
    const PRIME32_2: u32 = 0x85EBCA77u32;
    const PRIME32_3: u32 = 0xC2B2AE3Du32;
    const PRIME32_4: u32 = 0x27D4EB2Fu32;
    const PRIME32_5: u32 = 0x165667B1u32;

    #[inline(always)]
    fn rotl32(value: u32, amt: u32) -> u32 {
        return (value << (amt % 32)) | (value >> (32 - (amt % 32)));
    }

    #[inline(always)]
    fn read_u32_le(data: &[u8], offset: usize) -> u32 {
        let slice = &data[offset..offset + 4];
        return u32::from_le_bytes(slice.try_into().unwrap());
    }

    #[inline(always)]
    fn round(acc: u32, input: u32) -> u32 {
        let mut acc = acc.wrapping_add(input.wrapping_mul(PRIME32_2));
        acc = rotl32(acc, 13);
        return acc.wrapping_mul(PRIME32_1);
    }

    #[inline(always)]
    fn avalanche(mut hash: u32) -> u32 {
        hash ^= hash >> 15;
        hash = hash.wrapping_mul(PRIME32_2);
        hash ^= hash >> 13;
        hash = hash.wrapping_mul(PRIME32_3);
        hash ^= hash >> 16;
        return hash;
    }

    // In Rust slice cannot be null; for empty slice return same as reference impl:
    if input.is_empty() {
        return avalanche(seed.wrapping_add(PRIME32_5));
    }

    let mut hash: u32;
    let mut offset: usize = 0;
    let mut remaining = input.len();

    if remaining >= 16 {
        let mut acc1 = seed.wrapping_add(PRIME32_1).wrapping_add(PRIME32_2);
        let mut acc2 = seed.wrapping_add(PRIME32_2);
        let mut acc3 = seed;
        let mut acc4 = seed.wrapping_sub(PRIME32_1);

        while remaining >= 16 {
            acc1 = round(acc1, read_u32_le(input, offset));
            offset += 4;
            acc2 = round(acc2, read_u32_le(input, offset));
            offset += 4;
            acc3 = round(acc3, read_u32_le(input, offset));
            offset += 4;
            acc4 = round(acc4, read_u32_le(input, offset));
            offset += 4;
            remaining -= 16;
        }

        hash = rotl32(acc1, 1)
            .wrapping_add(rotl32(acc2, 7))
            .wrapping_add(rotl32(acc3, 12))
            .wrapping_add(rotl32(acc4, 18));
    } 
    else {
        hash = seed.wrapping_add(PRIME32_5);
    }

    hash = hash.wrapping_add(input.len() as u32);

    while remaining >= 4 {
        hash = hash.wrapping_add(read_u32_le(input, offset).wrapping_mul(PRIME32_3));
        hash = rotl32(hash, 17).wrapping_mul(PRIME32_4);
        offset += 4;
        remaining -= 4;
    }

    while remaining > 0 {
        hash = hash.wrapping_add((input[offset] as u32).wrapping_mul(PRIME32_5));
        hash = rotl32(hash, 11).wrapping_mul(PRIME32_1);
        offset += 1;
        remaining -= 1;
    }

    return avalanche(hash);
}

impl XXHash32 {
    fn set_seed(&mut self, seed: u32) -> () {
        self.seed = seed;
    }
}

impl hash::Hasher for XXHash32 {
    fn new() -> Self {
        return XXHash32 {
            seed: 0u32
        };
    }

    fn hash(&self, input: &[u8]) -> hash::Hash {
        let hashed = xxh32(&input, self.seed);
        return hash::Hash::from_array(&hashed.to_be_bytes());
    }

    fn name(&self) -> String {
        return "xxhash32".to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hash::Hasher;

    fn get_hasher() -> XXHash32 {
        let mut hasher: XXHash32 = XXHash32::new();
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
        /* https://asecuritysite.com/encryption/xxhash */
        assert_eq!(get_hash_from_string("Hello world"), "9705d437");
        assert_eq!(get_hash_from_string("Goodbye!"),    "8b23bd40");
        assert_eq!(get_hash_from_string("America8765"), "08302631");
        assert_eq!(get_hash_from_string(" "),           "072e1494");
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