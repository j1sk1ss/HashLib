use crate::hash;
mod misc;

pub struct Ripemd160;

impl hash::Hasher for Ripemd160 {
    fn new() -> Ripemd160 {
        return Ripemd160 { };
    }

    fn hash(&self, msg: &[u8]) -> hash::Hash {
        let mut h: [u32; 5] = [ 0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0 ];
        let mut padded = msg.to_vec();
        padded.push(0x80);
    
        while (padded.len() % 64) != 56 {
            padded.push(0);
        }
    
        let bit_len = (msg.len() as u64) * 8;
        padded.extend(&bit_len.to_le_bytes());
    
        for chunk in padded.chunks(64) {
            let mut w: [u32; 16] = [0u32; 16];
            for i in 0..16 {
                w[i] = u32::from_le_bytes([
                    chunk[4 * i],
                    chunk[4 * i + 1],
                    chunk[4 * i + 2],
                    chunk[4 * i + 3],
                ]);
            }
    
            let mut al = h[0];
            let mut bl = h[1];
            let mut cl = h[2];
            let mut dl = h[3];
            let mut el = h[4];
    
            let mut ar = h[0];
            let mut br = h[1];
            let mut cr = h[2];
            let mut dr = h[3];
            let mut er = h[4];
    
            for j in 0..80 {
                let tl = misc::rotl(
                    al.wrapping_add(misc::left_func(j, bl, cl, dl)).wrapping_add(w[misc::R1[j] as usize]).wrapping_add(misc::K[j / 16]),
                    misc::S1[j]
                )
                .wrapping_add(el);
    
                al = el;
                el = dl;
                dl = misc::rotl(cl, 10);
                cl = bl;
                bl = tl;
    
                let tr = misc::rotl(
                    ar.wrapping_add(misc::right_func(j, br, cr, dr)).wrapping_add(w[misc::R2[j] as usize]).wrapping_add(misc::KP[j / 16]),
                    misc::S2[j]
                )
                .wrapping_add(er);
    
                ar = er;
                er = dr;
                dr = misc::rotl(cr, 10);
                cr = br;
                br = tr;
            }
    
            let temp = h[1].wrapping_add(cl).wrapping_add(dr);
            h[1] = h[2].wrapping_add(dl).wrapping_add(er);
            h[2] = h[3].wrapping_add(el).wrapping_add(ar);
            h[3] = h[4].wrapping_add(al).wrapping_add(br);
            h[4] = h[0].wrapping_add(bl).wrapping_add(cr);
            h[0] = temp;
        }
    
        let mut result = [0u8; 20];
        for (i, &val) in h.iter().enumerate() {
            result[i * 4..(i + 1) * 4].copy_from_slice(&val.to_le_bytes());
        }
    
        return hash::Hash::from_array(&result);
    }

    fn name(&self) -> String {
        return "ripemd160".to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hash::Hasher;

    fn get_hasher() -> Ripemd160 {
        return Ripemd160::new();
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
        /* https://emn178.github.io/online-tools/ripemd-160/ */
        assert_eq!(get_hash_from_string("Hello world"), "dbea7bd24eef40a2e79387542e36dd408b77b21a");
        assert_eq!(get_hash_from_string("Goodbye!"),    "190a1a00ee63e9ce4761431ddb8987b220838f80");
        assert_eq!(get_hash_from_string("America8765"), "5adc24ec117a842a7c3b32952f7e0cf01f7d3ed2");
        assert_eq!(get_hash_from_string(" "),           "ac53a3aea6835b5ec12054e12d41d392e9d57b72");
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
