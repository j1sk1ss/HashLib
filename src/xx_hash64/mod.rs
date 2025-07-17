// https://github.com/easyaspi314/xxhash-clean/blob/master/xxhash64-ref.c
use crate::hash;

pub struct XXHash64 {
    seed: u64
}

pub fn xxh64(input: &[u8], seed: u64) -> u64 {
    const PRIME64_1: u64 = 0x9E3779B185EBCA87;
    const PRIME64_2: u64 = 0xC2B2AE3D27D4EB4F;
    const PRIME64_3: u64 = 0x165667B19E3779F9;
    const PRIME64_4: u64 = 0x85EBCA77C2B2AE63;
    const PRIME64_5: u64 = 0x27D4EB2F165667C5;

    #[inline(always)]
    fn rotl64(value: u64, amt: u32) -> u64 {
        return (value << (amt % 64)) | (value >> (64 - amt % 64));
    }

    #[inline(always)]
    fn read_u64_le(data: &[u8], offset: usize) -> u64 {
        let slice = &data[offset..offset + 8];
        return u64::from_le_bytes(slice.try_into().unwrap());
    }

    #[inline(always)]
    fn read_u32_le(data: &[u8], offset: usize) -> u32 {
        let slice = &data[offset..offset + 4];
        return u32::from_le_bytes(slice.try_into().unwrap());
    }

    #[inline(always)]
    fn round(acc: u64, input: u64) -> u64 {
        let mut acc = acc.wrapping_add(input.wrapping_mul(PRIME64_2));
        acc = rotl64(acc, 31);
        return acc.wrapping_mul(PRIME64_1);
    }

    #[inline(always)]
    fn merge_round(hash: u64, acc: u64) -> u64 {
        let mut hash = hash ^ round(0, acc);
        hash = hash.wrapping_mul(PRIME64_1);
        hash = hash.wrapping_add(PRIME64_4);
        return hash;
    }

    #[inline(always)]
    fn avalanche(mut hash: u64) -> u64 {
        hash ^= hash >> 33;
        hash = hash.wrapping_mul(PRIME64_2);
        hash ^= hash >> 29;
        hash = hash.wrapping_mul(PRIME64_3);
        hash ^= hash >> 32;
        return hash;
    }

    if input.is_empty() {
        return avalanche(seed.wrapping_add(PRIME64_5));
    }

    let mut hash;
    let mut offset = 0;
    let mut remaining = input.len();

    if remaining >= 32 {
        let mut acc1 = seed.wrapping_add(PRIME64_1).wrapping_add(PRIME64_2);
        let mut acc2 = seed.wrapping_add(PRIME64_2);
        let mut acc3 = seed;
        let mut acc4 = seed.wrapping_sub(PRIME64_1);

        while remaining >= 32 {
            acc1 = round(acc1, read_u64_le(input, offset)); offset += 8;
            acc2 = round(acc2, read_u64_le(input, offset)); offset += 8;
            acc3 = round(acc3, read_u64_le(input, offset)); offset += 8;
            acc4 = round(acc4, read_u64_le(input, offset)); offset += 8;
            remaining -= 32;
        }

        hash = rotl64(acc1, 1)
            .wrapping_add(rotl64(acc2, 7))
            .wrapping_add(rotl64(acc3, 12))
            .wrapping_add(rotl64(acc4, 18));

        hash = merge_round(hash, acc1);
        hash = merge_round(hash, acc2);
        hash = merge_round(hash, acc3);
        hash = merge_round(hash, acc4);
    } else {
        hash = seed.wrapping_add(PRIME64_5);
    }

    hash = hash.wrapping_add(input.len() as u64);

    while remaining >= 8 {
        hash ^= round(0, read_u64_le(input, offset));
        hash = rotl64(hash, 27)
            .wrapping_mul(PRIME64_1)
            .wrapping_add(PRIME64_4);
        offset += 8;
        remaining -= 8;
    }

    if remaining >= 4 {
        hash ^= (read_u32_le(input, offset) as u64).wrapping_mul(PRIME64_1);
        hash = rotl64(hash, 23).wrapping_mul(PRIME64_2).wrapping_add(PRIME64_3);
        offset += 4;
        remaining -= 4;
    }

    while remaining > 0 {
        hash ^= (input[offset] as u64).wrapping_mul(PRIME64_5);
        hash = rotl64(hash, 11).wrapping_mul(PRIME64_1);
        offset += 1;
        remaining -= 1;
    }

    return avalanche(hash);
}

impl XXHash64 {
    fn set_seed(&mut self, seed: u64) -> () {
        self.seed = seed;
    }
}

impl hash::Hasher for XXHash64 {
    fn new() -> Self {
        return XXHash64 {
            seed: 0u64
        };
    }

    fn hash(&self, input: &[u8]) -> hash::Hash {
        let hashed = xxh64(&input, self.seed);
        return hash::Hash::from_array(&hashed.to_be_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hash::Hasher;

    fn get_hasher() -> XXHash64 {
        let mut hasher: XXHash64 = XXHash64::new();
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
        assert_eq!(get_hash_from_string("Hello world"), "c500b0c912b376d8");
        assert_eq!(get_hash_from_string("Goodbye!"),    "6875643fe7d4bb96");
        assert_eq!(get_hash_from_string("America8765"), "e1145f407c5eb962");
        assert_eq!(get_hash_from_string(" "),           "079cf5ceb668638d");
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