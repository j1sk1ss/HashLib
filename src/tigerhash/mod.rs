mod sboxes;
use crate::hash;

fn round(a: &mut u64, b: &mut u64, c: &mut u64, round_idx: usize, mul: u8, x: &[u64]) -> () {
    let c_bytes = c.to_le_bytes();
    
    *c = c.wrapping_add(x[round_idx]);
    *a = a.wrapping_sub(
        sboxes::T1[c_bytes[0] as usize] 
        ^ sboxes::T2[c_bytes[2] as usize] 
        ^ sboxes::T3[c_bytes[4] as usize] 
        ^ sboxes::T4[c_bytes[6] as usize]
    );

    *b = b.wrapping_add(
        sboxes::T4[c_bytes[1] as usize] 
        ^ sboxes::T3[c_bytes[3] as usize] 
        ^ sboxes::T2[c_bytes[5] as usize] 
        ^ sboxes::T1[c_bytes[7] as usize]
    );
    *b = b.wrapping_mul(mul as u64);
}

fn pass_rounds(a: &mut u64, b: &mut u64, c: &mut u64, mul: u8, x: &[u64]) -> () {
    for i in 0..8 {
        round(a, b, c, i, mul, x);
    }
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

fn pad(data: &[u8]) -> Vec<u8> {
    let mut padded = data.to_vec();
    padded.push(0x01);
    
    let padding_len = (56 - (padded.len() % 64)) % 64;
    padded.extend(std::iter::repeat(0).take(padding_len));
    
    let bit_len = (data.len() as u64).wrapping_mul(8);
    padded.extend_from_slice(&bit_len.to_le_bytes());
    
    return padded;
}

pub fn hash(data: &[u8]) -> hash::Hash {
    let mut a: u64 = 0x0123456789ABCDEF;
    let mut b: u64 = 0xFEDCBA9876543210;
    let mut c: u64 = 0xF096A5B4C3B2E187;

    let mut aa: u64;
    let mut bb: u64;
    let mut cc: u64;

    let value = pad(data);
    let mut x: [u64; 8] = [0u64; 8];

    for chunk in value.chunks_exact(64) {
        for (i, word_bytes) in chunk.chunks_exact(8).enumerate() {
            x[i] = u64::from_le_bytes(word_bytes.try_into().unwrap());
        }

        aa = a;
        bb = b;
        cc = c;

        pass_rounds(&mut a, &mut b, &mut c, 5, &x);
        key_schedule(&mut x);
        
        pass_rounds(&mut c, &mut a, &mut b, 7, &x);
        key_schedule(&mut x);
        
        pass_rounds(&mut b, &mut c, &mut a, 9, &x);        
        feed_forward(&mut a, &mut b, &mut c, aa, bb, cc);
    }

    let mut result_bytes = Vec::with_capacity(24);
    result_bytes.extend_from_slice(&a.to_le_bytes());
    result_bytes.extend_from_slice(&b.to_le_bytes());
    result_bytes.extend_from_slice(&c.to_le_bytes());
    
    return hash::Hash::from_vec(result_bytes);
}