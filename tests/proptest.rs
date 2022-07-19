use eint::*;
use proptest::prelude::*;

construct_eint_twin!(T64, 1);
impl_widening_mul_u_twin!(T64, 1);
uint_twin_from_impl!(T64, E32);

impl T64 {
    fn recv(small: u64) -> Self {
        Self([small])
    }

    fn into(self) -> E64 {
        E64(self.0[0])
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 4096,
        ..ProptestConfig::default()
    })]

    #[test]
    fn test_average_add_s(x in 0..=u64::MAX, y in 0..=u64::MAX) {
        let r0 = Eint::average_add_s(E64::from(x), E64::from(y));
        let r1 = Eint::average_add_s(T64::recv(x), T64::recv(y));
        assert_eq!(r0, r1.into());
        assert_eq!(r0, E64((((x as i64 as i128) + (y as i64 as i128)) >> 1) as i64 as u64));
    }

    #[test]
    fn test_average_add_u(x in 0..=u64::MAX, y in 0..=u64::MAX) {
        let r0 = Eint::average_add_u(E64::from(x), E64::from(y));
        let r1 = Eint::average_add_u(T64::recv(x), T64::recv(y));
        assert_eq!(r0, r1.into());
        assert_eq!(r0, E64(((x as u128 + y as u128) >> 1) as u64));
    }

    #[test]
    fn test_average_sub_s(x in 0..=u64::MAX, y in 0..=u64::MAX) {
        let r0 = Eint::average_sub_s(E64::from(x), E64::from(y));
        let r1 = Eint::average_sub_s(T64::recv(x), T64::recv(y));
        assert_eq!(r0, r1.into());
        assert_eq!(r0, E64((((x as i64 as i128).wrapping_sub(y as i64 as i128)) >> 1) as i64 as u64));
    }

    #[test]
    fn test_average_sub_u(x in 0..=u64::MAX, y in 0..=u64::MAX) {
        let r0 = Eint::average_sub_u(E64::from(x), E64::from(y));
        let r1 = Eint::average_sub_u(T64::recv(x), T64::recv(y));
        assert_eq!(r0, r1.into());
        assert_eq!(r0, E64(((x as u128).wrapping_sub(y as u128) >> 1) as u64));
    }

    #[test]
    fn test_bit(x in 0..=u64::MAX, y in 0..=u32::MAX) {
        let r0 = E64::from(x).bit(y);
        let r1 = T64::recv(x).bit(y);
        assert_eq!(r0, r1);
        assert_eq!(r0, x.wrapping_shr(y) & 1 != 0);
    }

    #[test]
    fn test_bit_clr(x in 0..=u64::MAX, y in 0..=u32::MAX) {
        let mut r0 = E64::from(x);
        let mut r1 = T64::recv(x);
        r0.bit_clr(y);
        r1.bit_clr(y);
        assert_eq!(r0, r1.into());
        assert_eq!(r0.0, x & !(1u64.wrapping_shl(y)));
    }

    #[test]
    fn test_bit_set(x in 0..=u64::MAX, y in 0..=u32::MAX) {
        let mut r0 = E64::from(x);
        let mut r1 = T64::recv(x);
        r0.bit_set(y);
        r1.bit_set(y);
        assert_eq!(r0, r1.into());
        assert_eq!(r0.0, x | 1u64.wrapping_shl(y));
    }

    #[test]
    fn test_clz(x in 0..=u64::MAX) {
        let r0 = E64::from(x).clz();
        let r1 = T64::recv(x).clz();
        assert_eq!(r0, r1);
        assert_eq!(r0, x.leading_zeros());
    }

    #[test]
    fn test_cmp_s(x in 0..=u64::MAX, y in 0..=u64::MAX) {
        let r0 = Eint::cmp_s(&E64::from(x), &E64::from(y));
        let r1 = Eint::cmp_s(&T64::recv(x), &T64::recv(y));
        let r2 = (x as i64).cmp(&(y as i64));
        assert_eq!(r0, r1);
        assert_eq!(r0, r2);
    }

    #[test]
    fn test_cmp_u(x in 0..=u64::MAX, y in 0..=u64::MAX) {
        let r0 = Eint::cmp_u(&E64::from(x), &E64::from(y));
        let r1 = Eint::cmp_u(&T64::recv(x), &T64::recv(y));
        let r2 = x.cmp(&y);
        assert_eq!(r0, r1);
        assert_eq!(r0, r2);
    }

    #[test]
    fn test_cpop(x in 0..=u64::MAX) {
        let r0 = E64::from(x).cpop();
        let r1 = T64::recv(x).cpop();
        assert_eq!(r0, r1);
        assert_eq!(r0, x.count_ones());
    }

    #[test]
    fn test_ctz(x in 0..=u64::MAX) {
        let r0 = E64::from(x).ctz();
        let r1 = T64::recv(x).ctz();
        assert_eq!(r0, r1);
        assert_eq!(r0, x.trailing_zeros());
    }

    #[test]
    fn test_get(x in proptest::collection::vec(0..=u8::MAX, 32)) {
        let r0 = E256::get(&x);
        let r1 = unsafe { std::mem::transmute::<[u8; 32], E256>(x.as_slice().try_into().unwrap()) };
        assert_eq!(r0, r1);
    }

    #[test]
    fn test_is_negative(x in 0..=u64::MAX) {
        let r0 = E64::from(x).is_negative();
        let r1 = T64::recv(x).is_negative();
        assert_eq!(r0, r1);
    }

    #[test]
    fn test_is_positive(x in 0..=u64::MAX) {
        let r0 = E64::from(x).is_positive();
        let r1 = T64::recv(x).is_positive();
        assert_eq!(r0, r1);
        assert_eq!(r0, (x as i64).is_positive());
    }

    #[test]
    fn test_overflowing_add_s(x in 0..=u64::MAX, y in 0..=u64::MAX) {
        let (r0, b0) = Eint::overflowing_add_s(E64::from(x), E64::from(y));
        let (r1, b1) = Eint::overflowing_add_s(T64::recv(x), T64::recv(y));
        assert_eq!(r0, r1.into());
        assert_eq!(b0, b1);
    }

    #[test]
    fn test_overflowing_add_u(x in 0..=u64::MAX, y in 0..=u64::MAX) {
        let (r0, b0) = Eint::overflowing_add_u(E64::from(x), E64::from(y));
        let (r1, b1) = Eint::overflowing_add_u(T64::recv(x), T64::recv(y));
        assert_eq!(r0, r1.into());
        assert_eq!(b0, b1);
    }

    #[test]
    fn test_overflowing_mul_s(x in 0..=u64::MAX, y in 0..=u64::MAX) {
        let (r0, b0) = Eint::overflowing_mul_s(E64::from(x), E64::from(y));
        let (r1, b1) = Eint::overflowing_mul_s(T64::recv(x), T64::recv(y));
        let (r2, b2) = (x as i64).overflowing_mul(y as i64);
        assert_eq!(r0, r1.into());
        assert_eq!(b0, b1);
        assert_eq!(r0, E64(r2 as u64));
        assert_eq!(b0, b2);
    }

    #[test]
    fn test_overflowing_mul_u(x in 0..=u64::MAX, y in 0..=u64::MAX) {
        let (r0, b0) = Eint::overflowing_mul_u(E64::from(x), E64::from(y));
        let (r1, b1) = Eint::overflowing_mul_u(T64::recv(x), T64::recv(y));
        let (r2, b2) = x.overflowing_mul(y);
        assert_eq!(r0, r1.into());
        assert_eq!(b0, b1);
        assert_eq!(r0, E64(r2));
        assert_eq!(b0, b2);
    }

    #[test]
    fn test_overflowing_sub_s(x in 0..=u64::MAX, y in 0..=u64::MAX) {
        let (r0, b0) = Eint::overflowing_sub_s(E64::from(x), E64::from(y));
        let (r1, b1) = Eint::overflowing_sub_s(T64::recv(x), T64::recv(y));
        assert_eq!(r0, r1.into());
        assert_eq!(b0, b1);
    }

    #[test]
    fn test_overflowing_sub_u(x in 0..=u64::MAX, y in 0..=u64::MAX) {
        let (r0, b0) = Eint::overflowing_sub_u(E64::from(x), E64::from(y));
        let (r1, b1) = Eint::overflowing_sub_u(T64::recv(x), T64::recv(y));
        assert_eq!(r0, r1.into());
        assert_eq!(b0, b1);
    }

    #[test]
    fn test_put(x in proptest::collection::vec(0..=u8::MAX, 32)) {
        let mut r = [0u8; 32];
        E256::get(&x).put(&mut r);
        assert_eq!(x, r);
    }

    #[test]
    fn test_saturating_add_s(x in 0..=u64::MAX, y in 0..=u64::MAX){
        let (r0, b0) = Eint::saturating_add_s(E64::from(x), E64::from(y));
        let (r1, b1) = Eint::saturating_add_s(T64::recv(x), T64::recv(y));
        assert_eq!(r0, r1.into());
        assert_eq!(b0, b1);
        assert_eq!(r0, E64((x as i64).saturating_add(y as i64) as u64))
    }

    #[test]
    fn test_saturating_add_u(x in 0..=u64::MAX, y in 0..=u64::MAX){
        let (r0, b0) = Eint::saturating_add_u(E64::from(x), E64::from(y));
        let (r1, b1) = Eint::saturating_add_u(T64::recv(x), T64::recv(y));
        assert_eq!(r0, r1.into());
        assert_eq!(b0, b1);
        assert_eq!(r0, E64(x.saturating_add(y)))
    }

    #[test]
    fn test_saturating_sub_s(x in 0..=u64::MAX, y in 0..=u64::MAX){
        let (r0, b0) = Eint::saturating_sub_s(E64::from(x), E64::from(y));
        let (r1, b1) = Eint::saturating_sub_s(T64::recv(x), T64::recv(y));
        assert_eq!(r0, r1.into());
        assert_eq!(b0, b1);
        assert_eq!(r0, E64((x as i64).saturating_sub(y as i64) as u64))
    }

    #[test]
    fn test_saturating_sub_u(x in 0..=u64::MAX, y in 0..=u64::MAX){
        let (r0, b0) = Eint::saturating_sub_u(E64::from(x), E64::from(y));
        let (r1, b1) = Eint::saturating_sub_u(T64::recv(x), T64::recv(y));
        assert_eq!(r0, r1.into());
        assert_eq!(b0, b1);
        assert_eq!(r0, E64(x.saturating_sub(y)))
    }

    #[test]
    fn test_widening_add_s(x in 0..=u64::MAX, y in 0..=u64::MAX) {
        let r0 = Eint::widening_add_s(E64::from(x), E64::from(y));
        let r1 = Eint::widening_add_s(T64::recv(x), T64::recv(y));
        let r2 = x as i64 as i128 + y as i64 as i128;
        assert_eq!(r0.0, r1.0.into());
        assert_eq!(r0.1, r1.1.into());
        assert_eq!(r0.0, E64(r2 as u128 as u64));
        assert_eq!(r0.1, E64((r2 as u128 >> 64) as u64));
    }

    #[test]
    fn test_widening_add_u(x in 0..=u64::MAX, y in 0..=u64::MAX) {
        let r0 = Eint::widening_add_u(E64::from(x), E64::from(y));
        let r1 = Eint::widening_add_u(T64::recv(x), T64::recv(y));
        let r2 = x as u128 + y as u128;
        assert_eq!(r0.0, r1.0.into());
        assert_eq!(r0.1, r1.1.into());
        assert_eq!(r0.0, E64(r2 as u64));
        assert_eq!(r0.1, E64((r2 >> 64) as u64));
    }

    #[test]
    fn test_widening_mul_s(x in 0..=u64::MAX, y in 0..=u64::MAX) {
        let r0 = Eint::widening_mul_s(E64::from(x), E64::from(y));
        let r1 = Eint::widening_mul_s(T64::recv(x), T64::recv(y));
        let r2 = (x as i64 as i128 * y as i64 as i128) as u128;
        assert_eq!(r0.0, r1.0.into());
        assert_eq!(r0.1, r1.1.into());
        assert_eq!(r0.0, E64(r2 as u64));
        assert_eq!(r0.1, E64((r2 >> 64) as u64));
    }

    #[test]
    fn test_widening_mul_su(x in 0..=u64::MAX, y in 0..=u64::MAX) {
        let r0 = Eint::widening_mul_su(E64::from(x), E64::from(y));
        let r1 = Eint::widening_mul_su(T64::recv(x), T64::recv(y));
        let r2 = (x as i64 as i128 * y as u128 as i128) as u128;
        assert_eq!(r0.0, r1.0.into());
        assert_eq!(r0.1, r1.1.into());
        assert_eq!(r0.0, E64(r2 as u64));
        assert_eq!(r0.1, E64((r2 >> 64) as u64));
    }

    #[test]
    fn test_widening_mul_u(x in 0..=u64::MAX, y in 0..=u64::MAX) {
        let r0 = Eint::widening_mul_u(E64::from(x), E64::from(y));
        let r1 = Eint::widening_mul_u(T64::recv(x), T64::recv(y));
        let r2 = x as u128 * y as u128;
        assert_eq!(r0.0, r1.0.into());
        assert_eq!(r0.1, r1.1.into());
        assert_eq!(r0.0, E64(r2 as u64));
        assert_eq!(r0.1, E64((r2 >> 64) as u64));
    }

    #[test]
    fn test_widening_sub_s(x in 0..=u64::MAX, y in 0..=u64::MAX) {
        let r0 = Eint::widening_sub_s(E64::from(x), E64::from(y));
        let r1 = Eint::widening_sub_s(T64::recv(x), T64::recv(y));
        let r2 = (x as i64 as i128 - y as i64 as i128) as u128;
        assert_eq!(r0.0, r1.0.into());
        assert_eq!(r0.1, r1.1.into());
        assert_eq!(r0.0, E64(r2 as u64));
        assert_eq!(r0.1, E64((r2 >> 64) as u64));
    }

    #[test]
    fn test_widening_sub_u(x in 0..=u64::MAX, y in 0..=u64::MAX) {
        let r0 = Eint::widening_sub_u(E64::from(x), E64::from(y));
        let r1 = Eint::widening_sub_u(T64::recv(x), T64::recv(y));
        let r2 = (x as u128).wrapping_sub(y as u128);
        assert_eq!(r0.0, r1.0.into());
        assert_eq!(r0.1, r1.1.into());
        assert_eq!(r0.0, E64(r2 as u64));
        assert_eq!(r0.1, E64((r2 >> 64) as u64));
    }

    #[test]
    fn test_wrapping_add(x in 0..=u64::MAX, y in 0..=u64::MAX) {
        let r0 = Eint::wrapping_add(E64::from(x), E64::from(y));
        let r1 = Eint::wrapping_add(T64::recv(x), T64::recv(y));
        assert_eq!(r0, r1.into());
    }

    #[test]
    fn test_wrapping_div_s(x in 0..=u64::MAX, y in 0..=u64::MAX) {
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

    #[test]
    fn test_wrapping_div_u(x in 0..=u64::MAX, y in 0..=u64::MAX) {
        let r0 = Eint::wrapping_div_u(E64::from(x), E64::from(y));
        let r1 = Eint::wrapping_div_u(T64::recv(x), T64::recv(y));
        let r2 = if y == 0 { u64::MAX } else { x / y };
        assert_eq!(r0, r1.into());
        assert_eq!(r0, E64(r2));
    }

    #[test]
    fn test_wrapping_mul(x in 0..=u64::MAX, y in 0..=u64::MAX) {
        let r0 = Eint::wrapping_mul(E64::from(x), E64::from(y));
        let r1 = Eint::wrapping_mul(T64::recv(x), T64::recv(y));
        assert_eq!(r0, r1.into());
    }

    #[test]
    fn test_wrapping_rem_s(x in 0..=u64::MAX, y in 0..=u64::MAX) {
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

    #[test]
    fn test_wrapping_rem_u(x in 0..=u64::MAX, y in 0..=u64::MAX) {
        let r0 = Eint::wrapping_rem_u(E64::from(x), E64::from(y));
        let r1 = Eint::wrapping_rem_u(T64::recv(x), T64::recv(y));
        let r2 = if y == 0 { x } else { x % y };
        assert_eq!(r0, r1.into());
        assert_eq!(r0, E64(r2));
    }

    #[test]
    fn test_wrapping_shl(x in 0..=u64::MAX, y in 0..=u32::MAX) {
        let r0 = E64::from(x).wrapping_shl(y);
        let r1 = T64::recv(x).wrapping_shl(y);
        assert_eq!(r0, r1.into());
    }

    #[test]
    fn test_wrapping_shr(x in 0..=u64::MAX, y in 0..=u32::MAX) {
        let r0 = E64::from(x).wrapping_shr(y);
        let r1 = T64::recv(x).wrapping_shr(y);
        assert_eq!(r0, r1.into());
    }

    #[test]
    fn test_wrapping_sra(x in 0..=u64::MAX, y in 0..=u32::MAX) {
        let r0 = E64::from(x).wrapping_sra(y);
        let r1 = T64::recv(x).wrapping_sra(y);
        let r2 = (x as i64).wrapping_shr(y) as u64;
        assert_eq!(r0, r1.into());
        assert_eq!(r0, E64(r2));
    }

    #[test]
    fn test_wrapping_sub(x in 0..=u64::MAX, y in 0..=u64::MAX) {
        let r0 = Eint::wrapping_sub(E64::from(x), E64::from(y));
        let r1 = Eint::wrapping_sub(T64::recv(x), T64::recv(y));
        assert_eq!(r0, r1.into());
    }
}

#[test]
fn test_overflowing_mul_s_bug_0() {
    let x: u64 = 0x000000ffffffff12;
    let y: u64 = 0xffffffffffffff00;
    let (r0, b0) = Eint::overflowing_mul_s(E64::from(x), E64::from(y));
    let (r1, b1) = Eint::overflowing_mul_s(T64::recv(x), T64::recv(y));
    let (r2, b2) = (x as i64).overflowing_mul(y as i64);
    assert_eq!(r0, r1.into());
    assert_eq!(b0, b1);
    assert_eq!(r0, E64(r2 as u64));
    assert_eq!(b0, b2);
}

#[test]
fn test_overflowing_mul_s_bug_1() {
    let x: u64 = 0xffffffff00000000;
    let y: u64 = 0xffffff12;
    let (r0, b0) = Eint::overflowing_mul_s(E64::from(x), E64::from(y));
    let (r1, b1) = Eint::overflowing_mul_s(T64::recv(x), T64::recv(y));
    let (r2, b2) = (x as i64).overflowing_mul(y as i64);
    assert_eq!(r0, r1.into());
    assert_eq!(b0, b1);
    assert_eq!(r0, E64(r2 as u64));
    assert_eq!(b0, b2);
}

#[test]
fn test_widening_mul_u_bug_0() {
    let x = E256([0xb12f7788023e73f4, 0xe2aaa5a70e8d29d2, 0x01f281f891d2d8b6, 0x00000000000923b3]);
    let y = E256([0x1d17f2885f4f575d, 0x75bc106493590c97, 0x0080fc6291ec2141, 0x0000000000000d7d]);
    let (lo, hi) = Eint::widening_mul_u(x, y);
    assert_eq!(lo.0[0], 0x2561c5195e640ba4);
    assert_eq!(lo.0[1], 0x16c693e22c0d44ce);
    assert_eq!(lo.0[2], 0xd28d858679950e54);
    assert_eq!(lo.0[3], 0xb3be16ce839dba7c);
    assert_eq!(hi.0[0], 0x0d773f050be2a524);
    assert_eq!(hi.0[1], 0x202d251fd914f3d8);
    assert_eq!(hi.0[2], 0x000000007b468a1c);
    assert_eq!(hi.0[3], 0x0000000000000000);
}
