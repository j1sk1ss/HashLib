mod hash;
mod ripemd160;
mod tigerhash;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ripemd160_hash() -> () {
        let hash: hash::Hash = ripemd160::hash("Hello world".as_bytes());
        let hex_string: String = hash.to_string();
        println!("RIPEMD-160 hash of 'Hello world': {}", hex_string);
        let expected_hex: &str = "dbea7bd24eef40a2e79387542e36dd408b77b21a";
        assert_eq!(hex_string, expected_hex);
    }

    #[test]
    fn ripemd160_concat() -> () {
        let fhash: hash::Hash = ripemd160::hash("Hello world".as_bytes());
        let shash: hash::Hash = ripemd160::hash("Goodbye!".as_bytes());

        let hex_string = fhash.concat(&shash).to_string();
        println!("RIPEMD-160 hash of 'Hello world' + 'Goodbye!': {}", hex_string);

        let expected_hex = "dbea7bd24eef40a2e79387542e36dd408b77b21a190a1a00ee63e9ce4761431ddb8987b220838f80";
        assert_eq!(hex_string, expected_hex);
    }

    #[test]
    fn tigerhash_hash() -> () {
        let hash: hash::Hash = tigerhash::hash("Hello world".as_bytes());
        let hex_string: String = hash.to_string();
        println!("TIGERHASH hash of 'Hello world': {}", hex_string);
        let expected_hex: &str = "b5ded21577b03201f93bf55e4c88112b455f5990423a4253";
        assert_eq!(hex_string, expected_hex);
    }

    #[test]
    fn tigerhash_concat() -> () {
        let fhash: hash::Hash = tigerhash::hash("Hello world".as_bytes());
        let shash: hash::Hash = tigerhash::hash("Goodbye!".as_bytes());

        let hex_string = fhash.concat(&shash).to_string();
        println!("RIPEMD-160 hash of 'Hello world' + 'Goodbye!': {}", hex_string);

        let expected_hex = "b5ded21577b03201f93bf55e4c88112b455f5990423a42532c8a06efc4825b2e53a0940d2def05aebea799ccdfe7ac68";
        assert_eq!(hex_string, expected_hex);
    }
}
