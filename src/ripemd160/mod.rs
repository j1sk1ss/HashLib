use crate::hash;
mod misc;

static K: [u32; 5]  = [ 0x00000000, 0x5A827999, 0x6ED9EBA1, 0x8F1BBCDC, 0xA953FD4E ];
static KP: [u32; 5] = [ 0x50A28BE6, 0x5C4DD124, 0x6D703EF3, 0x7A6D76E9, 0x00000000 ];

static R1: [u32; 80] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 7, 4, 13, 1, 10, 6, 15, 3, 12, 0, 9, 5,
    2, 14, 11, 8, 3, 10, 14, 4, 9, 15, 8, 1, 2, 7, 0, 6, 13, 11, 5, 12, 1, 9, 11, 10, 0, 8, 12, 4,
    13, 3, 7, 15, 14, 5, 6, 2, 4, 0, 5, 9, 7, 12, 2, 10, 14, 1, 3, 8, 11, 6, 15, 13,
];

static R2: [u32; 80] = [
    5, 14, 7, 0, 9, 2, 11, 4, 13, 6, 15, 8, 1, 10, 3, 12, 6, 11, 3, 7, 0, 13, 5, 10, 14, 15, 8, 12,
    4, 9, 1, 2, 15, 5, 1, 3, 7, 14, 6, 9, 11, 8, 12, 2, 10, 0, 4, 13, 8, 6, 4, 1, 3, 11, 15, 0, 5,
    12, 2, 13, 9, 7, 10, 14, 12, 15, 10, 4, 1, 5, 8, 7, 6, 2, 13, 14, 0, 3, 9, 11,
];

static S1: [u32; 80] = [
    11, 14, 15, 12, 5, 8, 7, 9, 11, 13, 14, 15, 6, 7, 9, 8, 7, 6, 8, 13, 11, 9, 7, 15, 7, 12, 15,
    9, 11, 7, 13, 12, 11, 13, 6, 7, 14, 9, 13, 15, 14, 8, 13, 6, 5, 12, 7, 5, 11, 12, 14, 15, 14,
    15, 9, 8, 9, 14, 5, 6, 8, 6, 5, 12, 9, 15, 5, 11, 6, 8, 13, 12, 5, 12, 13, 14, 11, 8, 5, 6,
];

static S2: [u32; 80] = [
    8, 9, 9, 11, 13, 15, 15, 5, 7, 7, 8, 11, 14, 14, 12, 6, 9, 13, 15, 7, 12, 8, 9, 11, 7, 7, 12,
    7, 6, 15, 13, 11, 9, 7, 15, 11, 8, 6, 6, 14, 12, 13, 5, 14, 13, 13, 7, 5, 15, 5, 8, 11, 14, 14,
    6, 14, 6, 9, 12, 9, 12, 5, 15, 8, 8, 5, 12, 9, 12, 5, 14, 6, 8, 13, 6, 5, 15, 13, 11, 11,
];

pub fn hash(msg: &[u8]) -> hash::Hash {
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
                al.wrapping_add(misc::left_func(j, bl, cl, dl)).wrapping_add(w[R1[j] as usize]).wrapping_add(K[j / 16]),
                S1[j]
            )
            .wrapping_add(el);

            al = el;
            el = dl;
            dl = misc::rotl(cl, 10);
            cl = bl;
            bl = tl;

            let tr = misc::rotl(
                ar.wrapping_add(misc::right_func(j, br, cr, dr)).wrapping_add(w[R2[j] as usize]).wrapping_add(KP[j / 16]),
                S2[j]
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
