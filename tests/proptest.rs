use proptest::prelude::*;

proptest! {
    #[test]
    fn test_overflowing_add_u(x in u128::MIN..=u128::MAX, y in u128::MIN..=u128::MAX) {
        let e0 = E128::from(x)
    }
}
