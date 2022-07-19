#![no_main]
use crate::c_impl::*;
use eint::*;
use libfuzzer_sys::fuzz_target;

construct_eint_twin!(T64, 1);
uint_twin_from_impl!(T64, E32);

impl T64 {
    fn recv(small: u64) -> Self {
        Self([small])
    }

    fn into(self) -> E64 {
        E64(self.0[0])
    }
}

fn test_average_add_s(x: u64, y: u64) {
    let r0 = Eint::average_add_s(E64::from(x), E64::from(y));
    let r1 = Eint::average_add_s(T64::recv(x), T64::recv(y));
    assert_eq!(r0, r1.into());
    assert_eq!(r0, E64((((x as i64 as i128) + (y as i64 as i128)) >> 1) as i64 as u64));
}

fn test_average_add_u(x: u64, y: u64) {
    let r0 = Eint::average_add_u(E64::from(x), E64::from(y));
    let r1 = Eint::average_add_u(T64::recv(x), T64::recv(y));
    assert_eq!(r0, r1.into());
    assert_eq!(r0, E64(((x as u128 + y as u128) >> 1) as u64));
}

fn test_average_sub_s(x: u64, y: u64) {
    let r0 = Eint::average_sub_s(E64::from(x), E64::from(y));
    let r1 = Eint::average_sub_s(T64::recv(x), T64::recv(y));
    assert_eq!(r0, r1.into());
    assert_eq!(
        r0,
        E64((((x as i64 as i128).wrapping_sub(y as i64 as i128)) >> 1) as i64 as u64)
    );
}

fn test_average_sub_u(x: u64, y: u64) {
    let r0 = Eint::average_sub_u(E64::from(x), E64::from(y));
    let r1 = Eint::average_sub_u(T64::recv(x), T64::recv(y));
    assert_eq!(r0, r1.into());
    assert_eq!(r0, E64(((x as u128).wrapping_sub(y as u128) >> 1) as u64));
}

fn test_bit(x: u64, y: u64) {
    let r0 = E64::from(x).bit(y as u32);
    let r1 = T64::recv(x).bit(y as u32);
    assert_eq!(r0, r1);
    assert_eq!(r0, x.wrapping_shr(y as u32) & 1 != 0);
}

fn test_bit_clr(x: u64, y: u64) {
    let mut r0 = E64::from(x);
    let mut r1 = T64::recv(x);
    r0.bit_clr(y as u32);
    r1.bit_clr(y as u32);
    assert_eq!(r0, r1.into());
    assert_eq!(r0.0, x & !(1u64.wrapping_shl(y as u32)));
}

fn test_bit_set(x: u64, y: u64) {
    let mut r0 = E64::from(x);
    let mut r1 = T64::recv(x);
    r0.bit_set(y as u32);
    r1.bit_set(y as u32);
    assert_eq!(r0, r1.into());
    assert_eq!(r0.0, x | 1u64.wrapping_shl(y as u32));
}

fn test_clz(x: u64, _y: u64) {
    let r0 = E64::from(x).clz();
    let r1 = T64::recv(x).clz();
    assert_eq!(r0, r1);
    assert_eq!(r0, x.leading_zeros());
}

fn test_cmp_s(x: u64, y: u64) {
    let r0 = Eint::cmp_s(&E64::from(x), &E64::from(y));
    let r1 = Eint::cmp_s(&T64::recv(x), &T64::recv(y));
    let r2 = (x as i64).cmp(&(y as i64));
    assert_eq!(r0, r1);
    assert_eq!(r0, r2);
}

fn test_cmp_u(x: u64, y: u64) {
    let r0 = Eint::cmp_u(&E64::from(x), &E64::from(y));
    let r1 = Eint::cmp_u(&T64::recv(x), &T64::recv(y));
    let r2 = x.cmp(&y);
    assert_eq!(r0, r1);
    assert_eq!(r0, r2);
}

fn test_cpop(x: u64, _y: u64) {
    let r0 = E64::from(x).cpop();
    let r1 = T64::recv(x).cpop();
    assert_eq!(r0, r1);
    assert_eq!(r0, x.count_ones());
}

fn test_ctz(x: u64, _y: u64) {
    let r0 = E64::from(x).ctz();
    let r1 = T64::recv(x).ctz();
    assert_eq!(r0, r1);
    assert_eq!(r0, x.trailing_zeros());
}

fn test_is_negative(x: u64, _y: u64) {
    let r0 = E64::from(x).is_negative();
    let r1 = T64::recv(x).is_negative();
    assert_eq!(r0, r1);
}

fn test_is_positive(x: u64, _y: u64) {
    let r0 = E64::from(x).is_positive();
    let r1 = T64::recv(x).is_positive();
    assert_eq!(r0, r1);
    assert_eq!(r0, (x as i64).is_positive());
}

fn test_overflowing_add_s(x: u64, y: u64) {
    let (r0, b0) = Eint::overflowing_add_s(E64::from(x), E64::from(y));
    let (r1, b1) = Eint::overflowing_add_s(T64::recv(x), T64::recv(y));
    assert_eq!(r0, r1.into());
    assert_eq!(b0, b1);
}

fn test_overflowing_add_u(x: u64, y: u64) {
    let (r0, b0) = Eint::overflowing_add_u(E64::from(x), E64::from(y));
    let (r1, b1) = Eint::overflowing_add_u(T64::recv(x), T64::recv(y));
    assert_eq!(r0, r1.into());
    assert_eq!(b0, b1);
}

fn test_overflowing_mul_s(x: u64, y: u64) {
    let (r0, b0) = Eint::overflowing_mul_s(E64::from(x), E64::from(y));
    let (r1, b1) = Eint::overflowing_mul_s(T64::recv(x), T64::recv(y));
    let (r2, b2) = (x as i64).overflowing_mul(y as i64);
    assert_eq!(r0, r1.into());
    assert_eq!(b0, b1);
    assert_eq!(r0, E64(r2 as u64));
    assert_eq!(b0, b2);
}

fn test_overflowing_mul_u(x: u64, y: u64) {
    let (r0, b0) = Eint::overflowing_mul_u(E64::from(x), E64::from(y));
    let (r1, b1) = Eint::overflowing_mul_u(T64::recv(x), T64::recv(y));
    let (r2, b2) = x.overflowing_mul(y);
    assert_eq!(r0, r1.into());
    assert_eq!(b0, b1);
    assert_eq!(r0, E64(r2));
    assert_eq!(b0, b2);
}

fn test_overflowing_sub_s(x: u64, y: u64) {
    let (r0, b0) = Eint::overflowing_sub_s(E64::from(x), E64::from(y));
    let (r1, b1) = Eint::overflowing_sub_s(T64::recv(x), T64::recv(y));
    assert_eq!(r0, r1.into());
    assert_eq!(b0, b1);
}

fn test_overflowing_sub_u(x: u64, y: u64) {
    let (r0, b0) = Eint::overflowing_sub_u(E64::from(x), E64::from(y));
    let (r1, b1) = Eint::overflowing_sub_u(T64::recv(x), T64::recv(y));
    assert_eq!(r0, r1.into());
    assert_eq!(b0, b1);
}

fn test_saturating_add_s(x: u64, y: u64) {
    let (r0, b0) = Eint::saturating_add_s(E64::from(x), E64::from(y));
    let (r1, b1) = Eint::saturating_add_s(T64::recv(x), T64::recv(y));
    assert_eq!(r0, r1.into());
    assert_eq!(b0, b1);
    assert_eq!(r0, E64((x as i64).saturating_add(y as i64) as u64))
}

fn test_saturating_add_u(x: u64, y: u64) {
    let (r0, b0) = Eint::saturating_add_u(E64::from(x), E64::from(y));
    let (r1, b1) = Eint::saturating_add_u(T64::recv(x), T64::recv(y));
    assert_eq!(r0, r1.into());
    assert_eq!(b0, b1);
    assert_eq!(r0, E64(x.saturating_add(y)))
}

fn test_saturating_sub_s(x: u64, y: u64) {
    let (r0, b0) = Eint::saturating_sub_s(E64::from(x), E64::from(y));
    let (r1, b1) = Eint::saturating_sub_s(T64::recv(x), T64::recv(y));
    assert_eq!(r0, r1.into());
    assert_eq!(b0, b1);
    assert_eq!(r0, E64((x as i64).saturating_sub(y as i64) as u64))
}

fn test_saturating_sub_u(x: u64, y: u64) {
    let (r0, b0) = Eint::saturating_sub_u(E64::from(x), E64::from(y));
    let (r1, b1) = Eint::saturating_sub_u(T64::recv(x), T64::recv(y));
    assert_eq!(r0, r1.into());
    assert_eq!(b0, b1);
    assert_eq!(r0, E64(x.saturating_sub(y)))
}

fn test_widening_add_s(x: u64, y: u64) {
    let r0 = Eint::widening_add_s(E64::from(x), E64::from(y));
    let r1 = Eint::widening_add_s(T64::recv(x), T64::recv(y));
    let r2 = x as i64 as i128 + y as i64 as i128;
    assert_eq!(r0.0, r1.0.into());
    assert_eq!(r0.1, r1.1.into());
    assert_eq!(r0.0, E64(r2 as u128 as u64));
    assert_eq!(r0.1, E64((r2 as u128 >> 64) as u64));
}

fn test_widening_add_u(x: u64, y: u64) {
    let r0 = Eint::widening_add_u(E64::from(x), E64::from(y));
    let r1 = Eint::widening_add_u(T64::recv(x), T64::recv(y));
    let r2 = x as u128 + y as u128;
    assert_eq!(r0.0, r1.0.into());
    assert_eq!(r0.1, r1.1.into());
    assert_eq!(r0.0, E64(r2 as u64));
    assert_eq!(r0.1, E64((r2 >> 64) as u64));
}

fn test_widening_mul_s(x: u64, y: u64) {
    let r0 = Eint::widening_mul_s(E64::from(x), E64::from(y));
    let r1 = Eint::widening_mul_s(T64::recv(x), T64::recv(y));
    let r2 = (x as i64 as i128 * y as i64 as i128) as u128;
    assert_eq!(r0.0, r1.0.into());
    assert_eq!(r0.1, r1.1.into());
    assert_eq!(r0.0, E64(r2 as u64));
    assert_eq!(r0.1, E64((r2 >> 64) as u64));
}

fn test_widening_mul_su(x: u64, y: u64) {
    let r0 = Eint::widening_mul_su(E64::from(x), E64::from(y));
    let r1 = Eint::widening_mul_su(T64::recv(x), T64::recv(y));
    let r2 = (x as i64 as i128 * y as u128 as i128) as u128;
    assert_eq!(r0.0, r1.0.into());
    assert_eq!(r0.1, r1.1.into());
    assert_eq!(r0.0, E64(r2 as u64));
    assert_eq!(r0.1, E64((r2 >> 64) as u64));
}

fn test_widening_mul_u(x: u64, y: u64) {
    let r0 = Eint::widening_mul_u(E64::from(x), E64::from(y));
    let r1 = Eint::widening_mul_u(T64::recv(x), T64::recv(y));
    let r2 = x as u128 * y as u128;
    assert_eq!(r0.0, r1.0.into());
    assert_eq!(r0.1, r1.1.into());
    assert_eq!(r0.0, E64(r2 as u64));
    assert_eq!(r0.1, E64((r2 >> 64) as u64));
}

fn test_widening_sub_s(x: u64, y: u64) {
    let r0 = Eint::widening_sub_s(E64::from(x), E64::from(y));
    let r1 = Eint::widening_sub_s(T64::recv(x), T64::recv(y));
    let r2 = (x as i64 as i128 - y as i64 as i128) as u128;
    assert_eq!(r0.0, r1.0.into());
    assert_eq!(r0.1, r1.1.into());
    assert_eq!(r0.0, E64(r2 as u64));
    assert_eq!(r0.1, E64((r2 >> 64) as u64));
}

fn test_widening_sub_u(x: u64, y: u64) {
    let r0 = Eint::widening_sub_u(E64::from(x), E64::from(y));
    let r1 = Eint::widening_sub_u(T64::recv(x), T64::recv(y));
    let r2 = (x as u128).wrapping_sub(y as u128);
    assert_eq!(r0.0, r1.0.into());
    assert_eq!(r0.1, r1.1.into());
    assert_eq!(r0.0, E64(r2 as u64));
    assert_eq!(r0.1, E64((r2 >> 64) as u64));
}

fn test_wrapping_add(x: u64, y: u64) {
    let r0 = Eint::wrapping_add(E64::from(x), E64::from(y));
    let r1 = Eint::wrapping_add(T64::recv(x), T64::recv(y));
    assert_eq!(r0, r1.into());
}

fn test_wrapping_div_s(x: u64, y: u64) {
    let r0 = Eint::wrapping_div_s(E64::from(x), E64::from(y));
    let r1 = Eint::wrapping_div_s(T64::recv(x), T64::recv(y));
    let r2 = if y == 0 {
        u64::MAX
    } else if x as i64 == i64::MIN && y as i64 == -1 {
        i64::MIN as u64
    } else {
        (x as i64 / y as i64) as u64
    };
    assert_eq!(r0, r1.into());
    assert_eq!(r0, E64(r2));
}

fn test_wrapping_div_u(x: u64, y: u64) {
    let r0 = Eint::wrapping_div_u(E64::from(x), E64::from(y));
    let r1 = Eint::wrapping_div_u(T64::recv(x), T64::recv(y));
    let r2 = if y == 0 { u64::MAX } else { x / y };
    assert_eq!(r0, r1.into());
    assert_eq!(r0, E64(r2));
}

fn test_wrapping_mul(x: u64, y: u64) {
    let r0 = Eint::wrapping_mul(E64::from(x), E64::from(y));
    let r1 = Eint::wrapping_mul(T64::recv(x), T64::recv(y));
    assert_eq!(r0, r1.into());
}

fn test_wrapping_rem_s(x: u64, y: u64) {
    let r0 = Eint::wrapping_rem_s(E64::from(x), E64::from(y));
    let r1 = Eint::wrapping_rem_s(T64::recv(x), T64::recv(y));
    let r2 = if y == 0 {
        x
    } else if x as i64 == i64::MIN && y as i64 == -1 {
        0
    } else {
        (x as i64 % y as i64) as u64
    };
    assert_eq!(r0, r1.into());
    assert_eq!(r0, E64(r2));
}

fn test_wrapping_rem_u(x: u64, y: u64) {
    let r0 = Eint::wrapping_rem_u(E64::from(x), E64::from(y));
    let r1 = Eint::wrapping_rem_u(T64::recv(x), T64::recv(y));
    let r2 = if y == 0 { x } else { x % y };
    assert_eq!(r0, r1.into());
    assert_eq!(r0, E64(r2));
}

fn test_wrapping_shl(x: u64, y: u32) {
    let r0 = E64::from(x).wrapping_shl(y);
    let r1 = T64::recv(x).wrapping_shl(y);
    assert_eq!(r0, r1.into());
}

fn test_wrapping_shr(x: u64, y: u32) {
    let r0 = E64::from(x).wrapping_shr(y);
    let r1 = T64::recv(x).wrapping_shr(y);
    assert_eq!(r0, r1.into());
}

fn test_wrapping_sra(x: u64, y: u32) {
    let r0 = E64::from(x).wrapping_sra(y);
    let r1 = T64::recv(x).wrapping_sra(y);
    let r2 = (x as i64).wrapping_shr(y) as u64;
    assert_eq!(r0, r1.into());
    assert_eq!(r0, E64(r2));
}

fn test_wrapping_sub(x: u64, y: u64) {
    let r0 = Eint::wrapping_sub(E64::from(x), E64::from(y));
    let r1 = Eint::wrapping_sub(T64::recv(x), T64::recv(y));
    assert_eq!(r0, r1.into());
}

fuzz_target!(|data: (u64, u64)| {
    test_average_add_s(data.0, data.1);
    test_average_add_u(data.0, data.1);
    test_average_sub_s(data.0, data.1);
    test_average_sub_u(data.0, data.1);
    test_bit(data.0, data.1);
    test_bit_clr(data.0, data.1);
    test_bit_set(data.0, data.1);
    test_clz(data.0, data.1);
    test_cmp_s(data.0, data.1);
    test_cmp_u(data.0, data.1);
    test_cpop(data.0, data.1);
    test_ctz(data.0, data.1);
    test_is_negative(data.0, data.1);
    test_is_positive(data.0, data.1);
    test_overflowing_add_s(data.0, data.1);
    test_overflowing_add_u(data.0, data.1);
    test_overflowing_mul_s(data.0, data.1);
    test_overflowing_mul_u(data.0, data.1);
    test_overflowing_sub_s(data.0, data.1);
    test_overflowing_sub_u(data.0, data.1);
    test_saturating_add_s(data.0, data.1);
    test_saturating_add_u(data.0, data.1);
    test_saturating_sub_s(data.0, data.1);
    test_saturating_sub_u(data.0, data.1);
    test_widening_add_s(data.0, data.1);
    test_widening_add_u(data.0, data.1);
    test_widening_mul_s(data.0, data.1);
    test_widening_mul_su(data.0, data.1);
    test_widening_mul_u(data.0, data.1);
    test_widening_sub_s(data.0, data.1);
    test_widening_sub_u(data.0, data.1);
    test_wrapping_add(data.0, data.1);
    test_wrapping_div_s(data.0, data.1);
    test_wrapping_div_u(data.0, data.1);
    test_wrapping_mul(data.0, data.1);
    test_wrapping_rem_s(data.0, data.1);
    test_wrapping_rem_u(data.0, data.1);
    test_wrapping_sub(data.0, data.1);
    test_wrapping_shl(data.0, data.1 as u32);
    test_wrapping_shr(data.0, data.1 as u32);
    test_wrapping_sra(data.0, data.1 as u32);
});
