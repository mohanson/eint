#![no_main]
mod utils;
use eint::Eint;
use libfuzzer_sys::fuzz_target;
use utils::e2562array;
use utils::{gen_eint256_pair, gen_uint256_pair};

fn fuzz_mul(data: &[u8]) {
    let (a, b) = gen_uint256_pair(data);
    let (a2, b2) = gen_eint256_pair(data);

    let (r, _) = a.overflowing_mul(b);
    let r2 = a2.wrapping_mul(b2);

    let mut bin = [0u8; 32];
    r.to_little_endian(&mut bin);
    let bin2 = e2562array(r2);

    assert_eq!(bin, bin2);
}

fn fuzz_add(data: &[u8]) {
    let (a, b) = gen_uint256_pair(data);
    let (a2, b2) = gen_eint256_pair(data);

    let (r, _) = a.overflowing_add(b);
    let r2 = a2.wrapping_add(b2);

    let mut bin = [0u8; 32];
    r.to_little_endian(&mut bin);
    let bin2 = e2562array(r2);

    assert_eq!(bin, bin2);
}

fn fuzz_sub(data: &[u8]) {
    let (a, b) = gen_uint256_pair(data);
    let (a2, b2) = gen_eint256_pair(data);

    let (r, _) = a.overflowing_sub(b);
    let r2 = a2.wrapping_sub(b2);

    let mut bin = [0u8; 32];
    r.to_little_endian(&mut bin);
    let bin2 = e2562array(r2);

    assert_eq!(bin, bin2);
}

fuzz_target!(|data: [u8; 64]| {
    fuzz_mul(&data);
    fuzz_add(&data);
    fuzz_sub(&data);
});
