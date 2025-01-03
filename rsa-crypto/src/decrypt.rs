use num_bigint::BigInt;

/// Decrypt a ciphertext using RSA private key
pub fn rsa_decrypt(ciphertext: &BigInt, private_key: &(BigInt, BigInt)) -> BigInt {
    let (d, n) = private_key;

    // Ensure the ciphertext is within the valid range
    if ciphertext >= n || ciphertext.is_zero() {
        panic!("Ciphertext is out of range for the modulus");
    }

    // Perform modular exponentiation: m = c^d % n
    modular_exponentiation(ciphertext,d,n)
}
