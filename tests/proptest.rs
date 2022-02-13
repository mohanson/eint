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
    fn test_overflowing_add_u(x in u64::MIN..=u64::MAX, y in u64::MIN..=u64::MAX) {
        let e0 = Eint::overflowing_add_u(E64::from(x), E64::from(y));
        let e1 = Eint::overflowing_add_u(T64::recv(x), T64::recv(y));
        assert_eq!(e0.0, e1.0.into());
        assert_eq!(e0.1, e1.1);
    }
}
