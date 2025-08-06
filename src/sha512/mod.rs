/*
SHA512 hash function
https://github.com/LeFroid/sha256-512/blob/master/SHA512.c
*/

use crate::hash;
mod misc;

pub struct SHA512;

fn pre_process(msg: &[u8]) -> Vec<u8> {
    if msg.is_empty() {
        return vec![];
    }

    let l = (msg.len() as u128) * 8;
    let k = (896 - ((l + 1) % 1024)) % 1024;
    let total_len_bytes = ((l + 1 + k + 128) / 8) as usize;

    let mut padded = vec![0u8; total_len_bytes];

    padded[..msg.len()].copy_from_slice(msg);
    padded[msg.len()] = 0x80;

    let len_bytes = l.to_be_bytes();
    padded[total_len_bytes - 16..].copy_from_slice(&len_bytes);
    return padded;
}

fn get_hash(padded: &[u8]) -> Vec<u8> {
    let m: Vec<u64> = misc::bytes_to_u64_blocks(padded);
    let n_blocks: usize = m.len() / 16;

    let mut h: [u64; 8] = misc::H0;
    for i in 0..n_blocks {
        let w: Vec<u64> = misc::w(i, &m);
        let mut reg: [u64; 8] = h;

        for j in 0..80 {
            let t1 = reg[7]
                .wrapping_add(misc::big_sigma1(reg[4]))
                .wrapping_add(misc::ch(reg[4], reg[5], reg[6]))
                .wrapping_add(misc::K[j])
                .wrapping_add(w[j]);

            let t2 = misc::big_sigma0(reg[0])
                .wrapping_add(misc::maj(reg[0], reg[1], reg[2]));

            reg = [
                t1.wrapping_add(t2),
                reg[0],
                reg[1],
                reg[2],
                reg[3].wrapping_add(t1),
                reg[4],
                reg[5],
                reg[6],
            ];
        }

        for i in 0..8 {
            h[i] = h[i].wrapping_add(reg[i]);
        }
    }

    return misc::u64_slice_to_bytes(&h);
}

impl hash::Hasher for SHA512 {
    fn new() -> SHA512 {
        return SHA512 {};
    }

    fn hash(&self, data: &[u8]) -> hash::Hash {
        let padded_data = pre_process(data);
        return hash::Hash::from_vec(get_hash(&padded_data));
    }

    fn name(&self) -> String {
        return "sha512".to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hash::Hasher;

    fn get_hasher() -> SHA512 {
        return SHA512::new();
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
        /* https://emn178.github.io/online-tools/sha512.html */
        assert_eq!(get_hash_from_string("Hello world"), "b7f783baed8297f0db917462184ff4f08e69c2d5e5f79a942600f9725f58ce1f29c18139bf80b06c0fff2bdd34738452ecf40c488c22a7e3d80cdf6f9c1c0d47");
        assert_eq!(get_hash_from_string("Goodbye!"),    "36713534fdfd43b53e4211a2181a31ef7d5564bee70ec11a31977d6b1204cec430085308c751e108204d53c70220c1c9a869260604392c4602ab746c73b8e9bc");
        assert_eq!(get_hash_from_string("America8765"), "9111966f02c1acbde401f87e25e7f180a603867e0dbd7bcd476b2ff30b0ed90e3f64881355f8308837611684e6e1dc2f6db9f4b75c9d15ece336e48dcadd7fff");
        assert_eq!(get_hash_from_string(" "),           "f90ddd77e400dfe6a3fcf479b00b1ee29e7015c5bb8cd70f5f15b4886cc339275ff553fc8a053f8ddc7324f45168cffaf81f8c3ac93996f6536eef38e5e40768");
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
