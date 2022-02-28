#![no_main]
use eint::{Eint, E256};
use libfuzzer_sys::fuzz_target;
use uint::construct_uint;

construct_uint! {
    pub struct U256(4);
}

impl std::convert::From<E256> for U256 {
    fn from(e: E256) -> Self {
        let mut buf = [0u8; 32];
        e.put(&mut buf);
        U256::from_little_endian(&buf)
    }
}

impl std::convert::From<U256> for E256 {
    fn from(u: U256) -> Self {
        let mut buf = [0u8; 32];
        u.to_little_endian(&mut buf);
        E256::get(&buf)
    }
}

fn test_and(data: &[u8]) {
    let u0 = U256::from_little_endian(&data[0x00..0x20]);
    let u1 = U256::from_little_endian(&data[0x20..0x40]);
    let e0 = E256::get(&data[0x00..0x20]);
    let e1 = E256::get(&data[0x20..0x40]);
    let ru = u0 & u1;
    let re = e0 & e1;
    assert_eq!(Into::<E256>::into(ru), re);
}

fn test_clz(data: &[u8]) {
    let u0 = U256::from_little_endian(&data[0x00..0x20]);
    let e0 = E256::get(&data[0x00..0x20]);
    let ru = u0.leading_zeros();
    let re = e0.clz();
    assert_eq!(ru, re);
}

fn test_cmp_u(data: &[u8]) {
    let u0 = U256::from_little_endian(&data[0x00..0x20]);
    let u1 = U256::from_little_endian(&data[0x20..0x40]);
    let e0 = E256::get(&data[0x00..0x20]);
    let e1 = E256::get(&data[0x20..0x40]);
    assert_eq!(u0 < u1, e0 < e1);
    assert_eq!(u0 > u1, e0 > e1);
    assert_eq!(u0 <= u1, e0 <= e1);
    assert_eq!(u0 >= u1, e0 >= e1);
    assert_eq!(u0 == u1, e0 == e1);
    assert_eq!(u0 != u1, e0 != e1);
}

fn test_ctz(data: &[u8]) {
    let u0 = U256::from_little_endian(&data[0x00..0x20]);
    let e0 = E256::get(&data[0x00..0x20]);
    let ru = u0.trailing_zeros();
    let re = e0.ctz();
    assert_eq!(ru, re);
}

fn test_not(data: &[u8]) {
    let u0 = U256::from_little_endian(&data[0x00..0x20]);
    let e0 = E256::get(&data[0x00..0x20]);
    let ru = !u0;
    let re = !e0;
    assert_eq!(Into::<E256>::into(ru), re);
}

fn test_or(data: &[u8]) {
    let u0 = U256::from_little_endian(&data[0x00..0x20]);
    let u1 = U256::from_little_endian(&data[0x20..0x40]);
    let e0 = E256::get(&data[0x00..0x20]);
    let e1 = E256::get(&data[0x20..0x40]);
    let ru = u0 | u1;
    let re = e0 | e1;
    assert_eq!(Into::<E256>::into(ru), re);
}

fn test_saturating_add(data: &[u8]) {
    let u0 = U256::from_little_endian(&data[0x00..0x20]);
    let u1 = U256::from_little_endian(&data[0x20..0x40]);
    let e0 = E256::get(&data[0x00..0x20]);
    let e1 = E256::get(&data[0x20..0x40]);
    let ru = u0.saturating_add(u1);
    let (re, _) = e0.saturating_add_u(e1);
    assert_eq!(Into::<E256>::into(ru), re);
}

fn test_saturating_sub(data: &[u8]) {
    let u0 = U256::from_little_endian(&data[0x00..0x20]);
    let u1 = U256::from_little_endian(&data[0x20..0x40]);
    let e0 = E256::get(&data[0x00..0x20]);
    let e1 = E256::get(&data[0x20..0x40]);
    let ru = u0.saturating_sub(u1);
    let (re, _) = e0.saturating_sub_u(e1);
    assert_eq!(Into::<E256>::into(ru), re);
}

fn test_wrapping_add(data: &[u8]) {
    let u0 = U256::from_little_endian(&data[0x00..0x20]);
    let u1 = U256::from_little_endian(&data[0x20..0x40]);
    let e0 = E256::get(&data[0x00..0x20]);
    let e1 = E256::get(&data[0x20..0x40]);
    let (ru, _) = u0.overflowing_add(u1);
    let re = e0.wrapping_add(e1);
    assert_eq!(Into::<E256>::into(ru), re);
}

fn test_wrapping_div_u(data: &[u8]) {
    let u0 = U256::from_little_endian(&data[0x00..0x20]);
    let u1 = U256::from_little_endian(&data[0x20..0x40]);
    let e0 = E256::get(&data[0x00..0x20]);
    let e1 = E256::get(&data[0x20..0x40]);
    let ru = if u1.is_zero() { U256::MAX } else { u0 / u1 };
    let re = e0 / e1;
    assert_eq!(Into::<E256>::into(ru), re);
}

fn test_wrapping_mul(data: &[u8]) {
    let u0 = U256::from_little_endian(&data[0x00..0x20]);
    let u1 = U256::from_little_endian(&data[0x20..0x40]);
    let e0 = E256::get(&data[0x00..0x20]);
    let e1 = E256::get(&data[0x20..0x40]);
    let (ru, _) = u0.overflowing_mul(u1);
    let re = e0.wrapping_mul(e1);
    assert_eq!(Into::<E256>::into(ru), re);
}

fn test_wrapping_rem_u(data: &[u8]) {
    let u0 = U256::from_little_endian(&data[0x00..0x20]);
    let u1 = U256::from_little_endian(&data[0x20..0x40]);
    let e0 = E256::get(&data[0x00..0x20]);
    let e1 = E256::get(&data[0x20..0x40]);
    let ru = if u1.is_zero() { u0 } else { u0.checked_rem(u1).unwrap() };
    let re = e0.wrapping_rem_u(e1);
    assert_eq!(Into::<E256>::into(ru), re);
}

fn test_wrapping_shl(data: &[u8]) {
    let u0 = U256::from_little_endian(&data[0x00..0x20]);
    let u1 = U256::from_little_endian(&data[0x20..0x40]);
    let e0 = E256::get(&data[0x00..0x20]);
    let e1 = E256::get(&data[0x20..0x40]);
    let ru = u0 << (u1.low_u32() as u8);
    let re = e0 << (e1.u32() % 256);
    assert_eq!(Into::<E256>::into(ru), re);
}

fn test_wrapping_shr(data: &[u8]) {
    let u0 = U256::from_little_endian(&data[0x00..0x20]);
    let u1 = U256::from_little_endian(&data[0x20..0x40]);
    let e0 = E256::get(&data[0x00..0x20]);
    let e1 = E256::get(&data[0x20..0x40]);
    let ru = u0 >> (u1.low_u32() as u8);
    let re = e0 >> (e1.u32() % 256);
    assert_eq!(Into::<E256>::into(ru), re);
}

fn test_wrapping_sub(data: &[u8]) {
    let u0 = U256::from_little_endian(&data[0x00..0x20]);
    let u1 = U256::from_little_endian(&data[0x20..0x40]);
    let e0 = E256::get(&data[0x00..0x20]);
    let e1 = E256::get(&data[0x20..0x40]);
    let (ru, _) = u0.overflowing_sub(u1);
    let re = e0.wrapping_sub(e1);
    assert_eq!(Into::<E256>::into(ru), re);
}

fn test_xor(data: &[u8]) {
    let u0 = U256::from_little_endian(&data[0x00..0x20]);
    let u1 = U256::from_little_endian(&data[0x20..0x40]);
    let e0 = E256::get(&data[0x00..0x20]);
    let e1 = E256::get(&data[0x20..0x40]);
    let ru = u0 ^ u1;
    let re = e0 ^ e1;
    assert_eq!(Into::<E256>::into(ru), re);
}

fuzz_target!(|data: [u8; 64]| {
    test_and(&data);
    test_clz(&data);
    test_cmp_u(&data);
    test_ctz(&data);
    test_not(&data);
    test_or(&data);
    test_saturating_add(&data);
    test_saturating_sub(&data);
    test_wrapping_add(&data);
    test_wrapping_div_u(&data);
    test_wrapping_mul(&data);
    test_wrapping_rem_u(&data);
    test_wrapping_shl(&data);
    test_wrapping_shr(&data);
    test_wrapping_sub(&data);
    test_xor(&data);
});
