#![no_main]
mod utils;
use eint::{Eint, E256};
use libfuzzer_sys::fuzz_target;
use utils::{gen_eint256_pair, gen_uint256_pair};

fn fuzz_mul(data: &[u8]) {
    let u = gen_uint256_pair(data);
    let e = gen_eint256_pair(data);

    let (r, _) = u.a.overflowing_mul(u.b);
    let r2 = e.a.wrapping_mul(e.b);

    assert_eq!(Into::<E256>::into(r), r2);
}

fn fuzz_add(data: &[u8]) {
    let u = gen_uint256_pair(data);
    let e = gen_eint256_pair(data);

    let (r, _) = u.a.overflowing_add(u.b);
    let r2 = e.a.wrapping_add(e.b);

    assert_eq!(Into::<E256>::into(r), r2);
}

fn fuzz_sub(data: &[u8]) {
    let u = gen_uint256_pair(data);
    let e = gen_eint256_pair(data);

    let (r, _) = u.a.overflowing_sub(u.b);
    let r2 = e.a.wrapping_sub(e.b);

    assert_eq!(Into::<E256>::into(r), r2);
}

fn fuzz_shl(data: &[u8]) {
    let u = gen_uint256_pair(data);
    let e = gen_eint256_pair(data);

    let r = u.a << (u.b.low_u32() as u8);
    let r2 = e.a << (e.b.0 .0 as u8 as u32);

    assert_eq!(Into::<E256>::into(r), r2);
}

fn fuzz_shr(data: &[u8]) {
    let u = gen_uint256_pair(data);
    let e = gen_eint256_pair(data);

    let r = u.a >> (u.b.low_u32() as u8);
    let r2 = e.a >> (e.b.0 .0 as u8 as u32);

    assert_eq!(Into::<E256>::into(r), r2);
}

fn fuzz_div(data: &[u8]) {
    let u = gen_uint256_pair(data);
    let e = gen_eint256_pair(data);

    // being divided by zero is a special case
    if e.b == E256::MIN_U {
        return;
    }
    let r = u.a / u.b;
    let r2 = e.a / e.b;

    assert_eq!(Into::<E256>::into(r), r2);
}

fn fuzz_rem(data: &[u8]) {
    let u = gen_uint256_pair(data);
    let e = gen_eint256_pair(data);

    // being divided by zero is a special case
    if e.b == E256::MIN_U {
        return;
    }
    let r = u.a.checked_rem(u.b).unwrap();
    let r2 = e.a.wrapping_rem_u(e.b);

    assert_eq!(Into::<E256>::into(r), r2);
}

fn fuzz_leading_zeros(data: &[u8]) {
    let u = gen_uint256_pair(data);
    let e = gen_eint256_pair(data);

    let r = u.a.leading_zeros();
    let r2 = e.a.clz();

    assert_eq!(r, r2);
}

fn fuzz_trailing_zeros(data: &[u8]) {
    let u = gen_uint256_pair(data);
    let e = gen_eint256_pair(data);

    let r = u.a.trailing_zeros();
    let r2 = e.a.ctz();

    assert_eq!(r, r2);
}

fn fuzz_cmp(data: &[u8]) {
    let u = gen_uint256_pair(data);
    let e = gen_eint256_pair(data);

    let r = u.a < u.b;
    let r2 = e.a < e.b;

    assert_eq!(r, r2);
}

fn fuzz_saturating_add(data: &[u8]) {
    let u = gen_uint256_pair(data);
    let e = gen_eint256_pair(data);

    let r = u.a.saturating_add(u.b);
    let (r2, _) = e.a.saturating_add_u(e.b);

    assert_eq!(Into::<E256>::into(r), r2);
}

fn fuzz_saturating_sub(data: &[u8]) {
    let u = gen_uint256_pair(data);
    let e = gen_eint256_pair(data);

    let r = u.a.saturating_sub(u.b);
    let (r2, _) = e.a.saturating_sub_u(e.b);

    assert_eq!(Into::<E256>::into(r), r2);
}

fn fuzz_bit_op(data: &[u8]) {
    let u = gen_uint256_pair(data);
    let e = gen_eint256_pair(data);

    let r = u.a & u.b;
    let r2 = e.a & e.b;
    assert_eq!(Into::<E256>::into(r), r2);

    let r = u.a | u.b;
    let r2 = e.a | e.b;
    assert_eq!(Into::<E256>::into(r), r2);

    let r = u.a ^ u.b;
    let r2 = e.a ^ e.b;
    assert_eq!(Into::<E256>::into(r), r2);

    let r = !u.a;
    let r2 = !e.a;
    assert_eq!(Into::<E256>::into(r), r2);
}

fuzz_target!(|data: [u8; 64]| {
    fuzz_div(&data);
    fuzz_rem(&data);
    fuzz_shl(&data);
    fuzz_shr(&data);
    fuzz_mul(&data);
    fuzz_add(&data);
    fuzz_sub(&data);
    fuzz_leading_zeros(&data);
    fuzz_trailing_zeros(&data);
    fuzz_cmp(&data);
    fuzz_saturating_add(&data);
    fuzz_saturating_sub(&data);
    fuzz_bit_op(&data);
});
