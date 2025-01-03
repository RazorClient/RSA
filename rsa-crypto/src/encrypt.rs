use num_bigint::BigInt;

/// Encrypt a message using RSA public key
pub fn rsa_encrypt(message: &BigInt, public_key: &(BigInt, BigInt)) -> BigInt {
    let (e, n) = public_key;

    // Ensure the message is within the valid range
    if message >= n || message.is_zero() {
        panic!("Message is out of range for the modulus");
    }

    // Perform modular exponentiation: c = m^e % n
    modular_exponentiation(message, e, n)
}
