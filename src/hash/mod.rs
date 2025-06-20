pub struct Hash {
    body: Vec<u8>
}

impl Hash {
    pub fn new(size: usize) -> Hash {
        return Hash {
            body: vec![0u8; size]
        };
    }

    pub fn from_array(data: &[u8]) -> Hash {
        return Hash {
            body: data.to_vec()
        };
    }

    pub fn concat(&self, hash: &Hash) -> Hash {
        let mut result: Hash = Hash::new(0);
        result.body.extend(&self.body);
        result.body.extend(&hash.body);
        return result;
    }

    pub fn to_string(&self) -> String {
        return self.body.iter().map(|b| format!("{:02x}", b)).collect::<String>();
    }
}
