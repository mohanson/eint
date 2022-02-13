pub trait Eint: Clone + Copy + From<bool> {
    const MIN_U: Self;
    const MAX_U: Self;
    fn overflowing_add_u(self, other: Self) -> (Self, bool);
}

#[macro_export]
macro_rules! construct_eint_wrap {
    ($name:ident, $uint:ty, $sint:ty) => {
        #[derive(Clone, Copy)]
        pub struct $name(pub $uint);

        impl std::convert::From<bool> for $name {
            fn from(small: bool) -> Self {
                Self(small as $uint)
            }
        }

        impl Eint for $name {
            const MIN_U: Self = Self(0);
            const MAX_U: Self = Self(<$uint>::MAX);

            fn overflowing_add_u(self, other: Self) -> (Self, bool) {
                let (r, b) = self.0.overflowing_add(other.0);
                (Self(r), b)
            }
        }
    };
}

#[macro_export]
macro_rules! construct_eint_twin {
    ($name:ident, $half:ty) => {
        #[derive(Clone, Copy)]
        pub struct $name(pub $half, pub $half);

        impl std::convert::From<bool> for $name {
            fn from(small: bool) -> Self {
                Self(<$half>::from(small), <$half>::MIN_U)
            }
        }

        impl Eint for $name {
            const MIN_U: Self = Self(<$half>::MIN_U, <$half>::MIN_U);
            const MAX_U: Self = Self(<$half>::MAX_U, <$half>::MAX_U);

            fn overflowing_add_u(self, other: Self) -> (Self, bool) {
                let (lo, lo_carry) = self.0.overflowing_add_u(other.0);
                let (hi, hi_carry_0) = self.1.overflowing_add_u(<$half>::from(lo_carry));
                let (hi, hi_carry_1) = hi.overflowing_add_u(other.1);
                (Self(lo, hi), hi_carry_0 || hi_carry_1)
            }
        }
    };
}

construct_eint_wrap!(E8, u8, i8);
construct_eint_wrap!(E16, u16, i16);
construct_eint_wrap!(E32, u32, i32);
construct_eint_wrap!(E64, u64, i64);
construct_eint_wrap!(E128, u128, i128);
construct_eint_twin!(T128, E64);
construct_eint_twin!(E256, E128);
construct_eint_twin!(E512, E256);
construct_eint_twin!(E1024, E512);
construct_eint_twin!(E2048, E1024);
