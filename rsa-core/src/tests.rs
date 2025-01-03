#[cfg(test)]
mod tests {
    use crate::arithmetic::{modular_exponentiation, gcd, modular_inverse};
    use crate::utils::{bytes_to_bigint, bigint_to_bytes, random_bigint_in_range};
    use num_bigint::BigInt;

    #[test]
    fn test_modular_exponentiation() {
        let base = BigInt::from(4);
        let exp = BigInt::from(13);
        let modulus = BigInt::from(497);
        let result = modular_exponentiation(&base, &exp, &modulus);
        assert_eq!(result, BigInt::from(445)); 
    }

    #[test]
    fn test_gcd() {
        let a = BigInt::from(56);
        let b = BigInt::from(98);
        let result = gcd(&a, &b);
        assert_eq!(result, BigInt::from(14));
    }

    #[test]
    fn test_modular_inverse() {
        let a = BigInt::from(3);
        let modulus = BigInt::from(26);
        let result = modular_inverse(&a, &modulus);
        assert_eq!(result, Some(BigInt::from(9))); // Precomputed result
    }
        // Unit Tests for Utils Functions
        #[test]
        fn test_bytes_to_bigint() {
            let bytes = &[0x01, 0x00]; // 256 in big endian
            let bigint = bytes_to_bigint(bytes);
            assert_eq!(bigint, BigInt::from(256));
        }
    
        #[test]
        fn test_bigint_to_bytes() {
            let bigint = BigInt::from(256);
            let bytes = bigint_to_bytes(&bigint);
            assert_eq!(bytes, vec![0x01, 0x00]);
        }
    
        #[test]
        fn test_random_bigint_in_range() {
            let lower = BigInt::from(10);
            let upper = BigInt::from(20);
            let random_num = random_bigint_in_range(&lower, &upper);
            assert!(random_num >= lower && random_num < upper);
        }
    
        #[test]
        fn test_random_bigint_in_range_edge_cases() {
            let lower = BigInt::from(0);
            let upper = BigInt::from(1);
            let random_num = random_bigint_in_range(&lower, &upper);
            assert_eq!(random_num, BigInt::from(0));
        }
}
