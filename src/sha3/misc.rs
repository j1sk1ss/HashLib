static KECCAK_F_ROUNDS: usize = 24;

static KECCAK_ROUND_CONSTANTS: [u64; KECCAK_F_ROUNDS] = [
    0x0000000000000001, 0x0000000000008082,
    0x800000000000808A, 0x8000000080008000,
    0x000000000000808B, 0x0000000080000001,
    0x8000000080008081, 0x8000000000008009,
    0x000000000000008A, 0x0000000000000088,
    0x0000000080008009, 0x000000008000000A,
    0x000000008000808B, 0x800000000000008B,
    0x8000000000008089, 0x8000000000008003,
    0x8000000000008002, 0x8000000000000080,
    0x000000000000800A, 0x800000008000000A,
    0x8000000080008081, 0x8000000000008080,
    0x0000000080000001, 0x8000000080008008,
];

static RHO_OFFSETS: [[u32; 5]; 5] = [
    [0, 36, 3, 41, 18],
    [1, 44, 10, 45, 2],
    [62, 6, 43, 15, 61],
    [28, 55, 25, 21, 56],
    [27, 20, 39, 8, 14],
];

pub fn keccak_f(state: &mut [[u64; 5]; 5]) -> () {
    for round in 0..KECCAK_F_ROUNDS {
        let mut c = [0; 5];
        for x in 0..5 {
            c[x] = state[x][0] ^ state[x][1] ^ state[x][2] ^ state[x][3] ^ state[x][4];
        }
        
        let mut d = [0; 5];
        for x in 0..5 {
            d[x] = c[(x + 4) % 5] ^ c[(x + 1) % 5].rotate_left(1);
        }
        
        for x in 0..5 {
            for y in 0..5 {
                state[x][y] ^= d[x];
            }
        }

        let mut new_state = [[0; 5]; 5];
        for x in 0..5 {
            for y in 0..5 {
                let new_x = y;
                let new_y = (2 * x + 3 * y) % 5;
                new_state[new_x][new_y] = state[x][y].rotate_left(RHO_OFFSETS[x][y]);
            }
        }

        *state = new_state;

        let mut temp_state = [[0; 5]; 5];
        for x in 0..5 {
            for y in 0..5 {
                temp_state[x][y] = state[x][y] ^ ((!state[(x + 1) % 5][y]) & state[(x + 2) % 5][y]);
            }
        }

        *state = temp_state;
        state[0][0] ^= KECCAK_ROUND_CONSTANTS[round];
    }
}
