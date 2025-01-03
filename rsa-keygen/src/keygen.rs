use num_bigint::BigInt;
use num_traits::One;
use rsa_core::arithmetic::{gcd, modular_inverse};
use rsa_core::prime::generate_prime;


pub struct RSAKeys {
    pub public_key: (BigInt, BigInt),  // (e, n)
    pub private_key: (BigInt, BigInt), // (d, n)
}

/// Enum for key generation errors
#[derive(Debug)]
pub enum RSAKeyGenError {
    InvalidKeySize,
    PrimeGenerationError,
    InvalidExponent,
    ModularInverseError,
}


fn validate_key_size(bit_size: usize) -> Result<(), RSAKeyGenError> {
    if bit_size < 512 {
        return Err(RSAKeyGenError::InvalidKeySize);
    }
    Ok(())
}


fn generate_distinct_primes(bit_size: usize) -> Result<(BigInt, BigInt), RSAKeyGenError> {
    for _ in 0..100 { // Retry logic up to 100 times
        let p = generate_prime(bit_size);
        let q = generate_prime(bit_size);
        if p != q {
            return Ok((p, q));
        }
    }
    Err(RSAKeyGenError::PrimeGenerationError)
}


fn choose_public_exponent(
    phi_n: &BigInt,
    default_exponent: Option<BigInt>,
) -> Result<BigInt, RSAKeyGenError> {
    let e = default_exponent.unwrap_or_else(|| BigInt::from(65537));
    if gcd(&e, phi_n) != BigInt::one() {
        return Err(RSAKeyGenError::InvalidExponent);
    }
    Ok(e)
}

/// Generates RSA keys
pub fn generate_rsa_keys(
    bit_size: usize,
    public_exponent: Option<BigInt>,
) -> Result<RSAKeys, RSAKeyGenError> {
    
    // Step 1: Validate the key size
    validate_key_size(bit_size)?;

    // Step 2: Generate two distinct primes, p and q
    let (p, q) = generate_distinct_primes(bit_size / 2)?;

    // Step 3: Compute n = p * q and φ(n) = (p - 1) * (q - 1)
    let n = &p * &q;
    let phi_n = (&p - 1) * (&q - 1);

    // Step 4: Choose public exponent e
    let e = choose_public_exponent(&phi_n, public_exponent)?;

    // Step 5: Compute private exponent d (modular inverse of e mod φ(n))
    let d = modular_inverse(&e, &phi_n).ok_or(RSAKeyGenError::ModularInverseError)?;

    // Step 6: Return the public and private keys
    Ok(RSAKeys {
        public_key: (e, n.clone()),
        private_key: (d, n),
    })
}