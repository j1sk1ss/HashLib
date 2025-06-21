use crate::hash;
mod misc;

struct Sha3ctx {
    state: [[u64; 5]; 5],
    buffer: [u8; 136],
    buff_size: usize
}

impl Sha3ctx {
    fn new() -> Sha3ctx {
        return Sha3ctx {
            state: [[0u64; 5]; 5],
            buffer: [0u8; 136],
            buff_size: 0
        }
    }

    fn absorb_block(&mut self) {
        for i in 0..(136 / 8) {
            let mut word_bytes = [0u8; 8];
            word_bytes.copy_from_slice(&self.buffer[i * 8..(i + 1) * 8]);
            let word = u64::from_le_bytes(word_bytes);
            let x = i % 5;
            let y = i / 5;
            self.state[x][y] ^= word;
        }
    }

    fn absorb(&mut self, mut data: &[u8]) {
        while !data.is_empty() {
            let to_copy = usize::min(136 - self.buff_size, data.len());
            self.buffer[self.buff_size..self.buff_size + to_copy].copy_from_slice(&data[..to_copy]);

            self.buff_size += to_copy;
            data = &data[to_copy..];

            if self.buff_size == 136 {
                self.absorb_block();
                misc::keccak_f(&mut self.state);
                self.buff_size = 0;
            }
        }
    }

    fn finalize(&mut self) -> [u8; 32] {
        self.buffer[self.buff_size] = 0x06;
        for i in (self.buff_size + 1)..135 {
            self.buffer[i] = 0;
        }

        self.buffer[135] |= 0x80;
        self.absorb_block();
        misc::keccak_f(&mut self.state);

        let mut hash: [u8; 32] = [0u8; 32];
        for i in 0..4 {
            hash[i * 8..(i + 1) * 8].copy_from_slice(&self.state[i % 5][i / 5].to_be_bytes());
        }        

        return hash;
    }
}

pub struct SHA3;

impl hash::Hasher for SHA3 {
    fn new() -> SHA3 {
        return SHA3 {};
    }

    fn hash(&self, data: &[u8]) -> hash::Hash {
        let mut ctx: Sha3ctx = Sha3ctx::new();
        ctx.absorb(data);
        return hash::Hash::from_array(&ctx.finalize());
    }
}
