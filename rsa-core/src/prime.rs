use num_bigint::{BigInt, RandBigInt};
use num_traits::{Zero,One};



/// Generate a random prime number of `bit_size` bits
pub fn generate_prime(bit_size: usize) -> BigInt {
    let mut rng = rand::thread_rng();
    loop {
        let candidate = rng.gen_bigint(bit_size as u64);
        if is_prime(&candidate, 40) {
            return candidate;
        }
    }
}

/// Miller-Rabin Primality Test
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

    let (mut d, mut s) = (candidate - BigInt::one(), 0);
    while &d % 2 == BigInt::zero() {
        d /= 2;
        s += 1;
    }

    let mut rng = rand::thread_rng();
    for _ in 0..iterations {
        let a = rng.gen_bigint_range(&BigInt::from(2), &(candidate - BigInt::one()));
        let mut x = super::arithmetic::modular_exponentiation(&a, &d, candidate);
        if x == BigInt::one() || x == candidate - BigInt::one() {
            continue;
        }
        let mut composite = true;
        for _ in 0..s - 1 {
            x = (&x * &x) % candidate;
            if x == candidate - BigInt::one() {
                composite = false;
                break;
            }
        }
        if composite {
            return false;
        }
    }
    true
}
