use eint::*;
use proptest::prelude::*;

construct_eint_twin!(T64, E32);

impl T64 {
    fn recv(small: u64) -> Self {
        Self(E32(small as u32), E32((small >> 32) as u32))
    }

    fn into(self) -> E64 {
        E64(((self.1 .0 as u64) << 32) | (self.0 .0 as u64))
    }
}

proptest! {
    #[test]
    fn test_is_negative(x in u64::MIN..=u64::MAX) {
        let r0 = E64::from(x).is_negative();
        let r1 = T64::recv(x).is_negative();
        assert_eq!(r0, r1);
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
    fn test_wrapping_add(x in u64::MIN..=u64::MAX, y in u64::MIN..=u64::MAX) {
        let r0 = Eint::wrapping_add(E64::from(x), E64::from(y));
        let r1 = Eint::wrapping_add(T64::recv(x), T64::recv(y));
        assert_eq!(r0, r1.into());
    }
}
