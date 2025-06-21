static S: [u32; 64] = [
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22,
    5,  9, 14, 20, 5,  9, 14, 20, 5,  9, 14, 20, 5,  9, 14, 20,
    4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23,
    6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21
];

static K: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee,
    0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be,
    0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa,
    0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
    0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c,
    0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05,
    0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039,
    0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1,
    0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391
];

pub static PADD: [u8; 64] = [
    0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
];

fn f(x: u32, y: u32, z: u32) -> u32 {
    return (x & y) | (!x & z);
}

fn g(x: u32, y: u32, z: u32) -> u32 {
    return (x & z) | (y & !z);
}

fn h(x: u32, y: u32, z: u32) -> u32 {
    return x ^ y ^ z;
}

fn i(x: u32, y: u32, z: u32) -> u32 {
    return y ^ (x | !z);
}

pub fn md5_step(buffer: &mut [u32], input: &[u32]) -> () {
    let mut aa: u32 = buffer[0];
    let mut bb: u32 = buffer[1];
    let mut cc: u32 = buffer[2];
    let mut dd: u32 = buffer[3];

    let mut e: u32;
    let mut j: u32;
    for curr_i in 0..64 {
        match curr_i / 16 {
            0 => {
                e = f(bb, cc, dd);
                j = curr_i;
            }
            1 => {
                e = g(bb, cc, dd);
                j = ((curr_i * 5) + 1) % 16;
            }
            2 => {
                e = h(bb, cc, dd);
                j = ((curr_i * 3) + 5) % 16;
            }
            _ => {
                e = i(bb, cc, dd);
                j = (curr_i * 7) % 16;
            }
        }

        let tmp: u32 = dd;
        dd = cc;
        cc = bb;
        bb = bb.wrapping_add(
            (aa
                .wrapping_add(e)
                .wrapping_add(K[curr_i as usize])
                .wrapping_add(input[j as usize]))
            .rotate_left(S[curr_i as usize])
        );
        
        aa = tmp;
    }

    buffer[0] = buffer[0].wrapping_add(aa);
    buffer[1] = buffer[1].wrapping_add(bb);
    buffer[2] = buffer[2].wrapping_add(cc);
    buffer[3] = buffer[3].wrapping_add(dd);
}