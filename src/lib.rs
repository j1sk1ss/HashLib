/* lib.rs */
/* Hash functions */
pub mod md5;
pub mod sha3;
pub mod sha512;
pub mod blake2b;
pub mod ripemd160;
pub mod tigerhash;

/* Main hash abstraction module */
mod hash;
pub use hash::{Hash, Hasher, Hashable};

