/*
TigerHash 192,3 hash function.
Based on Ross Anderson and Eli Biham "Tiger: A Fast New Hash Function".
Also:
https://github.com/rhash/RHash/blob/master/librhash/tiger.c
https://github.com/Kumokage/TigerHash
*/

use crate::hash;
mod sboxes;

fn round(a: &mut u64, b: &mut u64, c: &mut u64, x: u64, mul: u64) -> () {
    *c ^= x;
    *a = a.wrapping_sub(
        sboxes::T1[(*c & 0xFF) as usize] ^ sboxes::T2[((*c >> 16) & 0xFF) as usize] 
        ^ sboxes::T3[((*c >> 32) & 0xFF) as usize] ^ sboxes::T4[((*c >> 48) & 0xFF) as usize]
    );

    *b = b.wrapping_add(
        sboxes::T4[((*c >> 8) & 0xFF) as usize] ^ sboxes::T3[((*c >> 24) & 0xFF) as usize] 
        ^ sboxes::T2[((*c >> 40) & 0xFF) as usize] ^ sboxes::T1[((*c >> 56) & 0xFF) as usize]
    );

    *b = b.wrapping_mul(mul);
}

fn pass(a: &mut u64, b: &mut u64, c: &mut u64, x: &[u64], mul: u8) -> () {
    round(a, b, c, x[0], mul as u64);
    round(b, c, a, x[1], mul as u64);
    round(c, a, b, x[2], mul as u64);
    round(a, b, c, x[3], mul as u64);
    round(b, c, a, x[4], mul as u64);
    round(c, a, b, x[5], mul as u64);
    round(a, b, c, x[6], mul as u64);
    round(b, c, a, x[7], mul as u64);
}

fn key_schedule(x: &mut [u64]) -> () {
    x[0] = x[0].wrapping_sub(x[7] ^ 0xA5A5A5A5A5A5A5A5);
    x[1] ^= x[0];
    x[2] = x[2].wrapping_add(x[1]);
    x[3] = x[3].wrapping_sub(x[2] ^ ((!x[1]) << 19));
    x[4] ^= x[3];
    x[5] = x[5].wrapping_add(x[4]);
    x[6] = x[6].wrapping_sub(x[5] ^ ((!x[4]) >> 23));
    x[7] ^= x[6];
    
    x[0] = x[0].wrapping_add(x[7]);
    x[1] = x[1].wrapping_sub(x[0] ^ ((!x[7]) << 19));
    x[2] ^= x[1];
    x[3] = x[3].wrapping_add(x[2]);
    x[4] = x[4].wrapping_sub(x[3] ^ ((!x[2]) >> 23));
    x[5] ^= x[4];
    x[6] = x[6].wrapping_add(x[5]);
    x[7] = x[7].wrapping_sub(x[6] ^ 0x0123456789ABCDEF);
}

fn feed_forward(a: &mut u64, b: &mut u64, c: &mut u64, aa: u64, bb: u64, cc: u64) -> () {
    *a ^= aa;
    *b = b.wrapping_sub(bb);
    *c = c.wrapping_add(cc);
}

pub struct TigerHash;

impl hash::Hasher for TigerHash {
    fn new() -> TigerHash {
        return TigerHash {};
    }
    
    fn hash(&self, data: &[u8]) -> hash::Hash {
        /* Initialization of tiger context */
        let mut a: u64 = 0x0123456789ABCDEF;
        let mut b: u64 = 0xFEDCBA9876543210;
        let mut c: u64 = 0xF096A5B4C3B2E187;
    
        let mut message: Vec<u8> = data.to_vec();
        let original_bit_len: u64 = (data.len() as u64).wrapping_mul(8);
        
        message.push(0x01);
        
        let current_len: usize = message.len();
        let padding_len: usize = (64 - (current_len + 8) % 64) % 64;
        message.extend(vec![0u8; padding_len]);
        message.extend_from_slice(&original_bit_len.to_le_bytes());
        
        let mut x: [u64; 8] = [0u64; 8];
        for chunk in message.chunks_exact(64) {
            for (i, word_bytes) in chunk.chunks_exact(8).enumerate() {
                x[i] = u64::from_le_bytes(word_bytes.try_into().unwrap());
            }
    
            let aa = a;
            let bb = b;
            let cc = c;
    
            pass(&mut a, &mut b, &mut c, &x, 5);
            key_schedule(&mut x);
            
            pass(&mut c, &mut a, &mut b, &x, 7);
            key_schedule(&mut x);
            
            pass(&mut b, &mut c, &mut a, &x, 9);
            feed_forward(&mut a, &mut b, &mut c, aa, bb, cc);
        }
    
        let mut result_bytes = Vec::with_capacity(24);
        result_bytes.extend_from_slice(&a.to_le_bytes());
        result_bytes.extend_from_slice(&b.to_le_bytes());
        result_bytes.extend_from_slice(&c.to_le_bytes());
        
        return hash::Hash::from_vec(result_bytes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hash::Hasher;

    fn get_hasher() -> TigerHash {
        return TigerHash::new();
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
        assert_eq!(get_hash_from_string("Hello world"), "1f5d29e51fb59e6247561e19a0e593dac8330180322881c4");
        assert_eq!(get_hash_from_string("Goodbye!"),    "68538d2d70371675b368769760e4025869e7682f0fdf8302");
        assert_eq!(get_hash_from_string("America8765"), "c6ed8fc90913aea078ede779163a773180bbb818520ca0be");
        assert_eq!(get_hash_from_string(" "),           "f0aefd02e2ad9fd927a4cba375399f7d8bf42e5a377e9fb1");
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
