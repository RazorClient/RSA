use num_bigint::{BigInt, RandBigInt};
use num_traits::{Zero,One};
use rand::rngs::OsRng;

use crate::arithmetic::modular_exponentiation;


/// Generate a random prime number of `bit_size` bits
/// bit size should be 2048
pub fn generate_prime(bit_size: usize) -> BigInt {
    //OsRng for Cryptographic Security
    let mut rng = rand::rngs::OsRng;
    loop {
        let candidate = rng.gen_bigint(bit_size as u64);
        //k=40 common choice for 2048 bit numbers
        if is_prime(&candidate, 40) {
            return candidate;
        }
    }
}

/// Miller-Rabin Primality Test
/// todo : chaneg number of rounds based on bit size
pub fn is_prime(candidate: &BigInt, iterations: usize) -> bool {

    if candidate <= &BigInt::from(1) {
        return false;
    }
    if candidate == &BigInt::from(2) {
        return true;
    }
    if candidate % 2 == BigInt::zero() {
        return false;
    }

    // Pre-check divisibility against small primes
    static SMALL_PRIMES: [u32; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    for &prime in &SMALL_PRIMES {
        let big_prime = BigInt::from(prime);
        if candidate == &big_prime {
            return true;
        }
        if candidate % &big_prime == BigInt::zero() {
            return false;
        }
    }

    // Use deterministic bases for numbers <= 2^64
    if candidate.bits() <= 64 {
        let deterministic_bases = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
        for &base in &deterministic_bases {
            let big_base = BigInt::from(base);
            if !miller_rabin_test(candidate, &big_base) {
                return false;
            }
        }
        return true;
    }

    // Perform probabilistic Miller-Rabin test for larger numbers
    let mut rng = OsRng;
    for _ in 0..iterations {
        let a = rng.gen_bigint_range(&BigInt::from(2), &(candidate - BigInt::one()));
        if !miller_rabin_test(candidate, &a) {
            return false;
        }
    }
    true
}

/// one round of the test 
/// a->base
fn miller_rabin_test(candidate: &BigInt, a: &BigInt) -> bool {
    let one = BigInt::one();
    let two = &one + &one;

    let mut d = candidate - &one;
    let mut s = 0;

    // Factor out powers of 2 from d
    while &d % &two == BigInt::zero() {
        d /= &two;
        s += 1;
    }

    let mut x = modular_exponentiation(a, &d, candidate);
    if x == one || x == candidate - &one {
        return true;
    }

    for _ in 0..s - 1 {
        x = (&x * &x) % candidate;
        if x == candidate - &one {
            return true;
        }
    }

    false
}

//todo the bigint stuff maybe clean that 