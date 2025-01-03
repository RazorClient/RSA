// use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use rsa_core::arithmetic::{modular_exponentiation, gcd, modular_inverse};
// use num_bigint::{BigInt, RandBigInt};
// use rand::{thread_rng};

// /// Generates a random BigInt of specified bit size.
// fn generate_random_bigint(bit_size: usize) -> BigInt {
//     let mut rng = thread_rng();
//     rng.gen_bigint(bit_size as u64)
// }

// /// Generates two random BigInts of specified bit size.
// fn generate_two_random_bigints(bit_size: usize) -> (BigInt, BigInt) {
//     let mut rng = thread_rng();
//     (rng.gen_bigint(bit_size as u64), rng.gen_bigint(bit_size as u64))
// }

// pub fn bench_modular_exponentiation(c: &mut Criterion) {
//     // Generate large random numbers for benchmarking
//     let base = generate_random_bigint(1024);    // 1024-bit base
//     let exponent = BigInt::from(65537);          // Common RSA exponent
//     let modulus = generate_random_bigint(1024); // 1024-bit modulus

//     c.bench_function("modular_exponentiation_1024_bit", |b| {
//         b.iter(|| {
//             modular_exponentiation(
//                 black_box(&base),
//                 black_box(&exponent),
//                 black_box(&modulus),
//             )
//         })
//     });
// }

// pub fn bench_gcd(c: &mut Criterion) {
//     // Generate large random numbers for benchmarking
//     let (num_a, num_b) = generate_two_random_bigints(1024); // Two 1024-bit numbers

//     c.bench_function("gcd_1024_bit", |b| {
//         b.iter(|| {
//             let (a_ref, b_ref) = (black_box(&num_a), black_box(&num_b));
//             gcd(a_ref, b_ref)
//         })
//     });
// }


// pub fn bench_modular_inverse(c: &mut Criterion) {
//     // Generate large random numbers for benchmarking
//     let value_a = generate_random_bigint(1024);       // 1024-bit number
//     let modulus = generate_random_bigint(1024); // 1024-bit modulus

//     c.bench_function("modular_inverse_1024_bit", |b| {
//         b.iter(|| {
//             modular_inverse(
//                 black_box(&value_a),
//                 black_box(&modulus),
//             )
//         })
//     });
// }


// criterion_group!(
//     benches,
//     bench_modular_exponentiation,
//     bench_gcd,
//     bench_modular_inverse
// );
// criterion_main!(benches);


use criterion::{criterion_group, criterion_main, Criterion};

pub fn trivial_benchmark(c: &mut Criterion) {
    c.bench_function("trivial", |b| {
        b.iter(|| 42 + 42);
    });
}

criterion_group!(benches, trivial_benchmark);
criterion_main!(benches);

