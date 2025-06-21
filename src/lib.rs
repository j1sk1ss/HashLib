mod hash;
mod sha3;
mod ripemd160;
mod tigerhash;

pub use hash::Hash;
pub use hash::Hasher;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ripemd160_hash() -> () {
        let hash_func: ripemd160::Ripemd160 = ripemd160::Ripemd160::new();
        let hash: hash::Hash = hash_func.hash("Hello world".as_bytes());
        let hex_string: String = hash.to_string();
        println!("RIPEMD-160 hash of 'Hello world': {}", hex_string);
        let expected_hex: &str = "dbea7bd24eef40a2e79387542e36dd408b77b21a";
        assert_eq!(hex_string, expected_hex);
    }

    #[test]
    fn ripemd160_concat() -> () {
        let hash_func: ripemd160::Ripemd160 = ripemd160::Ripemd160::new();
        let fhash: hash::Hash = hash_func.hash("Hello world".as_bytes());
        let shash: hash::Hash = hash_func.hash("Goodbye!".as_bytes());

        let hex_string = fhash.concat(&shash).to_string();
        println!("RIPEMD-160 hash of 'Hello world' + 'Goodbye!': {}", hex_string);

        let expected_hex = "dbea7bd24eef40a2e79387542e36dd408b77b21a190a1a00ee63e9ce4761431ddb8987b220838f80";
        assert_eq!(hex_string, expected_hex);
    }

    #[test]
    fn tigerhash_hash() -> () {
        let hash_func: tigerhash::TigerHash = tigerhash::TigerHash::new();
        let hash: hash::Hash = hash_func.hash("Hello world".as_bytes());
        let hex_string: String = hash.to_string();
        println!("TIGERHASH hash of 'Hello world': {}", hex_string);
        let expected_hex: &str = "1f5d29e51fb59e6247561e19a0e593dac8330180322881c4";
        assert_eq!(hex_string, expected_hex);
    }

    #[test]
    fn tigerhash_concat() -> () {
        let hash_func: tigerhash::TigerHash = tigerhash::TigerHash::new();
        let fhash: hash::Hash = hash_func.hash("Hello world".as_bytes());
        let shash: hash::Hash = hash_func.hash("Goodbye!".as_bytes());

        let hex_string = fhash.concat(&shash).to_string();
        println!("TIGERHASH hash of 'Hello world' + 'Goodbye!': {}", hex_string);

        let expected_hex = "1f5d29e51fb59e6247561e19a0e593dac8330180322881c468538d2d70371675b368769760e4025869e7682f0fdf8302";
        assert_eq!(hex_string, expected_hex);
    }

    #[test]
    fn sha3_hash() -> () {
        let hash_func: sha3::SHA3 = sha3::SHA3::new();
        let hash: hash::Hash = hash_func.hash("Hello world".as_bytes());
        let hex_string: String = hash.to_string();
        println!("SHA3 hash of 'Hello world': {}", hex_string);
        let expected_hex: &str = "369183d3786773cef4e56c7b849e7ef5f742867510b676d6b38f8e38a222d8a2";
        assert_eq!(hex_string, expected_hex);
    }

    #[test]
    fn sha3_concat() -> () {
        let hash_func: sha3::SHA3 = sha3::SHA3::new();
        let fhash: hash::Hash = hash_func.hash("Hello world".as_bytes());
        let shash: hash::Hash = hash_func.hash("Goodbye!".as_bytes());

        let hex_string = fhash.concat(&shash).to_string();
        println!("SHA3 hash of 'Hello world' + 'Goodbye!': {}", hex_string);

        let expected_hex = "369183d3786773cef4e56c7b849e7ef5f742867510b676d6b38f8e38a222d8a2db10b44f8d065b9675bc2f672999e3dbbd6fd1e0bca19bb441258391f2b67e6a";
        assert_eq!(hex_string, expected_hex);
    }
}
