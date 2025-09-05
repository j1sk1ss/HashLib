use std;

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
        return Hash {
            hashed: true,
            body:   data.as_bytes().to_vec()
        }
    }

    pub fn equals(&self, hash: &Hash) -> bool {
        return self.body == hash.body;
    }

    pub fn cmp(&self, b: &Hash) -> i64 {
        let min_len = self.body.len().min(b.body.len());
        for i in 0..min_len {
            if self.body[i] != b.body[i] {
                return (self.body[i] as i64) - (b.body[i] as i64);
            }
        }
    
        return (self.body.len() as i64) - (b.body.len() as i64);
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

impl PartialEq for Hash {
    fn eq(&self, other: &Self) -> bool {
        self.body == other.body
    }
}

impl Eq for Hash {}

impl std::hash::Hash for Hash {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.body.hash(state);
    }
}

impl std::fmt::Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in &self.body {
            write!(f, "{:02x}", byte)?;
        }

        return Ok(());
    }
}

pub trait Hashable: Clone {
    fn default() -> Self;
    fn get_id(&self) -> usize;
    fn to_bytes(&self) -> Vec<u8>;
}

pub trait Hasher {
    fn new() -> Self where Self: Sized;
    fn hash(&self, data: &[u8]) -> Hash;
    fn name(&self) -> String;
}
