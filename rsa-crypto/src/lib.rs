pub mod encrypt;
pub mod decrypt;

pub use encrypt::rsa_encrypt;
pub use decrypt::rsa_decrypt;

#[cfg(test)]
mod tests {
    use super::*;
    use rsa_keygen::generate_rsa_keys;
    use rsa_core::utils::{string_to_bigint, bigint_to_string};

    #[test]
    fn test_rsa_encryption_decryption() {
        // Generate RSA keys
        let keys = generate_rsa_keys(1024, Some(BigInt::from(65537))).unwrap();

        let public_key = keys.public_key;
        let private_key = keys.private_key;

        // Message to encrypt
        let message = "Hello, RSA!";
        let message_bigint = string_to_bigint(message);

        // Encrypt the message
        let ciphertext = rsa_encrypt(&message_bigint, &public_key);

        // Decrypt the ciphertext
        let decrypted_bigint = rsa_decrypt(&ciphertext, &private_key);
        let decrypted_message = bigint_to_string(&decrypted_bigint);

        // Verify the decrypted message matches the original
        assert_eq!(message, decrypted_message);
    }
}
