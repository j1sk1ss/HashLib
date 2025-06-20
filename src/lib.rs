mod hash;
mod ripemd160;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world_hash() {
        let hash: hash::Hash = ripemd160::hash("Hello world".as_bytes());
        let hex_string = hash.to_string();

        println!("RIPEMD-160 hash of 'Hello world': {}", hex_string);

        let expected_hex = "dbea7bd24eef40a2e79387542e36dd408b77b21a";
        assert_eq!(hex_string, expected_hex);
    }

    #[test]
    fn hash_concat() {
        let fhash: hash::Hash = ripemd160::hash("Hello world".as_bytes());
        let shash: hash::Hash = ripemd160::hash("Goodbye!".as_bytes());

        let hex_string = fhash.concat(&shash).to_string();
        println!("RIPEMD-160 hash of 'Hello world' + 'Goodbye!': {}", hex_string);

        let expected_hex = "dbea7bd24eef40a2e79387542e36dd408b77b21a190a1a00ee63e9ce4761431ddb8987b220838f80";
        assert_eq!(hex_string, expected_hex);
    }
}
