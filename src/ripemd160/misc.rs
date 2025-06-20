#[inline(always)]
fn f(x: u32, y: u32, z: u32) -> u32 {
    return x ^ y ^ z;
}

#[inline(always)]
fn g(x: u32, y: u32, z: u32) -> u32 {
    return (x & y) | (!x & z);
}

#[inline(always)]
fn h(x: u32, y: u32, z: u32) -> u32 {
    return (x | !y) ^ z;
}

#[inline(always)]
fn i(x: u32, y: u32, z: u32) -> u32 {
    return (x & z) | (y & !z);
}

#[inline(always)]
fn j(x: u32, y: u32, z: u32) -> u32 {
    return x ^ (y | !z);
}

#[inline(always)]
pub fn rotl(x: u32, n: u32) -> u32 {
    return x.rotate_left(n);
}

pub fn left_func(jz: usize, x: u32, y: u32, z: u32) -> u32 {
    match jz {
        0..=15 => f(x, y, z),
        16..=31 => g(x, y, z),
        32..=47 => h(x, y, z),
        48..=63 => i(x, y, z),
        64..=79 => j(x, y, z),
        _ => unreachable!(),
    }
}

pub fn right_func(jz: usize, x: u32, y: u32, z: u32) -> u32 {
    match jz {
        0..=15 => j(x, y, z),
        16..=31 => i(x, y, z),
        32..=47 => h(x, y, z),
        48..=63 => g(x, y, z),
        64..=79 => f(x, y, z),
        _ => unreachable!(),
    }
}
