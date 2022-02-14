pub trait Eint: Clone + Copy + Eq + PartialEq + std::fmt::Debug {
    const BITS: u32;
    const MAX_U: Self;
    const MIN_U: Self;

    fn is_negative(self) -> bool;
    fn overflowing_add_s(self, other: Self) -> (Self, bool);
    fn overflowing_add_u(self, other: Self) -> (Self, bool);
    fn wrapping_add(self, other: Self) -> Self;
}

#[macro_export]
macro_rules! construct_eint_wrap_from_uint {
    ($name:ident, $uint:ty, $from:ty) => {
        impl std::convert::From<$from> for $name {
            fn from(small: $from) -> Self {
                Self(small as $uint)
            }
        }
    };
}

#[macro_export]
macro_rules! construct_eint_wrap {
    ($name:ident, $uint:ty, $sint:ty) => {
        #[derive(Clone, Copy, Eq, PartialEq)]
        pub struct $name(pub $uint);

        construct_eint_wrap_from_uint!($name, $uint, bool);
        construct_eint_wrap_from_uint!($name, $uint, u8);
        construct_eint_wrap_from_uint!($name, $uint, u16);
        construct_eint_wrap_from_uint!($name, $uint, u32);
        construct_eint_wrap_from_uint!($name, $uint, u64);
        construct_eint_wrap_from_uint!($name, $uint, u128);

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let suffix = format!("{:x}", self.0);
                let prefix = String::from("0").repeat(Self::BITS as usize / 4 - suffix.len());
                write!(f, "{}", prefix)?;
                write!(f, "{}", suffix)
            }
        }

        impl Eint for $name {
            const BITS: u32 = <$uint>::BITS;
            const MIN_U: Self = Self(0);
            const MAX_U: Self = Self(<$uint>::MAX);

            fn is_negative(self) -> bool {
                (self.0 as $sint).is_negative()
            }

            fn overflowing_add_s(self, other: Self) -> (Self, bool) {
                let (r, carry) = (self.0 as $sint).overflowing_add(other.0 as $sint);
                (Self(r as $uint), carry)
            }

            fn overflowing_add_u(self, other: Self) -> (Self, bool) {
                let (r, carry) = self.0.overflowing_add(other.0);
                (Self(r), carry)
            }

            fn wrapping_add(self, other: Self) -> Self {
                Self(self.0.wrapping_add(other.0))
            }
        }
    };
}

#[macro_export]
macro_rules! construct_eint_twin_from_uint {
    ($name:ident, $half:ty, $from:ty) => {
        impl std::convert::From<$from> for $name {
            fn from(small: $from) -> Self {
                debug_assert!($name::BITS >= 256);
                Self(<$half>::from(small), <$half>::MIN_U)
            }
        }
    };
}

#[macro_export]
macro_rules! construct_eint_twin {
    ($name:ident, $half:ty) => {
        #[derive(Clone, Copy, Eq, PartialEq)]
        pub struct $name(pub $half, pub $half);

        construct_eint_twin_from_uint!($name, $half, bool);
        construct_eint_twin_from_uint!($name, $half, u8);
        construct_eint_twin_from_uint!($name, $half, u16);
        construct_eint_twin_from_uint!($name, $half, u32);
        construct_eint_twin_from_uint!($name, $half, u64);
        construct_eint_twin_from_uint!($name, $half, u128);

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}{:?}", self.1, self.0)
            }
        }

        impl Eint for $name {
            const BITS: u32 = <$half>::BITS * 2;
            const MIN_U: Self = Self(<$half>::MIN_U, <$half>::MIN_U);
            const MAX_U: Self = Self(<$half>::MAX_U, <$half>::MAX_U);

            fn is_negative(self) -> bool {
                self.1.is_negative()
            }

            fn overflowing_add_s(self, other: Self) -> (Self, bool) {
                let r = self.wrapping_add(other);
                if self.is_negative() == other.is_negative() {
                    (r, r.is_negative() != self.is_negative())
                } else {
                    (r, false)
                }
            }

            fn overflowing_add_u(self, other: Self) -> (Self, bool) {
                let (lo, lo_carry) = self.0.overflowing_add_u(other.0);
                let (hi, hi_carry_0) = self.1.overflowing_add_u(<$half>::from(lo_carry));
                let (hi, hi_carry_1) = hi.overflowing_add_u(other.1);
                (Self(lo, hi), hi_carry_0 || hi_carry_1)
            }

            fn wrapping_add(self, other: Self) -> Self {
                let (lo, carry) = self.0.overflowing_add_u(other.0);
                let hi = self.1.wrapping_add(other.1).wrapping_add(<$half>::from(carry));
                Self(lo, hi)
            }
        }
    };
}

construct_eint_wrap!(E8, u8, i8);
construct_eint_wrap!(E16, u16, i16);
construct_eint_wrap!(E32, u32, i32);
construct_eint_wrap!(E64, u64, i64);
construct_eint_wrap!(E128, u128, i128);
construct_eint_twin!(E256, E128);
construct_eint_twin!(E512, E256);
construct_eint_twin!(E1024, E512);
construct_eint_twin!(E2048, E1024);
