pub static K: [u64; 80] = [
    0x428A2F98D728AE22, 0x7137449123EF65CD, 0xB5C0FBCFEC4D3B2F, 0xE9B5DBA58189DBBC,
    0x3956C25BF348B538, 0x59F111F1B605D019, 0x923F82A4AF194F9B, 0xAB1C5ED5DA6D8118,
    0xD807AA98A3030242, 0x12835B0145706FBE, 0x243185BE4EE4B28C, 0x550C7DC3D5FFB4E2,
    0x72BE5D74F27B896F, 0x80DEB1FE3B1696B1, 0x9BDC06A725C71235, 0xC19BF174CF692694,
    0xE49B69C19EF14AD2, 0xEFBE4786384F25E3, 0x0FC19DC68B8CD5B5, 0x240CA1CC77AC9C65,
    0x2DE92C6F592B0275, 0x4A7484AA6EA6E483, 0x5CB0A9DCBD41FBD4, 0x76F988DA831153B5,
    0x983E5152EE66DFAB, 0xA831C66D2DB43210, 0xB00327C898FB213F, 0xBF597FC7BEEF0EE4,
    0xC6E00BF33DA88FC2, 0xD5A79147930AA725, 0x06CA6351E003826F, 0x142929670A0E6E70,
    0x27B70A8546D22FFC, 0x2E1B21385C26C926, 0x4D2C6DFC5AC42AED, 0x53380D139D95B3DF,
    0x650A73548BAF63DE, 0x766A0ABB3C77B2A8, 0x81C2C92E47EDAEE6, 0x92722C851482353B,
    0xA2BFE8A14CF10364, 0xA81A664BBC423001, 0xC24B8B70D0F89791, 0xC76C51A30654BE30,
    0xD192E819D6EF5218, 0xD69906245565A910, 0xF40E35855771202A, 0x106AA07032BBD1B8,
    0x19A4C116B8D2D0C8, 0x1E376C085141AB53, 0x2748774CDF8EEB99, 0x34B0BCB5E19B48A8,
    0x391C0CB3C5C95A63, 0x4ED8AA4AE3418ACB, 0x5B9CCA4F7763E373, 0x682E6FF3D6B2B8A3,
    0x748F82EE5DEFB2FC, 0x78A5636F43172F60, 0x84C87814A1F0AB72, 0x8CC702081A6439EC,
    0x90BEFFFA23631E28, 0xA4506CEBDE82BDE9, 0xBEF9A3F7B2C67915, 0xC67178F2E372532B,
    0xCA273ECEEA26619C, 0xD186B8C721C0C207, 0xEADA7DD6CDE0EB1E, 0xF57D4F7FEE6ED178,
    0x06F067AA72176FBA, 0x0A637DC5A2C898A6, 0x113F9804BEF90DAE, 0x1B710B35131C471B,
    0x28DB77F523047D84, 0x32CAAB7B40C72493, 0x3C9EBE0A15C9BEBC, 0x431D67C49C100D4C,
    0x4CC5D4BECB3E42B6, 0x597F299CFC657E2A, 0x5FCB6FAB3AD6FAEC, 0x6C44198C4A475817
];

pub static H0: [u64; 8] = [
    0x6A09E667F3BCC908,
    0xBB67AE8584CAA73B,
    0x3C6EF372FE94F82B,
    0xA54FF53A5F1D36F1,
    0x510E527FADE682D1,
    0x9B05688C2B3E6C1F,
    0x1F83D9ABFB41BD6B,
    0x5BE0CD19137E2179,
];

#[inline(always)]
fn rotr(x: u64, num_bits: u64) -> u64 {
    return (x >> num_bits) | (x << (64 - num_bits));
}

#[inline(always)]
pub fn ch(x: u64, y: u64, z: u64) -> u64 {
    return (x & y) ^ ((!x) & z);
}

#[inline(always)]
pub fn maj(x: u64, y: u64, z: u64) -> u64 {
    return (x & y) ^ (x & z) ^ (y & z);
}

#[inline(always)]
pub fn big_sigma0(x: u64) -> u64 {
    return rotr(x, 28) ^ rotr(x, 34) ^ rotr(x, 39);
}

#[inline(always)]
pub fn big_sigma1(x: u64) -> u64 {
    return rotr(x, 14) ^ rotr(x, 18) ^ rotr(x, 41);
}

#[inline(always)]
pub fn small_sigma0(x: u64) -> u64 {
    return rotr(x, 1) ^ rotr(x, 8) ^ (x >> 7);
}

#[inline(always)]
pub fn small_sigma1(x: u64) -> u64 {
    return rotr(x, 19) ^ rotr(x, 61) ^ (x >> 6);
}

pub fn w(n: usize, m: &[u64]) -> Vec<u64> {
    let mut w = vec![0u64; 80];
    let offset = n * 16;

    for i in 0..16 {
        w[i] = m[offset + i];
    }

    for i in 16..80 {
        w[i] = small_sigma1(w[i - 2])
             .wrapping_add(w[i - 7])
             .wrapping_add(small_sigma0(w[i - 15]))
             .wrapping_add(w[i - 16]);
    }

    return w;
}

pub fn u64_slice_to_bytes(data: &[u64]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(data.len() * 8);
    for &word in data {
        bytes.extend_from_slice(&word.to_be_bytes());
    }
    
    return bytes;
}

pub fn bytes_to_u64_blocks(padded: &[u8]) -> Vec<u64> {
    return padded
        .chunks(8)
        .map(|chunk| u64::from_be_bytes(chunk.try_into().unwrap()))
        .collect();
}
