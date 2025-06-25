#[derive(Clone)]
pub struct Hash {
    hashed: bool,
    body: Vec<u8>
}

impl Hash {
    pub fn new(size: usize) -> Hash {
        return Hash {
            hashed: false,
            body:   vec![0u8; size]
        };
    }

    pub fn from_vec(data: Vec<u8>) -> Hash {
        return Hash {
            hashed: true,
            body:   data
        };
    }

    pub fn from_array(data: &[u8]) -> Hash {
        return Hash {
            hashed: true,
            body:   data.to_vec()
        };
    }

    pub fn from_string(data: &str) -> Hash {
        return Hash::new(data.len());
    }

    pub fn equals(&self, hash: &Hash) -> bool {
        return self.body == hash.body;
    }

    pub fn concat(&self, hash: &Hash) -> Hash {
        let mut result: Hash = Hash::new(0);
        result.body.extend(&self.body);
        result.body.extend(&hash.body);
        return result;
    }

    pub fn to_bytes(&self) -> &[u8] {
        return &self.body;
    }

    pub fn to_string(&self) -> String {
        return self.body.iter().map(|b| format!("{:02x}", b)).collect::<String>();
    }

    pub fn is_hashed(&self) -> bool {
        return self.hashed;
    }
}

pub trait Hashable: Clone {
    fn default() -> Self;
    fn get_id(&self) -> usize;
    fn to_bytes(&self) -> Vec<u8>;
}

pub trait Hasher {
    fn new() -> Self;
    fn hash(&self, data: &[u8]) -> Hash;
}
