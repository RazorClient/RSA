use num_bigint::{BigInt, RandBigInt};
use std::str::FromStr;

/// Convert a byte slice to a BigInt
pub fn bytes_to_bigint(bytes: &[u8]) -> BigInt {
    BigInt::from_bytes_be(num_bigint::Sign::Plus, bytes)
}

/// Convert a BigInt to a byte vector
pub fn bigint_to_bytes(value: &BigInt) -> Vec<u8> {
    value.to_bytes_be().1
}

/// Generate a random BigInt within a range
pub fn random_bigint_in_range(lower: &BigInt, upper: &BigInt) -> BigInt {
    let mut rng = rand::thread_rng();
    rng.gen_bigint_range(lower, upper)
}
//beaks 

pub fn string_to_bigint(input: &str) -> Option<BigInt> {
    BigInt::from_str(input).ok()
}
//breaks

pub fn bigint_to_string(bigint: &BigInt) -> String {
    bigint.to_string()
}