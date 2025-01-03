use num_bigint::BigInt;
use num_traits::{One, Zero};

// TODO: Replace naive modular arithmetic with Montgomery reduction for performance.
// TODO: Explore Barrett reduction for repeated modulo operations.
// TODO: Investigate SIMD-based optimizations for BigInt operations.

/// (base^exp) % modulus for bigint 
pub fn modular_exponentiation(base: &BigInt, exp: &BigInt, modulus: &BigInt) -> BigInt {
    let mut result = BigInt::one();
    let mut base = base % modulus;
    let mut exp = exp.clone();

    while !exp.is_zero() {
        if &exp % 2 == BigInt::one() {
            result = (&result * &base) % modulus;
        }
        exp /= 2;
        base = (&base * &base) % modulus;
    }
    result
}

/// GCD using Euclid's algorithm
#[inline]
pub fn gcd(a: &BigInt, b: &BigInt) -> BigInt {
    if b.is_zero() {
        a.clone()
    } else {
        gcd(b, &(a % b))
    }
}

/// Extended Euclidean Algorithm for modular inverse
pub fn modular_inverse(a: &BigInt, m: &BigInt) -> Option<BigInt> {
    let (g, x, _) = extended_gcd(a, m);
    if g.is_one() {
        Some((x % m + m) % m)
    } else {
        None
    }
}

/// Helper for extended GCD;
/// iterative for speed
fn extended_gcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    let mut old_r = a.clone();
    let mut r = b.clone();
    let mut old_s = BigInt::one();
    let mut s = BigInt::zero();
    let mut old_t = BigInt::zero();
    let mut t = BigInt::one();

    while !r.is_zero() {
        let quotient = &old_r / &r;

        let temp_r = old_r.clone();
        old_r = r.clone();
        r = temp_r - &quotient * r;

        let temp_s = old_s.clone();
        old_s = s.clone();
        s = temp_s - &quotient * s;

        let temp_t = old_t.clone();
        old_t = t.clone();
        t = temp_t - &quotient * t;
    }

    (old_r, old_s, old_t)
}
