use eint::*;
use proptest::prelude::*;

construct_eint_twin!(T64, E32);
impl std::convert::From<E32> for T64 {
    fn from(small: E32) -> Self {
        Self(small, E32::MIN_U)
    }
}

impl T64 {
    fn recv(small: u64) -> Self {
        Self(E32(small as u32), E32((small >> 32) as u32))
    }

    fn into(self) -> E64 {
        E64(((self.1 .0 as u64) << 32) | (self.0 .0 as u64))
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 1024,
        ..ProptestConfig::default()
    })]

    #[test]
    fn test_average_add_s(x in u64::MIN..=u64::MAX, y in u64::MIN..=u64::MAX) {
        let r0 = Eint::average_add_s(E64::from(x), E64::from(y));
        let r1 = Eint::average_add_s(T64::recv(x), T64::recv(y));
        assert_eq!(r0, r1.into());
        assert_eq!(r0, E64((((x as i64 as i128) + (y as i64 as i128)) >> 1) as i64 as u64));
    }

    #[test]
    fn test_average_add_u(x in u64::MIN..=u64::MAX, y in u64::MIN..=u64::MAX) {
        let r0 = Eint::average_add_u(E64::from(x), E64::from(y));
        let r1 = Eint::average_add_u(T64::recv(x), T64::recv(y));
        assert_eq!(r0, r1.into());
        assert_eq!(r0, E64(((x as u128 + y as u128) >> 1) as u64));
    }

    #[test]
    fn test_average_sub_s(x in u64::MIN..=u64::MAX, y in u64::MIN..=u64::MAX) {
        let r0 = Eint::average_sub_s(E64::from(x), E64::from(y));
        let r1 = Eint::average_sub_s(T64::recv(x), T64::recv(y));
        assert_eq!(r0, r1.into());
        assert_eq!(r0, E64((((x as i64 as i128).wrapping_sub(y as i64 as i128)) >> 1) as i64 as u64));
    }

    #[test]
    fn test_average_sub_u(x in u64::MIN..=u64::MAX, y in u64::MIN..=u64::MAX) {
        let r0 = Eint::average_sub_u(E64::from(x), E64::from(y));
        let r1 = Eint::average_sub_u(T64::recv(x), T64::recv(y));
        assert_eq!(r0, r1.into());
        assert_eq!(r0, E64(((x as u128).wrapping_sub(y as u128) >> 1) as u64));
    }

    #[test]
    fn test_clz(x in u64::MIN..=u64::MAX) {
        let r0 = E64::from(x).clz();
        let r1 = T64::recv(x).clz();
        assert_eq!(r0, r1);
        assert_eq!(r0, x.leading_zeros());
    }

    #[test]
    fn test_cmp_s(x in u64::MIN..=u64::MAX, y in u64::MIN..=u64::MAX) {
        let r0 = Eint::cmp_s(&E64::from(x), &E64::from(y));
        let r1 = Eint::cmp_s(&T64::recv(x), &T64::recv(y));
        let r2 = (x as i64).cmp(&(y as i64));
        assert_eq!(r0, r1);
        assert_eq!(r0, r2);
    }

    #[test]
    fn test_cmp_u(x in u64::MIN..=u64::MAX, y in u64::MIN..=u64::MAX) {
        let r0 = Eint::cmp_u(&E64::from(x), &E64::from(y));
        let r1 = Eint::cmp_u(&T64::recv(x), &T64::recv(y));
        let r2 = x.cmp(&y);
        assert_eq!(r0, r1);
        assert_eq!(r0, r2);
    }

    #[test]
    fn test_cpop(x in u64::MIN..=u64::MAX) {
        let r0 = E64::from(x).cpop();
        let r1 = T64::recv(x).cpop();
        assert_eq!(r0, r1);
        assert_eq!(r0, x.count_ones());
    }

    #[test]
    fn test_ctz(x in u64::MIN..=u64::MAX) {
        let r0 = E64::from(x).ctz();
        let r1 = T64::recv(x).ctz();
        assert_eq!(r0, r1);
        assert_eq!(r0, x.trailing_zeros());
    }

    #[test]
    fn test_is_negative(x in u64::MIN..=u64::MAX) {
        let r0 = E64::from(x).is_negative();
        let r1 = T64::recv(x).is_negative();
        assert_eq!(r0, r1);
    }

    #[test]
    fn test_is_positive(x in u64::MIN..=u64::MAX) {
        let r0 = E64::from(x).is_positive();
        let r1 = T64::recv(x).is_positive();
        assert_eq!(r0, r1);
        assert_eq!(r0, (x as i64).is_positive());
    }

    #[test]
    fn test_overflowing_add_s(x in u64::MIN..=u64::MAX, y in u64::MIN..=u64::MAX) {
        let (r0, b0) = Eint::overflowing_add_s(E64::from(x), E64::from(y));
        let (r1, b1) = Eint::overflowing_add_s(T64::recv(x), T64::recv(y));
        assert_eq!(r0, r1.into());
        assert_eq!(b0, b1);
    }

    #[test]
    fn test_overflowing_add_u(x in u64::MIN..=u64::MAX, y in u64::MIN..=u64::MAX) {
        let (r0, b0) = Eint::overflowing_add_u(E64::from(x), E64::from(y));
        let (r1, b1) = Eint::overflowing_add_u(T64::recv(x), T64::recv(y));
        assert_eq!(r0, r1.into());
        assert_eq!(b0, b1);
    }

    #[test]
    fn test_overflowing_mul_s(x in u64::MIN..=u64::MAX, y in u64::MIN..=u64::MAX) {
        let (r0, b0) = Eint::overflowing_mul_s(E64::from(x), E64::from(y));
        let (r1, b1) = Eint::overflowing_mul_s(T64::recv(x), T64::recv(y));
        let r2 = (x as i64 as i128).wrapping_mul(y as i64 as i128) as u128;
        assert_eq!(r0, r1.into());
        assert_eq!(b0, b1);
        assert_eq!(r0, E64(r2 as u64));
        assert_eq!(b0, r2 > u64::MAX as u128);
    }

    #[test]
    fn test_overflowing_mul_u(x in u64::MIN..=u64::MAX, y in u64::MIN..=u64::MAX) {
        let (r0, b0) = Eint::overflowing_mul_u(E64::from(x), E64::from(y));
        let (r1, b1) = Eint::overflowing_mul_u(T64::recv(x), T64::recv(y));
        let (r2, b2) = x.overflowing_mul(y);
        assert_eq!(r0, r1.into());
        assert_eq!(b0, b1);
        assert_eq!(r0, E64(r2));
        assert_eq!(b0, b2);
    }

    #[test]
    fn test_overflowing_sub_s(x in u64::MIN..=u64::MAX, y in u64::MIN..=u64::MAX) {
        let (r0, b0) = Eint::overflowing_sub_s(E64::from(x), E64::from(y));
        let (r1, b1) = Eint::overflowing_sub_s(T64::recv(x), T64::recv(y));
        assert_eq!(r0, r1.into());
        assert_eq!(b0, b1);
    }

    #[test]
    fn test_overflowing_sub_u(x in u64::MIN..=u64::MAX, y in u64::MIN..=u64::MAX) {
        let (r0, b0) = Eint::overflowing_sub_u(E64::from(x), E64::from(y));
        let (r1, b1) = Eint::overflowing_sub_u(T64::recv(x), T64::recv(y));
        assert_eq!(r0, r1.into());
        assert_eq!(b0, b1);
    }

    #[test]
    fn test_saturating_add_s(x in u64::MIN..=u64::MAX, y in u64::MIN..=u64::MAX){
        let (r0, b0) = Eint::saturating_add_s(E64::from(x), E64::from(y));
        let (r1, b1) = Eint::saturating_add_s(T64::recv(x), T64::recv(y));
        assert_eq!(r0, r1.into());
        assert_eq!(b0, b1);
        assert_eq!(r0, E64((x as i64).saturating_add(y as i64) as u64))
    }

    #[test]
    fn test_saturating_add_u(x in u64::MIN..=u64::MAX, y in u64::MIN..=u64::MAX){
        let (r0, b0) = Eint::saturating_add_u(E64::from(x), E64::from(y));
        let (r1, b1) = Eint::saturating_add_u(T64::recv(x), T64::recv(y));
        assert_eq!(r0, r1.into());
        assert_eq!(b0, b1);
        assert_eq!(r0, E64(x.saturating_add(y)))
    }

    #[test]
    fn test_saturating_sub_s(x in u64::MIN..=u64::MAX, y in u64::MIN..=u64::MAX){
        let (r0, b0) = Eint::saturating_sub_s(E64::from(x), E64::from(y));
        let (r1, b1) = Eint::saturating_sub_s(T64::recv(x), T64::recv(y));
        assert_eq!(r0, r1.into());
        assert_eq!(b0, b1);
        assert_eq!(r0, E64((x as i64).saturating_sub(y as i64) as u64))
    }

    #[test]
    fn test_saturating_sub_u(x in u64::MIN..=u64::MAX, y in u64::MIN..=u64::MAX){
        let (r0, b0) = Eint::saturating_sub_u(E64::from(x), E64::from(y));
        let (r1, b1) = Eint::saturating_sub_u(T64::recv(x), T64::recv(y));
        assert_eq!(r0, r1.into());
        assert_eq!(b0, b1);
        assert_eq!(r0, E64(x.saturating_sub(y)))
    }

    #[test]
    fn test_widdening_add_s(x in u64::MIN..=u64::MAX, y in u64::MIN..=u64::MAX) {
        let r0 = Eint::widening_add_s(E64::from(x), E64::from(y));
        let r1 = Eint::widening_add_s(T64::recv(x), T64::recv(y));
        let r2 = x as i64 as i128 + y as i64 as i128;
        assert_eq!(r0.0, r1.0.into());
        assert_eq!(r0.1, r1.1.into());
        assert_eq!(r0.0, E64(r2 as u128 as u64));
        assert_eq!(r0.1, E64((r2 as u128 >> 64) as u64));
    }

    #[test]
    fn test_widdening_add_u(x in u64::MIN..=u64::MAX, y in u64::MIN..=u64::MAX) {
        let r0 = Eint::widening_add_u(E64::from(x), E64::from(y));
        let r1 = Eint::widening_add_u(T64::recv(x), T64::recv(y));
        let r2 = x as u128 + y as u128;
        assert_eq!(r0.0, r1.0.into());
        assert_eq!(r0.1, r1.1.into());
        assert_eq!(r0.0, E64(r2 as u64));
        assert_eq!(r0.1, E64((r2 >> 64) as u64));
    }

    #[test]
    fn test_widdening_mul_s(x in u64::MIN..=u64::MAX, y in u64::MIN..=u64::MAX) {
        let r0 = Eint::widening_mul_s(E64::from(x), E64::from(y));
        let r1 = Eint::widening_mul_s(T64::recv(x), T64::recv(y));
        let r2 = (x as i64 as i128 * y as i64 as i128) as u128;
        assert_eq!(r0.0, r1.0.into());
        assert_eq!(r0.1, r1.1.into());
        assert_eq!(r0.0, E64(r2 as u64));
        assert_eq!(r0.1, E64((r2 >> 64) as u64));
    }

    #[test]
    fn test_widdening_mul_su(x in u64::MIN..=u64::MAX, y in u64::MIN..=u64::MAX) {
        let r0 = Eint::widening_mul_su(E64::from(x), E64::from(y));
        let r1 = Eint::widening_mul_su(T64::recv(x), T64::recv(y));
        let r2 = (x as i64 as i128 * y as u128 as i128) as u128;
        assert_eq!(r0.0, r1.0.into());
        assert_eq!(r0.1, r1.1.into());
        assert_eq!(r0.0, E64(r2 as u64));
        assert_eq!(r0.1, E64((r2 >> 64) as u64));
    }

    #[test]
    fn test_widdening_mul_u(x in u64::MIN..=u64::MAX, y in u64::MIN..=u64::MAX) {
        let r0 = Eint::widening_mul_u(E64::from(x), E64::from(y));
        let r1 = Eint::widening_mul_u(T64::recv(x), T64::recv(y));
        let r2 = x as u128 * y as u128;
        assert_eq!(r0.0, r1.0.into());
        assert_eq!(r0.1, r1.1.into());
        assert_eq!(r0.0, E64(r2 as u64));
        assert_eq!(r0.1, E64((r2 >> 64) as u64));
    }

    #[test]
    fn test_widdening_sub_s(x in u64::MIN..=u64::MAX, y in u64::MIN..=u64::MAX) {
        let r0 = Eint::widening_sub_s(E64::from(x), E64::from(y));
        let r1 = Eint::widening_sub_s(T64::recv(x), T64::recv(y));
        let r2 = (x as i64 as i128 - y as i64 as i128) as u128;
        assert_eq!(r0.0, r1.0.into());
        assert_eq!(r0.1, r1.1.into());
        assert_eq!(r0.0, E64(r2 as u64));
        assert_eq!(r0.1, E64((r2 >> 64) as u64));
    }

    #[test]
    fn test_widdening_sub_u(x in u64::MIN..=u64::MAX, y in u64::MIN..=u64::MAX) {
        let r0 = Eint::widening_sub_u(E64::from(x), E64::from(y));
        let r1 = Eint::widening_sub_u(T64::recv(x), T64::recv(y));
        let r2 = (x as u128).wrapping_sub(y as u128);
        assert_eq!(r0.0, r1.0.into());
        assert_eq!(r0.1, r1.1.into());
        assert_eq!(r0.0, E64(r2 as u64));
        assert_eq!(r0.1, E64((r2 >> 64) as u64));
    }

    #[test]
    fn test_wrapping_add(x in u64::MIN..=u64::MAX, y in u64::MIN..=u64::MAX) {
        let r0 = Eint::wrapping_add(E64::from(x), E64::from(y));
        let r1 = Eint::wrapping_add(T64::recv(x), T64::recv(y));
        assert_eq!(r0, r1.into());
    }

    #[test]
    fn test_wrapping_div_s(x in u64::MIN..=u64::MAX, y in u64::MIN..=u64::MAX) {
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
    fn test_wrapping_div_u(x in u64::MIN..=u64::MAX, y in u64::MIN..=u64::MAX) {
        let r0 = Eint::wrapping_div_u(E64::from(x), E64::from(y));
        let r1 = Eint::wrapping_div_u(T64::recv(x), T64::recv(y));
        let r2 = if y == 0 { u64::MAX } else { x / y };
        assert_eq!(r0, r1.into());
        assert_eq!(r0, E64(r2));
    }

    #[test]
    fn test_wrapping_mul(x in u64::MIN..=u64::MAX, y in u64::MIN..=u64::MAX) {
        let r0 = Eint::wrapping_mul(E64::from(x), E64::from(y));
        let r1 = Eint::wrapping_mul(T64::recv(x), T64::recv(y));
        assert_eq!(r0, r1.into());
    }

    #[test]
    fn test_wrapping_rem_s(x in u64::MIN..=u64::MAX, y in u64::MIN..=u64::MAX) {
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
    fn test_wrapping_rem_u(x in u64::MIN..=u64::MAX, y in u64::MIN..=u64::MAX) {
        let r0 = Eint::wrapping_rem_u(E64::from(x), E64::from(y));
        let r1 = Eint::wrapping_rem_u(T64::recv(x), T64::recv(y));
        let r2 = if y == 0 { 0 } else { x % y };
        assert_eq!(r0, r1.into());
        assert_eq!(r0, E64(r2));
    }

    #[test]
    fn test_wrapping_shl(x in u64::MIN..=u64::MAX, y in u32::MIN..=u32::MAX) {
        let r0 = E64::from(x).wrapping_shl(y);
        let r1 = T64::recv(x).wrapping_shl(y);
        assert_eq!(r0, r1.into());
    }

    #[test]
    fn test_wrapping_shr(x in u64::MIN..=u64::MAX, y in u32::MIN..=u32::MAX) {
        let r0 = E64::from(x).wrapping_shr(y);
        let r1 = T64::recv(x).wrapping_shr(y);
        assert_eq!(r0, r1.into());
    }

    #[test]
    fn test_wrapping_sra(x in u64::MIN..=u64::MAX, y in u32::MIN..=u32::MAX) {
        let r0 = E64::from(x).wrapping_sra(y);
        let r1 = T64::recv(x).wrapping_sra(y);
        let r2 = (x as i64).wrapping_shr(y) as u64;
        assert_eq!(r0, r1.into());
        assert_eq!(r0, E64(r2));
    }

    #[test]
    fn test_wrapping_sub(x in u64::MIN..=u64::MAX, y in u64::MIN..=u64::MAX) {
        let r0 = Eint::wrapping_sub(E64::from(x), E64::from(y));
        let r1 = Eint::wrapping_sub(T64::recv(x), T64::recv(y));
        assert_eq!(r0, r1.into());
    }
}
