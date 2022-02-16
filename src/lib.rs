pub trait Eint:
    Clone
    + Copy
    + Default
    + Eq
    + From<bool>
    + From<i8>
    + From<i16>
    + From<i32>
    + From<i64>
    + From<i128>
    + From<u8>
    + From<u16>
    + From<u32>
    + From<u64>
    + From<u128>
    + PartialEq
    + std::fmt::Debug
    + std::fmt::Display
    + std::fmt::LowerHex
    + std::ops::BitAnd<Output = Self>
    + std::ops::BitAndAssign
    + std::ops::BitOr<Output = Self>
    + std::ops::BitOrAssign
    + std::ops::BitXor<Output = Self>
    + std::ops::BitXorAssign
    + std::ops::Neg<Output = Self>
    + std::ops::Not
    + std::cmp::Ord
    + std::cmp::PartialOrd
    + std::ops::Add<Output = Self>
    + std::ops::AddAssign
    + std::ops::Div<Output = Self>
    + std::ops::DivAssign
    + std::ops::Mul<Output = Self>
    + std::ops::MulAssign
    + std::ops::Rem<Output = Self>
    + std::ops::RemAssign
    + std::ops::Sub<Output = Self>
    + std::ops::SubAssign
    + std::ops::Shl<u32, Output = Self>
    + std::ops::Shr<u32, Output = Self>
{
    const BITS: u32;
    const MAX_S: Self;
    const MAX_U: Self;
    const MIN_S: Self;
    const MIN_U: Self;
    const ONE: Self;
    const ZERO: Self;

    /// Returns (self + rhs) >> 1.
    fn average_add_s(self, other: Self) -> Self {
        (self & other).wrapping_add((self ^ other).wrapping_sra(1))
    }

    /// Returns (self + rhs) >> 1.
    fn average_add_u(self, other: Self) -> Self {
        (self & other).wrapping_add((self ^ other).wrapping_shr(1))
    }

    /// Returns (self - rhs) >> 1.
    fn average_sub_s(self, other: Self) -> Self {
        let (lo, borrow) = self.overflowing_sub_u(other);
        let hi_0 = if !self.is_negative() { Self::MIN_U } else { Self::MAX_U };
        let hi_1 = if !other.is_negative() { Self::MIN_U } else { Self::MAX_U };
        let hi = hi_0.wrapping_sub(hi_1).wrapping_sub(Self::from(borrow));
        lo.wrapping_shr(1) | hi.wrapping_shl(1).wrapping_shl(Self::BITS - 2)
    }

    /// Returns (self - rhs) >> 1.
    fn average_sub_u(self, other: Self) -> Self {
        let (lo, borrow) = self.overflowing_sub_u(other);
        if borrow {
            (lo >> 1) | (Self::ONE << (Self::BITS - 1))
        } else {
            lo >> 1
        }
    }

    /// Returns the number of leading zeros in the binary representation of self.
    fn clz(self) -> u32;

    /// Compare signed.
    fn cmp_s(&self, other: &Self) -> std::cmp::Ordering;

    /// Compare.
    fn cmp_u(&self, other: &Self) -> std::cmp::Ordering;

    /// Returns the number of ones in the binary representation of self.
    fn cpop(self) -> u32;

    /// Returns the number of trailing zeros in the binary representation of self.
    fn ctz(self) -> u32;

    /// Get a native endian integer value from its representation as a byte slice in little endian.
    fn get(mem: &[u8]) -> Self;

    /// Returns the higher part.
    fn hi(self) -> Self;

    /// Returns true if highest bit is set.
    fn is_negative(self) -> bool;

    /// Returns true if highest bit is not set.
    fn is_positive(self) -> bool;

    /// Returns the lower part.
    fn lo(self) -> Self;

    /// Returns the lower part and sign extend it.
    fn lo_sext(self) -> Self;

    /// Calculates self + rhs.
    fn overflowing_add_s(self, other: Self) -> (Self, bool);

    /// Calculates self + rhs.
    fn overflowing_add_u(self, other: Self) -> (Self, bool);

    /// Calculates self * rhs.
    fn overflowing_mul_s(self, other: Self) -> (Self, bool);

    /// Calculates self * rhs.
    fn overflowing_mul_u(self, other: Self) -> (Self, bool);

    /// Calculates self - rhs.
    fn overflowing_sub_s(self, other: Self) -> (Self, bool);

    /// Calculates self - rhs.
    fn overflowing_sub_u(self, other: Self) -> (Self, bool);

    /// Save the integer as a byte array in little-endian byte order to memory.
    fn put(&self, mem: &mut [u8]);

    /// Put the lower part integer as a byte array in little-endian byte order to memory.
    fn put_lo(&self, mem: &mut [u8]);

    /// Saturating integer addition. Computes self + rhs, saturating at the numeric bounds instead of overflowing.
    fn saturating_add_s(self, other: Self) -> (Self, bool) {
        let r = self.wrapping_add(other);
        if !(self ^ other).is_negative() {
            if (r ^ self).is_negative() {
                let r = if self.is_negative() { Self::MIN_S } else { Self::MAX_S };
                return (r, true);
            }
        }
        (r, false)
    }

    /// Saturating integer addition. Computes self + rhs, saturating at the numeric bounds instead of overflowing.
    fn saturating_add_u(self, other: Self) -> (Self, bool) {
        let (r, overflow) = self.overflowing_add_u(other);
        if overflow {
            (Self::MAX_U, overflow)
        } else {
            (r, overflow)
        }
    }

    /// Saturating integer subtraction. Computes self - rhs, saturating at the numeric bounds instead of overflowing.
    fn saturating_sub_s(self, other: Self) -> (Self, bool) {
        let r = self.wrapping_sub(other);
        if (self ^ other).is_negative() {
            if (r ^ self).is_negative() {
                let r = if self.is_negative() { Self::MIN_S } else { Self::MAX_S };
                return (r, true);
            }
        }
        (r, false)
    }

    /// Saturating integer subtraction. Computes self - rhs, saturating at the numeric bounds instead of overflowing.
    fn saturating_sub_u(self, other: Self) -> (Self, bool) {
        if self > other {
            (self.wrapping_sub(other), false)
        } else {
            (Self::MIN_U, true)
        }
    }

    /// Sign extended.
    fn sext(self, other: u32) -> Self {
        self.wrapping_shl(Self::BITS - other - 1).wrapping_sra(Self::BITS - other - 1)
    }

    /// Returns the lower 8 bits.
    fn u8(self) -> u8;

    /// Returns the lower 16 bits.
    fn u16(self) -> u16;

    /// Returns the lower 32 bits.
    fn u32(self) -> u32;

    /// Returns the lower 64 bits.
    fn u64(self) -> u64;

    /// Signed widening add.
    fn widening_add_s(self, other: Self) -> (Self, Self) {
        let hi_0 = if self.is_negative() { Self::MAX_U } else { Self::MIN_U };
        let hi_1 = if other.is_negative() { Self::MAX_U } else { Self::MIN_U };
        let (lo, carry) = self.overflowing_add_u(other);
        let hi = hi_0.wrapping_add(hi_1).wrapping_add(Self::from(carry));
        (lo, hi)
    }

    /// Widening add.
    fn widening_add_u(self, other: Self) -> (Self, Self) {
        let (lo, carry) = self.overflowing_add_u(other);
        (lo, Self::from(carry))
    }

    /// Signed interger widening multiple.
    ///
    /// Inspired by https://sqlite.in/?qa=668884/c-32-bit-signed-integer-multiplication-without-using-64-bit-data-type
    fn widening_mul_s(self, other: Self) -> (Self, Self) {
        let (lo, hi) = self.widening_mul_u(other);
        let hi = hi
            - if self.is_negative() { other } else { Self::MIN_U }
            - if other.is_negative() { self } else { Self::MIN_U };
        (lo, hi)
    }

    /// Widening signed and unsigned integer multiply.
    fn widening_mul_su(self, other: Self) -> (Self, Self) {
        if !other.is_negative() {
            self.widening_mul_s(other)
        } else {
            let (lo, hi) = self.widening_mul_s(other);
            let hi = hi + self;
            (lo, hi)
        }
    }

    /// Function widening_mul returns the product of x and y: (lo, hi) = x * y
    /// with the product bits' upper half returned in hi and the lower half returned in lo.
    ///
    /// See https://pkg.go.dev/math/bits@go1.17.2#Mul64
    fn widening_mul_u(self, other: Self) -> (Self, Self) {
        let x0 = self.lo();
        let x1 = self.hi();
        let y0 = other.lo();
        let y1 = other.hi();
        let w0 = x0.wrapping_mul(y0);
        let t = x1.wrapping_mul(y0).wrapping_add(w0.hi());
        let w1 = t.lo();
        let w2 = t.hi();
        let w1 = x0.wrapping_mul(y1).wrapping_add(w1);
        let hi = x1.wrapping_mul(y1).wrapping_add(w2).wrapping_add(w1.hi());
        let lo = self.wrapping_mul(other);
        (lo, hi)
    }

    /// Signed widening substract.
    fn widening_sub_s(self, other: Self) -> (Self, Self) {
        let hi_0 = if self.is_negative() { Self::MAX_U } else { Self::MIN_U };
        let hi_1 = if other.is_negative() { Self::MAX_U } else { Self::MIN_U };
        let (lo, borrow) = self.overflowing_sub_u(other);
        let hi = hi_0.wrapping_sub(hi_1).wrapping_sub(Self::from(borrow));
        (lo, hi)
    }

    /// Widening substract.
    fn widening_sub_u(self, other: Self) -> (Self, Self) {
        let (lo, borrow) = self.overflowing_sub_u(other);
        (lo, if borrow { Self::MAX_U } else { Self::MIN_U })
    }

    /// Wrapping (modular) addition. Computes self + rhs, wrapping around at the boundary of the type.
    fn wrapping_add(self, other: Self) -> Self;

    /// Wrapping (modular) division. Computes self / rhs.
    /// 1) x / 0 = MAX_U
    fn wrapping_div_u(self, other: Self) -> Self;

    /// Wrapping (modular) division signed.
    /// 1) x / 0 = -1.
    /// 2) MIN_S / -1 = MIN_S
    fn wrapping_div_s(self, other: Self) -> Self;

    /// Wrapping (modular) multiplication. Computes self * rhs, wrapping around at the boundary of the type.
    fn wrapping_mul(self, other: Self) -> Self;

    /// Wrapping (modular) remainder signed.
    /// 1) x % 0 = x
    /// 2) MIN_S % -1 = 0
    fn wrapping_rem_s(self, other: Self) -> Self;

    /// Wrapping (modular) remainder.
    /// 1) x % 0 = x
    fn wrapping_rem_u(self, other: Self) -> Self;

    /// Panic-free bitwise shift-left; yields self << mask(rhs), where mask removes any high-order bits of rhs that
    /// would cause the shift to exceed the bitwidth of the type.
    fn wrapping_shl(self, other: u32) -> Self;

    /// Panic-free bitwise shift-right; yields self >> mask(rhs), where mask removes any high-order bits of rhs that
    /// would cause the shift to exceed the bitwidth of the type.
    fn wrapping_shr(self, other: u32) -> Self;

    /// Panic-free bitwise sign shift-right.
    fn wrapping_sra(self, other: u32) -> Self;

    /// Wrapping (modular) subtraction. Computes self - rhs, wrapping around at the boundary of the type.
    fn wrapping_sub(self, other: Self) -> Self;

    /// Zero extended.
    fn zext(self, other: u32) -> Self {
        self.wrapping_shl(Self::BITS - other - 1).wrapping_shr(Self::BITS - other - 1)
    }
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
        #[derive(Copy, Clone, Default, PartialEq, Eq)]
        pub struct $name(pub $uint);

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let suffix = format!("{:x}", self.0);
                let prefix = String::from("0").repeat(Self::BITS as usize / 4 - suffix.len());
                write!(f, "{}", prefix)?;
                write!(f, "{}", suffix)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let suffix = format!("{:x}", self.0);
                let prefix = String::from("0").repeat(Self::BITS as usize / 4 - suffix.len());
                write!(f, "{}", prefix)?;
                write!(f, "{}", suffix)
            }
        }

        impl std::fmt::LowerHex for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let suffix = format!("{:x}", self.0);
                let prefix = String::from("0").repeat(Self::BITS as usize / 4 - suffix.len());
                write!(f, "{}", prefix)?;
                write!(f, "{}", suffix)
            }
        }

        construct_eint_wrap_from_uint!($name, $uint, bool);
        construct_eint_wrap_from_uint!($name, $uint, i8);
        construct_eint_wrap_from_uint!($name, $uint, i16);
        construct_eint_wrap_from_uint!($name, $uint, i32);
        construct_eint_wrap_from_uint!($name, $uint, i64);
        construct_eint_wrap_from_uint!($name, $uint, i128);
        construct_eint_wrap_from_uint!($name, $uint, u8);
        construct_eint_wrap_from_uint!($name, $uint, u16);
        construct_eint_wrap_from_uint!($name, $uint, u32);
        construct_eint_wrap_from_uint!($name, $uint, u64);
        construct_eint_wrap_from_uint!($name, $uint, u128);

        impl std::ops::BitAnd for $name {
            type Output = Self;
            fn bitand(self, other: Self) -> Self::Output {
                Self(self.0 & other.0)
            }
        }

        impl std::ops::BitAndAssign for $name {
            fn bitand_assign(&mut self, other: Self) {
                self.0 &= other.0
            }
        }

        impl std::ops::BitOr for $name {
            type Output = Self;
            fn bitor(self, other: Self) -> Self::Output {
                Self(self.0 | other.0)
            }
        }

        impl std::ops::BitOrAssign for $name {
            fn bitor_assign(&mut self, other: Self) {
                self.0 |= other.0
            }
        }

        impl std::ops::BitXor for $name {
            type Output = Self;
            fn bitxor(self, other: Self) -> Self::Output {
                Self(self.0 ^ other.0)
            }
        }

        impl std::ops::BitXorAssign for $name {
            fn bitxor_assign(&mut self, other: Self) {
                self.0 ^= other.0
            }
        }

        impl std::ops::Not for $name {
            type Output = Self;
            fn not(self) -> Self::Output {
                Self(!self.0)
            }
        }

        impl std::ops::Neg for $name {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self((!self.0).wrapping_add(1))
            }
        }

        impl std::cmp::PartialOrd for $name {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                return self.0.partial_cmp(&other.0);
            }
        }

        impl std::cmp::Ord for $name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.0.cmp(&other.0)
            }
        }

        impl std::ops::Add for $name {
            type Output = Self;
            fn add(self, other: Self) -> Self {
                Self(self.0.wrapping_add(other.0))
            }
        }

        impl std::ops::AddAssign for $name {
            fn add_assign(&mut self, other: Self) {
                self.0 = self.0.wrapping_add(other.0)
            }
        }

        impl std::ops::Sub for $name {
            type Output = Self;
            fn sub(self, other: Self) -> Self::Output {
                Self(self.0.wrapping_sub(other.0))
            }
        }

        impl std::ops::SubAssign for $name {
            fn sub_assign(&mut self, other: Self) {
                self.0 = self.0.wrapping_sub(other.0)
            }
        }

        impl std::ops::Mul for $name {
            type Output = Self;
            fn mul(self, other: Self) -> Self::Output {
                Self(self.0.wrapping_mul(other.0))
            }
        }

        impl std::ops::MulAssign for $name {
            fn mul_assign(&mut self, other: Self) {
                self.0 = self.0.wrapping_mul(other.0)
            }
        }

        impl std::ops::Div for $name {
            type Output = Self;
            fn div(self, other: Self) -> Self::Output {
                if other.0 == 0 {
                    Self::MAX_U
                } else {
                    Self(self.0.wrapping_div(other.0))
                }
            }
        }

        impl std::ops::DivAssign for $name {
            fn div_assign(&mut self, other: Self) {
                self.0 = if other.0 == 0 {
                    <$uint>::MAX
                } else {
                    self.0.wrapping_div(other.0)
                }
            }
        }

        impl std::ops::Rem for $name {
            type Output = Self;
            fn rem(self, other: Self) -> Self::Output {
                if other.0 == 0 {
                    self
                } else {
                    Self(self.0.wrapping_rem(other.0))
                }
            }
        }

        impl std::ops::RemAssign for $name {
            fn rem_assign(&mut self, other: Self) {
                self.0 = if other.0 == 0 {
                    self.0
                } else {
                    self.0.wrapping_rem(other.0)
                }
            }
        }

        impl std::ops::Shl<u32> for $name {
            type Output = Self;
            fn shl(self, other: u32) -> Self::Output {
                Self(self.0.wrapping_shl(other))
            }
        }

        impl std::ops::Shr<u32> for $name {
            type Output = Self;
            fn shr(self, other: u32) -> Self::Output {
                Self(self.0.wrapping_shr(other))
            }
        }

        impl Eint for $name {
            const BITS: u32 = <$uint>::MIN.leading_zeros();
            const MIN_U: Self = Self(0);
            const MAX_U: Self = Self(<$uint>::MAX);
            const MIN_S: Self = Self(<$sint>::MIN as $uint);
            const MAX_S: Self = Self(<$sint>::MAX as $uint);
            const ONE: Self = Self(1);
            const ZERO: Self = Self(0);

            fn clz(self) -> u32 {
                self.0.leading_zeros()
            }

            fn cmp_s(&self, other: &Self) -> std::cmp::Ordering {
                (self.0 as $sint).cmp(&(other.0 as $sint))
            }

            fn cmp_u(&self, other: &Self) -> std::cmp::Ordering {
                self.0.cmp(&other.0)
            }

            fn cpop(self) -> u32 {
                self.0.count_ones()
            }

            fn ctz(self) -> u32 {
                self.0.trailing_zeros()
            }

            fn get(mem: &[u8]) -> Self {
                let mut buf = [0u8; Self::BITS as usize >> 3];
                buf.copy_from_slice(&mem);
                Self(<$uint>::from_le_bytes(buf))
            }

            fn hi(self) -> Self {
                self >> (Self::BITS >> 1)
            }

            fn is_negative(self) -> bool {
                (self.0 as $sint).is_negative()
            }

            fn is_positive(self) -> bool {
                (self.0 as $sint).is_positive()
            }

            fn lo(self) -> Self {
                self & (Self::MAX_U >> (Self::BITS >> 1))
            }

            fn lo_sext(self) -> Self {
                self.sext((Self::BITS >> 1) - 1)
            }

            fn overflowing_add_s(self, other: Self) -> (Self, bool) {
                let (r, carry) = (self.0 as $sint).overflowing_add(other.0 as $sint);
                (Self(r as $uint), carry)
            }

            fn overflowing_add_u(self, other: Self) -> (Self, bool) {
                let (r, carry) = self.0.overflowing_add(other.0);
                (Self(r), carry)
            }

            fn overflowing_mul_s(self, other: Self) -> (Self, bool) {
                let (r, carry) = (self.0 as $sint).overflowing_mul(other.0 as $sint);
                (Self(r as $uint), carry)
            }

            fn overflowing_mul_u(self, other: Self) -> (Self, bool) {
                let (r, carry) = self.0.overflowing_mul(other.0);
                (Self(r), carry)
            }

            fn overflowing_sub_s(self, other: Self) -> (Self, bool) {
                let (r, borrow) = (self.0 as $sint).overflowing_sub(other.0 as $sint);
                (Self(r as $uint), borrow)
            }

            fn overflowing_sub_u(self, other: Self) -> (Self, bool) {
                let (r, borrow) = self.0.overflowing_sub(other.0);
                (Self(r), borrow)
            }

            fn put(&self, mem: &mut [u8]) {
                let buf = self.0.to_le_bytes();
                mem.copy_from_slice(&buf);
            }

            fn put_lo(&self, mem: &mut [u8]) {
                let buf = self.0.to_le_bytes();
                if Self::BITS == 8 {
                    mem[0] = buf[0] & 0x0f
                } else {
                    mem.copy_from_slice(&buf[0..buf.len() >> 1]);
                }
            }

            fn u8(self) -> u8 {
                self.0 as u8
            }

            fn u16(self) -> u16 {
                self.0 as u16
            }

            fn u32(self) -> u32 {
                self.0 as u32
            }

            fn u64(self) -> u64 {
                self.0 as u64
            }

            fn wrapping_add(self, other: Self) -> Self {
                Self(self.0.wrapping_add(other.0))
            }

            fn wrapping_div_s(self, other: Self) -> Self {
                if other.0 == 0 {
                    Self::MAX_U
                } else if self == Self::MIN_S && other == Self::MAX_U {
                    Self::MIN_S
                } else {
                    Self((self.0 as $sint).wrapping_div(other.0 as $sint) as $uint)
                }
            }

            fn wrapping_div_u(self, other: Self) -> Self {
                if other.0 == 0 {
                    Self::MAX_U
                } else {
                    Self(self.0.wrapping_div(other.0))
                }
            }

            fn wrapping_mul(self, other: Self) -> Self {
                Self(self.0.wrapping_mul(other.0))
            }

            fn wrapping_rem_s(self, other: Self) -> Self {
                if other.0 == 0 {
                    self
                } else if self.0 == 1 << (Self::BITS - 1) && other == Self::MAX_U {
                    Self::MIN_U
                } else {
                    Self((self.0 as $sint).wrapping_rem(other.0 as $sint) as $uint)
                }
            }

            fn wrapping_rem_u(self, other: Self) -> Self {
                if other.0 == 0 {
                    self
                } else {
                    Self(self.0.wrapping_rem(other.0))
                }
            }

            fn wrapping_shl(self, other: u32) -> Self {
                Self(self.0.wrapping_shl(other))
            }

            fn wrapping_shr(self, other: u32) -> Self {
                Self(self.0.wrapping_shr(other))
            }

            fn wrapping_sra(self, other: u32) -> Self {
                Self((self.0 as $sint).wrapping_shr(other) as $uint)
            }

            fn wrapping_sub(self, other: Self) -> Self {
                Self(self.0.wrapping_sub(other.0))
            }
        }
    };
}

macro_rules! uint_wrap_from_impl {
    ($name:ty, $from:ty) => {
        impl From<$from> for $name {
            fn from(small: $from) -> Self {
                Self::from(small.0)
            }
        }
    };
}

construct_eint_wrap!(E8, u8, i8);
construct_eint_wrap!(E16, u16, i16);
construct_eint_wrap!(E32, u32, i32);
construct_eint_wrap!(E64, u64, i64);
construct_eint_wrap!(E128, u128, i128);
uint_wrap_from_impl!(E16, E8);
uint_wrap_from_impl!(E32, E8);
uint_wrap_from_impl!(E32, E16);
uint_wrap_from_impl!(E64, E8);
uint_wrap_from_impl!(E64, E16);
uint_wrap_from_impl!(E64, E32);
uint_wrap_from_impl!(E128, E8);
uint_wrap_from_impl!(E128, E16);
uint_wrap_from_impl!(E128, E32);
uint_wrap_from_impl!(E128, E64);

#[macro_export]
macro_rules! construct_eint_twin_from_uint {
    ($name:ident, $half:ty, $from:ty) => {
        impl std::convert::From<$from> for $name {
            fn from(small: $from) -> Self {
                Self(<$half>::from(small), <$half>::MIN_U)
            }
        }
    };
}

#[macro_export]
macro_rules! construct_eint_twin_from_sint {
    ($name:ident, $half:ty, $from:ty) => {
        impl std::convert::From<$from> for $name {
            fn from(small: $from) -> Self {
                let lo = <$half>::from(small);
                let hi = if small >= 0 {
                    <$half>::MIN_U
                } else {
                    <$half>::MAX_U
                };
                Self(lo, hi)
            }
        }
    };
}

#[macro_export]
macro_rules! construct_eint_twin {
    ($name:ident, $half:ty) => {
        #[derive(Copy, Clone, Default, PartialEq, Eq)]
        pub struct $name(pub $half, pub $half);

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:x}{:x}", self.1, self.0)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:x}{:x}", self.1, self.0)
            }
        }

        impl std::fmt::LowerHex for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:x}{:x}", self.1, self.0)
            }
        }

        construct_eint_twin_from_uint!($name, $half, bool);
        construct_eint_twin_from_sint!($name, $half, i8);
        construct_eint_twin_from_sint!($name, $half, i16);
        construct_eint_twin_from_sint!($name, $half, i32);
        construct_eint_twin_from_sint!($name, $half, i64);
        construct_eint_twin_from_sint!($name, $half, i128);
        construct_eint_twin_from_uint!($name, $half, u8);
        construct_eint_twin_from_uint!($name, $half, u16);
        construct_eint_twin_from_uint!($name, $half, u32);
        construct_eint_twin_from_uint!($name, $half, u64);
        construct_eint_twin_from_uint!($name, $half, u128);

        impl std::ops::BitAnd for $name {
            type Output = Self;
            fn bitand(self, other: Self) -> Self::Output {
                Self(self.0 & other.0, self.1 & other.1)
            }
        }

        impl std::ops::BitAndAssign for $name {
            fn bitand_assign(&mut self, other: Self) {
                self.0 &= other.0;
                self.1 &= other.1;
            }
        }

        impl std::ops::BitOr for $name {
            type Output = Self;
            fn bitor(self, other: Self) -> Self::Output {
                Self(self.0 | other.0, self.1 | other.1)
            }
        }

        impl std::ops::BitOrAssign for $name {
            fn bitor_assign(&mut self, other: Self) {
                self.0 |= other.0;
                self.1 |= other.1;
            }
        }

        impl std::ops::BitXor for $name {
            type Output = Self;
            fn bitxor(self, other: Self) -> Self::Output {
                Self(self.0 ^ other.0, self.1 ^ other.1)
            }
        }

        impl std::ops::BitXorAssign for $name {
            fn bitxor_assign(&mut self, other: Self) {
                self.0 ^= other.0;
                self.1 ^= other.1;
            }
        }

        impl std::ops::Not for $name {
            type Output = Self;
            fn not(self) -> Self::Output {
                Self(!self.0, !self.1)
            }
        }

        impl std::ops::Neg for $name {
            type Output = Self;
            fn neg(self) -> Self::Output {
                (!self).wrapping_add(<$name>::ONE)
            }
        }

        impl std::cmp::PartialOrd for $name {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl std::cmp::Ord for $name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.cmp_u(other)
            }
        }

        impl std::ops::Add for $name {
            type Output = Self;
            fn add(self, other: Self) -> Self {
                self.wrapping_add(other)
            }
        }

        impl std::ops::AddAssign for $name {
            fn add_assign(&mut self, other: Self) {
                *self = self.wrapping_add(other)
            }
        }

        impl std::ops::Sub for $name {
            type Output = Self;
            fn sub(self, other: Self) -> Self::Output {
                self.wrapping_sub(other)
            }
        }

        impl std::ops::SubAssign for $name {
            fn sub_assign(&mut self, other: Self) {
                *self = self.wrapping_sub(other)
            }
        }

        impl std::ops::Mul for $name {
            type Output = Self;
            fn mul(self, other: Self) -> Self::Output {
                self.wrapping_mul(other)
            }
        }

        impl std::ops::MulAssign for $name {
            fn mul_assign(&mut self, other: Self) {
                *self = self.wrapping_mul(other)
            }
        }

        impl std::ops::Div for $name {
            type Output = Self;
            fn div(self, other: Self) -> Self::Output {
                self.wrapping_div_u(other)
            }
        }

        impl std::ops::DivAssign for $name {
            fn div_assign(&mut self, other: Self) {
                *self = self.wrapping_div_u(other)
            }
        }

        impl std::ops::Rem for $name {
            type Output = Self;
            fn rem(self, other: Self) -> Self::Output {
                self.wrapping_rem_u(other)
            }
        }

        impl std::ops::RemAssign for $name {
            fn rem_assign(&mut self, other: Self) {
                *self = self.wrapping_rem_u(other);
            }
        }

        impl std::ops::Shl<u32> for $name {
            type Output = Self;
            fn shl(self, other: u32) -> Self::Output {
                self.wrapping_shl(other)
            }
        }

        impl std::ops::Shr<u32> for $name {
            type Output = Self;
            fn shr(self, other: u32) -> Self::Output {
                self.wrapping_shr(other)
            }
        }

        impl Eint for $name {
            const BITS: u32 = <$half>::BITS * 2;
            const MAX_S: Self = Self(<$half>::MAX_U, <$half>::MAX_S);
            const MAX_U: Self = Self(<$half>::MAX_U, <$half>::MAX_U);
            const MIN_S: Self = Self(<$half>::MIN_U, <$half>::MIN_S);
            const MIN_U: Self = Self(<$half>::MIN_U, <$half>::MIN_U);
            const ONE: Self = Self(<$half>::ONE, <$half>::MIN_U);
            const ZERO: Self = Self(<$half>::MIN_U, <$half>::MIN_U);

            fn clz(self) -> u32 {
                if self.1 == <$half>::MIN_U {
                    Self::BITS / 2 + self.0.clz()
                } else {
                    self.1.clz()
                }
            }

            fn cmp_s(&self, other: &Self) -> std::cmp::Ordering {
                let l_sign = self.is_negative();
                let r_sign = other.is_negative();
                match (l_sign, r_sign) {
                    (false, false) => self.cmp(&other),
                    (false, true) => std::cmp::Ordering::Greater,
                    (true, false) => std::cmp::Ordering::Less,
                    (true, true) => self.cmp(&other),
                }
            }

            fn cmp_u(&self, other: &Self) -> std::cmp::Ordering {
                let hi_cmp = self.1.cmp(&other.1);
                if hi_cmp != std::cmp::Ordering::Equal {
                    hi_cmp
                } else {
                    self.0.cmp(&other.0)
                }
            }

            fn cpop(self) -> u32 {
                self.0.cpop() + self.1.cpop()
            }

            fn ctz(self) -> u32 {
                if self.0 == <$half>::MIN_U {
                    Self::BITS / 2 + self.1.ctz()
                } else {
                    self.0.ctz()
                }
            }

            fn get(mem: &[u8]) -> Self {
                Self(
                    <$half>::get(&mem[0..Self::BITS as usize >> 4]),
                    <$half>::get(&mem[Self::BITS as usize >> 4..Self::BITS as usize >> 3]),
                )
            }

            fn hi(self) -> Self {
                Self(self.1, <$half>::MIN_U)
            }

            fn is_negative(self) -> bool {
                self != <$name>::MIN_U && self.wrapping_shr(Self::BITS - 1) == <$name>::ONE
            }

            fn is_positive(self) -> bool {
                self != <$name>::MIN_U && self.wrapping_shr(Self::BITS - 1) == <$name>::MIN_U
            }

            fn lo(self) -> Self {
                Self(self.0, <$half>::MIN_U)
            }

            fn lo_sext(self) -> Self {
                let hi = if self.0.is_negative() {
                    <$half>::MAX_U
                } else {
                    <$half>::MIN_U
                };
                Self(self.0, hi)
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
                let (hi, hi_carry_1) = self.1.overflowing_add_u(<$half>::from(lo_carry));
                let (hi, hi_carry_2) = hi.overflowing_add_u(other.1);
                (Self(lo, hi), hi_carry_1 || hi_carry_2)
            }

            fn overflowing_mul_s(self, other: Self) -> (Self, bool) {
                let (lo, hi) = self.widening_mul_s(other);
                (lo, hi != Self::MIN_U)
            }

            fn overflowing_mul_u(self, other: Self) -> (Self, bool) {
                let (hi, hi_overflow_mul) = match (self.1, other.1) {
                    (_, <$half>::MIN_U) => self.1.overflowing_mul_u(other.0),
                    (<$half>::MIN_U, _) => other.1.overflowing_mul_u(self.0),
                    _ => (
                        self.1.wrapping_mul(other.0).wrapping_add(other.1.wrapping_mul(self.0)),
                        true,
                    ),
                };
                let lo = self.0.widening_mul_u(other.0);
                let (hi, hi_overflow_add) = lo.1.overflowing_add_u(hi);
                let lo = Self(lo.0, hi);
                (lo, hi_overflow_mul || hi_overflow_add)
            }

            fn overflowing_sub_s(self, other: Self) -> (Self, bool) {
                let r = self.wrapping_sub(other);
                if self.is_negative() == other.is_negative() {
                    (r, false)
                } else {
                    (r, r.is_negative() != self.is_negative())
                }
            }

            fn overflowing_sub_u(self, other: Self) -> (Self, bool) {
                let (lo, lo_borrow) = self.0.overflowing_sub_u(other.0);
                let (hi, hi_borrow_1) = self.1.overflowing_sub_u(<$half>::from(lo_borrow));
                let (hi, hi_borrow_2) = hi.overflowing_sub_u(other.1);
                (Self(lo, hi), hi_borrow_1 || hi_borrow_2)
            }

            fn put(&self, mem: &mut [u8]) {
                self.0.put(&mut mem[0..Self::BITS as usize >> 4]);
                self.1.put(&mut mem[Self::BITS as usize >> 4..Self::BITS as usize >> 3]);
            }

            fn put_lo(&self, mem: &mut [u8]) {
                self.0.put(mem);
            }

            fn u8(self) -> u8 {
                self.0.u8()
            }

            fn u16(self) -> u16 {
                self.0.u16()
            }

            fn u32(self) -> u32 {
                self.0.u32()
            }

            fn u64(self) -> u64 {
                self.0.u64()
            }

            fn wrapping_add(self, other: Self) -> Self {
                let (lo, carry) = self.0.overflowing_add_u(other.0);
                let hi = self.1.wrapping_add(other.1).wrapping_add(<$half>::from(carry));
                Self(lo, hi)
            }

            fn wrapping_div_s(self, other: Self) -> Self {
                if other == Self::MIN_U {
                    Self::MAX_U
                } else if self == Self::MIN_S && other == Self::MAX_U {
                    Self::MIN_S
                } else {
                    self.div_s(other).0
                }
            }

            fn wrapping_div_u(self, other: Self) -> Self {
                if other == Self::MIN_U {
                    Self::MAX_U
                } else {
                    self.div_u(other).0
                }
            }

            fn wrapping_mul(self, other: Self) -> Self {
                let (lo, hi) = self.0.widening_mul_u(other.0);
                let hi_0 = self.0.wrapping_mul(other.1);
                let hi_1 = self.1.wrapping_mul(other.0);
                let hi = hi.wrapping_add(hi_0).wrapping_add(hi_1);
                Self(lo, hi)
            }

            fn wrapping_rem_s(self, other: Self) -> Self {
                let minus_min = Self::ONE << (Self::BITS - 1);
                let minus_one = Self::MAX_U;
                if other == Self::MIN_U {
                    self
                } else if self == minus_min && other == minus_one {
                    Self::MIN_U
                } else {
                    self.div_s(other).1
                }
            }

            fn wrapping_rem_u(self, other: Self) -> Self {
                if other == Self::MIN_U {
                    self
                } else {
                    self.div_u(other).1
                }
            }

            fn wrapping_shl(self, other: u32) -> Self {
                let shamt = other % Self::BITS;
                if shamt < Self::BITS / 2 {
                    let lo = self.0.wrapping_shl(shamt);
                    let hi =
                        self.1.wrapping_shl(shamt) | self.0.wrapping_shr(1).wrapping_shr((Self::BITS / 2) - 1 - shamt);
                    Self(lo, hi)
                } else {
                    let lo = <$half>::MIN_U;
                    let hi = self.0.wrapping_shl(shamt - Self::BITS / 2);
                    Self(lo, hi)
                }
            }

            fn wrapping_shr(self, other: u32) -> Self {
                let shamt = other % Self::BITS;
                if shamt < Self::BITS / 2 {
                    let lo =
                        self.0.wrapping_shr(shamt) | self.1.wrapping_shl(1).wrapping_shl((Self::BITS / 2) - 1 - shamt);
                    let hi = self.1.wrapping_shr(shamt);
                    Self(lo, hi)
                } else {
                    let lo = self.1.wrapping_shr(shamt - Self::BITS / 2);
                    let hi = <$half>::MIN_U;
                    Self(lo, hi)
                }
            }

            fn wrapping_sra(self, other: u32) -> Self {
                let shamt = other % Self::BITS;
                let hi = if self.is_negative() && shamt != 0 {
                    Self::MAX_U << (Self::BITS - shamt)
                } else {
                    Self::MIN_U
                };
                let lo = self.wrapping_shr(shamt);
                hi | lo
            }

            fn wrapping_sub(self, other: Self) -> Self {
                let (lo, borrow) = self.0.overflowing_sub_u(other.0);
                let hi = self.1.wrapping_sub(other.1).wrapping_sub(<$half>::from(borrow));
                Self(lo, hi)
            }
        }

        impl $name {
            /// div_half_0 returns the quotient and remainder of (hi, lo) divided by y: quo = (hi, lo)/y,
            /// rem = (hi, lo)%y with the dividend bits' upper half in parameter hi and the lower half in parameter lo.
            /// div_half_0 panics for y == 0 (division by zero) or y <= hi (quotient overflow).
            ///
            /// See https://cs.opensource.google/go/go/+/refs/tags/go1.17.3:src/math/bits/bits.go;l=512
            fn div_u_half_0(self, y: $half) -> ($half, $half) {
                let twos = <$half>::ONE << (Self::BITS / 4);
                let mask = twos - <$half>::ONE;
                debug_assert!(y != <$half>::ZERO);
                debug_assert!(y > self.1);
                let s = y.clz();
                let y = y << s;
                let yn1 = y >> (Self::BITS / 4);
                let yn0 = y & mask;
                let un32 = (self.1 << s)
                    | if s == 0 {
                        <$half>::ZERO
                    } else {
                        self.0 >> (Self::BITS / 2 - s)
                    };
                let un10 = self.0 << s;
                let un1 = un10 >> (Self::BITS / 4);
                let un0 = un10 & mask;
                let mut q1 = un32 / yn1;
                let mut rhat = un32 - q1 * yn1;
                while q1 >= twos || q1 * yn0 > twos * rhat + un1 {
                    q1 -= <$half>::ONE;
                    rhat += yn1;
                    if rhat >= twos {
                        break;
                    }
                }
                let un21 = un32 * twos + un1 - q1 * y;
                let mut q0 = un21 / yn1;
                rhat = un21 - q0 * yn1;
                while q0 >= twos || q0 * yn0 > twos * rhat + un0 {
                    q0 -= <$half>::ONE;
                    rhat += yn1;
                    if rhat >= twos {
                        break;
                    }
                }
                (q1 * twos + q0, (un21 * twos + un0 - q0 * y) >> s)
            }

            /// See https://github.com/Pilatuz/bigx/blob/8615506d17c5/uint128.go#L319
            fn div_u_half_1(self, y: $half) -> (Self, $half) {
                if self.1 < y {
                    let (lo, r) = self.div_u_half_0(y);
                    (Self::from(lo), r)
                } else {
                    let (hi, r) = Self::from(self.1).div_u_half_0(y);
                    let (lo, r) = Self(self.0, r).div_u_half_0(y);
                    (Self(lo, hi), r)
                }
            }

            /// See https://github.com/Pilatuz/bigx/blob/8615506d17c5/uint128.go#L291
            fn div_u(self, other: Self) -> (Self, Self) {
                if other.1 == <$half>::ZERO {
                    let (q, r) = self.div_u_half_1(other.0);
                    return (q, Self::from(r));
                }
                let n = other.1.clz();
                let u1 = self >> 1;
                let v1 = other << n;
                let (tq, _) = u1.div_u_half_0(v1.1);
                let mut tq = tq >> (Self::BITS / 2 - 1 - n);
                if tq != <$half>::ZERO {
                    tq -= <$half>::ONE;
                }
                let mut q = Self::from(tq);
                let mut r = self - other * q;
                if r >= other {
                    q += Self::ONE;
                    r -= other;
                }
                (q, r)
            }

            /// See https://github.com/chfast/intx/blob/2f62de735fe688e9645af8904099f0571f8f0d9c/include/intx/intx.hpp#L789
            fn div_s(self, other: Self) -> (Self, Self) {
                let x = self;
                let y = other;
                let x_is_neg = x.is_negative();
                let y_is_neg = y.is_negative();
                let x_abs = if x_is_neg { -x } else { x };
                let y_abs = if y_is_neg { -y } else { y };
                let q_is_neg = x_is_neg ^ y_is_neg;
                let r = x_abs.div_u(y_abs);
                let quo = r.0;
                let rem = r.1;
                let quo = Self::from(if q_is_neg { -quo } else { quo });
                let rem = Self::from(if x_is_neg { -rem } else { rem });
                (quo, rem)
            }
        }
    };
}

macro_rules! uint_twin_from_impl {
    ($name:ident, $half:ty) => {
        impl std::convert::From<$half> for $name {
            fn from(small: $half) -> Self {
                Self(small, <$half>::MIN_U)
            }
        }
    };
    ($name:ident, $half:ty, $from:ty) => {
        impl std::convert::From<$from> for $name {
            fn from(small: $from) -> Self {
                Self(<$half>::from(small), <$half>::MIN_U)
            }
        }
    };
}

construct_eint_twin!(E256, E128);
construct_eint_twin!(E512, E256);
construct_eint_twin!(E1024, E512);
construct_eint_twin!(E2048, E1024);
uint_twin_from_impl!(E256, E128);
uint_twin_from_impl!(E512, E256, E128);
uint_twin_from_impl!(E512, E256);
uint_twin_from_impl!(E1024, E512, E128);
uint_twin_from_impl!(E1024, E512, E256);
uint_twin_from_impl!(E1024, E512);
uint_twin_from_impl!(E2048, E1024, E128);
uint_twin_from_impl!(E2048, E1024, E256);
uint_twin_from_impl!(E2048, E1024, E512);
uint_twin_from_impl!(E2048, E1024);
