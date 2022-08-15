//! # Eint
//!
//! Extended precision integer Rust library. Provides signed/unsigned integer 256 to 2048.
//!
//! ```text
//! [dependencies]
//! eint = "0.1"
//! ```

#![no_std]
extern crate alloc;
use alloc::{format, string::String};

pub trait WideningMulU {
    fn _widening_mul_u(self, other: Self) -> (Self, Self)
    where
        Self: Sized;
}

#[macro_export]
macro_rules! impl_widening_mul_u_wrap {
    ($eint:ty, $wint:ty) => {
        impl WideningMulU for $eint {
            fn _widening_mul_u(self, other: Self) -> (Self, Self) {
                let lh = (self.0 as $wint) * (other.0 as $wint);
                let l = Self::from(lh);
                let h = Self::from(lh >> Self::BITS);
                (l, h)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_widening_mul_u_u128 {
    ($eint:ty) => {
        impl WideningMulU for $eint {
            fn _widening_mul_u(self, other: Self) -> (Self, Self) {
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
        }
    };
}

#[macro_export]
macro_rules! impl_widening_mul_u_twin {
    ($eint:ty, $size:expr) => {
        impl WideningMulU for $eint {
            fn _widening_mul_u(self, other: Self) -> (Self, Self) {
                let mut lh = [0u64; $size * 2];
                for i in 0..$size {
                    let mut c = 0u64;
                    for j in 0..$size {
                        let uv = self.0[j] as u128 * other.0[i] as u128 + lh[i + j] as u128 + c as u128;
                        lh[i + j] = uv as u64;
                        c = (uv >> 64) as u64;
                    }
                    lh[i + $size] = c;
                }

                let mut lo = [0u64; $size];
                lo.copy_from_slice(&lh[0..$size]);
                let mut hi = [0u64; $size];
                hi.copy_from_slice(&lh[$size..$size * 2]);
                (Self(lo), Self(hi))
            }
        }
    };
}

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
    + From<Self>
    + PartialEq
    + core::cmp::Ord
    + core::cmp::PartialOrd
    + core::fmt::Debug
    + core::fmt::Display
    + core::fmt::LowerHex
    + core::ops::Add<Output = Self>
    + core::ops::AddAssign
    + core::ops::BitAnd<Output = Self>
    + core::ops::BitAndAssign
    + core::ops::BitOr<Output = Self>
    + core::ops::BitOrAssign
    + core::ops::BitXor<Output = Self>
    + core::ops::BitXorAssign
    + core::ops::Div<Output = Self>
    + core::ops::DivAssign
    + core::ops::Mul<Output = Self>
    + core::ops::MulAssign
    + core::ops::Neg<Output = Self>
    + core::ops::Not
    + core::ops::Rem<Output = Self>
    + core::ops::RemAssign
    + core::ops::Sub<Output = Self>
    + core::ops::SubAssign
    + core::ops::Shl<u32, Output = Self>
    + core::ops::Shr<u32, Output = Self>
    + WideningMulU
{
    const BITS: u32;
    const MAX_S: Self;
    const MAX_U: Self;
    const MIN_S: Self;
    const MIN_U: Self;
    const ONE: Self;
    const ZERO: Self;

    /// Returns (self + rhs) >> 1. Signed.
    fn average_add_s(self, other: Self) -> Self {
        (self & other).wrapping_add((self ^ other).wrapping_sra(1))
    }

    /// Returns (self + rhs) >> 1.
    fn average_add_u(self, other: Self) -> Self {
        (self & other).wrapping_add((self ^ other).wrapping_shr(1))
    }

    /// Returns (self - rhs) >> 1. Signed.
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

    /// Get bit.
    fn bit(&self, n: u32) -> bool;

    /// Clear bit.
    fn bit_clr(&mut self, n: u32);

    /// Set bit.
    fn bit_set(&mut self, n: u32);

    /// Returns the number of leading zeros in the binary representation of self.
    fn clz(&self) -> u32;

    /// Compare. Signed.
    fn cmp_s(&self, other: &Self) -> core::cmp::Ordering;

    /// Compare.
    fn cmp_u(&self, other: &Self) -> core::cmp::Ordering;

    /// Returns the number of ones in the binary representation of self.
    fn cpop(&self) -> u32;

    /// Returns the number of trailing zeros in the binary representation of self.
    fn ctz(&self) -> u32;

    /// Get a native endian integer value from its representation as a byte slice in little endian.
    fn get(mem: &[u8]) -> Self {
        unsafe { core::ptr::read(mem.as_ptr() as *const _) }
    }

    /// Returns the higher part.
    fn hi(self) -> Self;

    /// Returns true if highest bit is set.
    fn is_negative(&self) -> bool;

    /// Returns true if highest bit is not set.
    fn is_positive(&self) -> bool;

    /// Returns the lower part.
    fn lo(self) -> Self;

    /// Returns the lower part and sign extend it.
    fn lo_sext(self) -> Self;

    /// Calculates self + rhs. Signed.
    fn overflowing_add_s(self, other: Self) -> (Self, bool);

    /// Calculates self + rhs.
    fn overflowing_add_u(self, other: Self) -> (Self, bool);

    /// Calculates self * rhs. Signed.
    fn overflowing_mul_s(self, other: Self) -> (Self, bool);

    /// Calculates self * rhs.
    fn overflowing_mul_u(self, other: Self) -> (Self, bool);

    /// Calculates self - rhs. Signed.
    fn overflowing_sub_s(self, other: Self) -> (Self, bool);

    /// Calculates self - rhs.
    fn overflowing_sub_u(self, other: Self) -> (Self, bool);

    /// Save the integer as a byte array in little-endian byte order to memory.
    fn put(&self, mem: &mut [u8]);

    /// Put the lower part integer as a byte array in little-endian byte order to memory.
    fn put_lo(&self, mem: &mut [u8]);

    /// Saturating integer addition. Computes self + rhs, saturating at the numeric bounds instead of overflowing. Signed.
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

    /// Saturating integer subtraction. Computes self - rhs, saturating at the numeric bounds instead of overflowing. Signed.
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

    /// Widening add. Signed.
    /// (lo, hi) = x + y with the product bits' upper half returned in hi and the lower half returned in lo.
    fn widening_add_s(self, other: Self) -> (Self, Self) {
        let hi_0 = if self.is_negative() { Self::MAX_U } else { Self::MIN_U };
        let hi_1 = if other.is_negative() { Self::MAX_U } else { Self::MIN_U };
        let (lo, carry) = self.overflowing_add_u(other);
        let hi = hi_0.wrapping_add(hi_1).wrapping_add(Self::from(carry));
        (lo, hi)
    }

    /// Widening add.
    /// (lo, hi) = x + y with the product bits' upper half returned in hi and the lower half returned in lo.
    fn widening_add_u(self, other: Self) -> (Self, Self) {
        let (lo, carry) = self.overflowing_add_u(other);
        (lo, Self::from(carry))
    }

    /// Widening multiple. Signed.
    ///
    /// (lo, hi) = x * y with the product bits' upper half returned in hi and the lower half returned in lo.
    /// Inspired by https://sqlite.in/?qa=668884/c-32-bit-signed-integer-multiplication-without-using-64-bit-data-type
    fn widening_mul_s(self, other: Self) -> (Self, Self) {
        let (lo, hi) = self.widening_mul_u(other);
        let hi = hi
            - if self.is_negative() { other } else { Self::MIN_U }
            - if other.is_negative() { self } else { Self::MIN_U };
        (lo, hi)
    }

    /// Widening signed and unsigned integer multiply.
    /// (lo, hi) = x * y with the product bits' upper half returned in hi and the lower half returned in lo.
    fn widening_mul_su(self, other: Self) -> (Self, Self) {
        if !other.is_negative() {
            self.widening_mul_s(other)
        } else {
            let (lo, hi) = self.widening_mul_s(other);
            let hi = hi + self;
            (lo, hi)
        }
    }

    /// Widening multiple.
    /// (lo, hi) = x * y with the product bits' upper half returned in hi and the lower half returned in lo.
    fn widening_mul_u(self, other: Self) -> (Self, Self) {
        self._widening_mul_u(other)
    }

    /// Widening substract. Signed.
    /// (lo, hi) = x - y with the product bits' upper half returned in hi and the lower half returned in lo.
    fn widening_sub_s(self, other: Self) -> (Self, Self) {
        let hi_0 = if self.is_negative() { Self::MAX_U } else { Self::MIN_U };
        let hi_1 = if other.is_negative() { Self::MAX_U } else { Self::MIN_U };
        let (lo, borrow) = self.overflowing_sub_u(other);
        let hi = hi_0.wrapping_sub(hi_1).wrapping_sub(Self::from(borrow));
        (lo, hi)
    }

    /// Widening substract.
    /// (lo, hi) = x - y with the product bits' upper half returned in hi and the lower half returned in lo.
    fn widening_sub_u(self, other: Self) -> (Self, Self) {
        let (lo, borrow) = self.overflowing_sub_u(other);
        (lo, if borrow { Self::MAX_U } else { Self::MIN_U })
    }

    /// Wrapping (modular) addition. Computes self + rhs, wrapping around at the boundary of the type.
    fn wrapping_add(self, other: Self) -> Self;

    /// Wrapping (modular) division. Computes self / rhs.
    /// 1) x / 0 = MAX_U
    fn wrapping_div_u(self, other: Self) -> Self;

    /// Wrapping (modular) division. Signed.
    /// 1) x / 0 = -1.
    /// 2) MIN_S / -1 = MIN_S
    fn wrapping_div_s(self, other: Self) -> Self;

    /// Wrapping (modular) multiplication. Computes self * rhs, wrapping around at the boundary of the type.
    fn wrapping_mul(self, other: Self) -> Self;

    /// Wrapping (modular) remainder. Signed.
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
        impl core::convert::From<$from> for $name {
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

        impl core::cmp::Ord for $name {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.0.cmp(&other.0)
            }
        }

        impl core::cmp::PartialOrd for $name {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                return self.0.partial_cmp(&other.0);
            }
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let suffix = format!("{:x}", self.0);
                let prefix = String::from("0").repeat(Self::BITS as usize / 4 - suffix.len());
                write!(f, "{}", prefix)?;
                write!(f, "{}", suffix)
            }
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let suffix = format!("{:x}", self.0);
                let prefix = String::from("0").repeat(Self::BITS as usize / 4 - suffix.len());
                write!(f, "{}", prefix)?;
                write!(f, "{}", suffix)
            }
        }

        impl core::fmt::LowerHex for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let suffix = format!("{:x}", self.0);
                let prefix = String::from("0").repeat(Self::BITS as usize / 4 - suffix.len());
                write!(f, "{}", prefix)?;
                write!(f, "{}", suffix)
            }
        }

        impl core::ops::Add for $name {
            type Output = Self;
            fn add(self, other: Self) -> Self {
                Self(self.0.wrapping_add(other.0))
            }
        }

        impl core::ops::AddAssign for $name {
            fn add_assign(&mut self, other: Self) {
                self.0 = self.0.wrapping_add(other.0)
            }
        }

        impl core::ops::Div for $name {
            type Output = Self;
            fn div(self, other: Self) -> Self::Output {
                if other.0 == 0 {
                    Self::MAX_U
                } else {
                    Self(self.0.wrapping_div(other.0))
                }
            }
        }

        impl core::ops::DivAssign for $name {
            fn div_assign(&mut self, other: Self) {
                self.0 = if other.0 == 0 { <$uint>::MAX } else { self.0.wrapping_div(other.0) }
            }
        }

        impl core::ops::BitAnd for $name {
            type Output = Self;
            fn bitand(self, other: Self) -> Self::Output {
                Self(self.0 & other.0)
            }
        }

        impl core::ops::BitAndAssign for $name {
            fn bitand_assign(&mut self, other: Self) {
                self.0 &= other.0
            }
        }

        impl core::ops::BitOr for $name {
            type Output = Self;
            fn bitor(self, other: Self) -> Self::Output {
                Self(self.0 | other.0)
            }
        }

        impl core::ops::BitOrAssign for $name {
            fn bitor_assign(&mut self, other: Self) {
                self.0 |= other.0
            }
        }

        impl core::ops::BitXor for $name {
            type Output = Self;
            fn bitxor(self, other: Self) -> Self::Output {
                Self(self.0 ^ other.0)
            }
        }

        impl core::ops::BitXorAssign for $name {
            fn bitxor_assign(&mut self, other: Self) {
                self.0 ^= other.0
            }
        }

        impl core::ops::Mul for $name {
            type Output = Self;
            fn mul(self, other: Self) -> Self::Output {
                Self(self.0.wrapping_mul(other.0))
            }
        }

        impl core::ops::MulAssign for $name {
            fn mul_assign(&mut self, other: Self) {
                self.0 = self.0.wrapping_mul(other.0)
            }
        }

        impl core::ops::Neg for $name {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self((!self.0).wrapping_add(1))
            }
        }

        impl core::ops::Not for $name {
            type Output = Self;
            fn not(self) -> Self::Output {
                Self(!self.0)
            }
        }

        impl core::ops::Rem for $name {
            type Output = Self;
            fn rem(self, other: Self) -> Self::Output {
                if other.0 == 0 {
                    self
                } else {
                    Self(self.0.wrapping_rem(other.0))
                }
            }
        }

        impl core::ops::RemAssign for $name {
            fn rem_assign(&mut self, other: Self) {
                self.0 = if other.0 == 0 { self.0 } else { self.0.wrapping_rem(other.0) }
            }
        }

        impl core::ops::Shl<u32> for $name {
            type Output = Self;
            fn shl(self, other: u32) -> Self::Output {
                Self(self.0.wrapping_shl(other))
            }
        }

        impl core::ops::Shr<u32> for $name {
            type Output = Self;
            fn shr(self, other: u32) -> Self::Output {
                Self(self.0.wrapping_shr(other))
            }
        }

        impl core::ops::Sub for $name {
            type Output = Self;
            fn sub(self, other: Self) -> Self::Output {
                Self(self.0.wrapping_sub(other.0))
            }
        }

        impl core::ops::SubAssign for $name {
            fn sub_assign(&mut self, other: Self) {
                self.0 = self.0.wrapping_sub(other.0)
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

            fn bit(&self, n: u32) -> bool {
                self.0.wrapping_shr(n) & 1 != 0
            }

            fn bit_clr(&mut self, n: u32) {
                self.0 &= !<$name>::ONE.0.wrapping_shl(n)
            }

            fn bit_set(&mut self, n: u32) {
                self.0 |= <$name>::ONE.0.wrapping_shl(n)
            }

            fn clz(&self) -> u32 {
                self.0.leading_zeros()
            }

            fn cmp_s(&self, other: &Self) -> core::cmp::Ordering {
                (self.0 as $sint).cmp(&(other.0 as $sint))
            }

            fn cmp_u(&self, other: &Self) -> core::cmp::Ordering {
                self.0.cmp(&other.0)
            }

            fn cpop(&self) -> u32 {
                self.0.count_ones()
            }

            fn ctz(&self) -> u32 {
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

            fn is_negative(&self) -> bool {
                (self.0 as $sint).is_negative()
            }

            fn is_positive(&self) -> bool {
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
impl_widening_mul_u_wrap!(E8, u16);
impl_widening_mul_u_wrap!(E16, u32);
impl_widening_mul_u_wrap!(E32, u64);
impl_widening_mul_u_wrap!(E64, u128);
impl_widening_mul_u_u128!(E128);
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
    ($name:ident, $from:ty) => {
        impl core::convert::From<$from> for $name {
            fn from(small: $from) -> Self {
                let mut b = [u64::MIN; Self::BITS as usize >> 6];
                unsafe {
                    core::ptr::copy_nonoverlapping(
                        &small as *const $from as *const u8,
                        b.as_mut_ptr() as *mut u8,
                        Self::BITS as usize >> 3,
                    );
                }
                Self(b)
            }
        }
    };
}

#[macro_export]
macro_rules! construct_eint_twin_from_sint {
    ($name:ident, $from:ty) => {
        impl core::convert::From<$from> for $name {
            fn from(small: $from) -> Self {
                let mut b = if small >= 0 {
                    [u64::MIN; Self::BITS as usize >> 6]
                } else {
                    [u64::MAX; Self::BITS as usize >> 6]
                };
                unsafe {
                    core::ptr::copy_nonoverlapping(
                        &small as *const $from as *const u8,
                        b.as_mut_ptr() as *mut u8,
                        Self::BITS as usize >> 3,
                    );
                }
                Self(b)
            }
        }
    };
}

#[macro_export]
macro_rules! construct_eint_twin {
    ($name:ident, $size:expr) => {
        #[derive(Copy, Clone, Default, PartialEq, Eq)]
        pub struct $name(pub [u64; $size]);

        impl core::convert::From<bool> for $name {
            fn from(small: bool) -> Self {
                if small {
                    Self::ONE
                } else {
                    Self::MIN_U
                }
            }
        }

        construct_eint_twin_from_sint!($name, i8);
        construct_eint_twin_from_sint!($name, i16);
        construct_eint_twin_from_sint!($name, i32);
        construct_eint_twin_from_sint!($name, i64);
        construct_eint_twin_from_sint!($name, i128);
        construct_eint_twin_from_uint!($name, u8);
        construct_eint_twin_from_uint!($name, u16);
        construct_eint_twin_from_uint!($name, u32);
        construct_eint_twin_from_uint!($name, u64);
        construct_eint_twin_from_uint!($name, u128);

        impl core::cmp::PartialOrd for $name {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl core::cmp::Ord for $name {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.cmp_u(other)
            }
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                for i in self.0.iter().rev() {
                    write!(f, "{:016x}", i)?
                }
                Ok(())
            }
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                for i in self.0.iter().rev() {
                    write!(f, "{:016x}", i)?
                }
                Ok(())
            }
        }

        impl core::fmt::LowerHex for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                for i in self.0.iter().rev() {
                    write!(f, "{:016x}", i)?
                }
                Ok(())
            }
        }

        impl core::ops::Add for $name {
            type Output = Self;
            fn add(self, other: Self) -> Self {
                self.wrapping_add(other)
            }
        }

        impl core::ops::AddAssign for $name {
            fn add_assign(&mut self, other: Self) {
                *self = self.wrapping_add(other)
            }
        }

        impl core::ops::BitAnd for $name {
            type Output = Self;
            fn bitand(self, other: Self) -> Self::Output {
                let mut b = [0u64; $size];
                for i in 0..$size {
                    b[i] = self.0[i] & other.0[i];
                }
                Self(b)
            }
        }

        impl core::ops::BitAndAssign for $name {
            fn bitand_assign(&mut self, other: Self) {
                for i in 0..$size {
                    self.0[i] &= other.0[i];
                }
            }
        }

        impl core::ops::BitOr for $name {
            type Output = Self;
            fn bitor(self, other: Self) -> Self::Output {
                let mut b = [0u64; $size];
                for i in 0..$size {
                    b[i] = self.0[i] | other.0[i];
                }
                Self(b)
            }
        }

        impl core::ops::BitOrAssign for $name {
            fn bitor_assign(&mut self, other: Self) {
                for i in 0..$size {
                    self.0[i] |= other.0[i];
                }
            }
        }

        impl core::ops::BitXor for $name {
            type Output = Self;
            fn bitxor(self, other: Self) -> Self::Output {
                let mut b = [0u64; $size];
                for i in 0..$size {
                    b[i] = self.0[i] ^ other.0[i];
                }
                Self(b)
            }
        }

        impl core::ops::BitXorAssign for $name {
            fn bitxor_assign(&mut self, other: Self) {
                for i in 0..$size {
                    self.0[i] ^= other.0[i];
                }
            }
        }

        impl core::ops::Div for $name {
            type Output = Self;
            fn div(self, other: Self) -> Self::Output {
                self.wrapping_div_u(other)
            }
        }

        impl core::ops::DivAssign for $name {
            fn div_assign(&mut self, other: Self) {
                *self = self.wrapping_div_u(other)
            }
        }

        impl core::ops::Mul for $name {
            type Output = Self;
            fn mul(self, other: Self) -> Self::Output {
                self.wrapping_mul(other)
            }
        }

        impl core::ops::MulAssign for $name {
            fn mul_assign(&mut self, other: Self) {
                *self = self.wrapping_mul(other)
            }
        }

        impl core::ops::Neg for $name {
            type Output = Self;
            fn neg(self) -> Self::Output {
                (!self).wrapping_add(<$name>::ONE)
            }
        }

        impl core::ops::Not for $name {
            type Output = Self;
            fn not(self) -> Self::Output {
                let mut b = [0u64; $size];
                for i in 0..$size {
                    b[i] = !self.0[i];
                }
                Self(b)
            }
        }

        impl core::ops::Rem for $name {
            type Output = Self;
            fn rem(self, other: Self) -> Self::Output {
                self.wrapping_rem_u(other)
            }
        }

        impl core::ops::RemAssign for $name {
            fn rem_assign(&mut self, other: Self) {
                *self = self.wrapping_rem_u(other);
            }
        }

        impl core::ops::Shl<u32> for $name {
            type Output = Self;
            fn shl(self, other: u32) -> Self::Output {
                self.wrapping_shl(other)
            }
        }

        impl core::ops::Shr<u32> for $name {
            type Output = Self;
            fn shr(self, other: u32) -> Self::Output {
                self.wrapping_shr(other)
            }
        }

        impl core::ops::Sub for $name {
            type Output = Self;
            fn sub(self, other: Self) -> Self::Output {
                self.wrapping_sub(other)
            }
        }

        impl core::ops::SubAssign for $name {
            fn sub_assign(&mut self, other: Self) {
                *self = self.wrapping_sub(other)
            }
        }

        impl Eint for $name {
            const BITS: u32 = $size * 64;
            const MAX_S: Self = {
                let mut b = [u64::MAX; $size as usize];
                b[$size as usize - 1] = i64::MAX as u64;
                Self(b)
            };
            const MAX_U: Self = Self([u64::MAX; $size]);
            const MIN_S: Self = {
                let mut b = [u64::MIN; $size as usize];
                b[$size as usize - 1] = i64::MIN as u64;
                Self(b)
            };
            const MIN_U: Self = Self([u64::MIN; $size]);
            const ONE: Self = {
                let mut b = [u64::MIN; $size as usize];
                b[0] = 1;
                Self(b)
            };
            const ZERO: Self = Self([u64::MIN; $size]);

            fn bit(&self, n: u32) -> bool {
                let n = n % Self::BITS;
                self.0[n as usize / 64] & (1 << (n % 64)) != 0
            }

            fn bit_clr(&mut self, n: u32) {
                let n = n % Self::BITS;
                self.0[n as usize / 64] &= !(1 << (n % 64))
            }

            fn bit_set(&mut self, n: u32) {
                let n = n % Self::BITS;
                self.0[n as usize / 64] |= 1 << (n % 64)
            }

            fn clz(&self) -> u32 {
                let mut r = 0;
                for i in 0..$size {
                    let w = self.0[$size - i - 1];
                    if w == 0 {
                        r += 64;
                    } else {
                        r += w.leading_zeros();
                        break;
                    }
                }
                r
            }

            fn cmp_s(&self, other: &Self) -> core::cmp::Ordering {
                let l_sign = self.is_negative();
                let r_sign = other.is_negative();
                match (l_sign, r_sign) {
                    (false, false) => self.cmp(&other),
                    (false, true) => core::cmp::Ordering::Greater,
                    (true, false) => core::cmp::Ordering::Less,
                    (true, true) => self.cmp(&other),
                }
            }

            fn cmp_u(&self, other: &Self) -> core::cmp::Ordering {
                self.0.iter().rev().cmp(other.0.iter().rev())
            }

            fn cpop(&self) -> u32 {
                let mut r = 0;
                for i in 0..$size {
                    r += self.0[i].count_ones();
                }
                r
            }

            fn ctz(&self) -> u32 {
                let mut r = 0;
                for i in 0..$size {
                    let w = self.0[i];
                    if w == 0 {
                        r += 64;
                    } else {
                        r += w.trailing_zeros();
                        break;
                    }
                }
                r
            }

            fn get(mem: &[u8]) -> Self {
                let mut b = [0u64; $size];
                unsafe {
                    core::ptr::copy_nonoverlapping(
                        mem.as_ptr() as *const u8,
                        b.as_mut_ptr() as *mut u8,
                        Self::BITS as usize >> 3,
                    );
                }
                Self(b)
            }

            fn hi(self) -> Self {
                let mut b = [0u64; $size];
                b[0..$size / 2].copy_from_slice(&self.0[$size / 2..$size]);
                Self(b)
            }

            fn is_negative(&self) -> bool {
                (self.0[$size - 1] as i64).is_negative()
            }

            fn is_positive(&self) -> bool {
                (self.0[$size - 1] as i64).is_positive()
            }

            fn lo(self) -> Self {
                let mut b = [0u64; $size];
                b[0..$size / 2].copy_from_slice(&self.0[0..$size / 2]);
                Self(b)
            }

            fn lo_sext(self) -> Self {
                let mut b = self.0;
                if (b[$size / 2 - 1] as i64).is_negative() {
                    b = Self::MAX_U.0;
                    b[0..$size / 2].copy_from_slice(&self.0[0..$size / 2]);
                    Self(b)
                } else {
                    self
                }
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
                let mut b = [0u64; $size];
                let mut carry = false;
                for i in 0..$size {
                    let (r0, carry0) = self.0[i].overflowing_add(other.0[i]);
                    let (r1, carry1) = r0.overflowing_add(carry as u64);
                    b[i] = r1;
                    carry = carry0 | carry1
                }
                (Self(b), carry)
            }

            fn overflowing_mul_s(self, other: Self) -> (Self, bool) {
                let (lo, hi) = self.widening_mul_s(other);
                if !hi.is_negative() {
                    if hi != Self::MIN_U || lo.is_negative() {
                        return (lo, true);
                    } else {
                        return (lo, false);
                    }
                } else {
                    if hi != Self::MAX_U || lo < Self::MIN_S {
                        return (lo, true);
                    } else {
                        return (lo, false);
                    }
                }
            }

            fn overflowing_mul_u(self, other: Self) -> (Self, bool) {
                let (lo, hi) = self.widening_mul_u(other);
                (lo, hi != Self::ZERO)
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
                let mut b = [0u64; $size];
                let mut borrow = false;
                for i in 0..$size {
                    let (r0, borrow0) = self.0[i].overflowing_sub(other.0[i]);
                    let (r1, borrow1) = r0.overflowing_sub(borrow as u64);
                    b[i] = r1;
                    borrow = borrow0 | borrow1
                }
                (Self(b), borrow)
            }

            fn put(&self, mem: &mut [u8]) {
                unsafe {
                    core::ptr::copy_nonoverlapping(
                        self.0.as_ptr() as *const u8,
                        mem.as_mut_ptr(),
                        Self::BITS as usize >> 3,
                    );
                }
            }

            fn put_lo(&self, mem: &mut [u8]) {
                unsafe {
                    core::ptr::copy_nonoverlapping(
                        self.0.as_ptr() as *const u8,
                        mem.as_mut_ptr(),
                        Self::BITS as usize >> 4,
                    );
                }
            }

            fn u8(self) -> u8 {
                self.0[0] as u8
            }

            fn u16(self) -> u16 {
                self.0[0] as u16
            }

            fn u32(self) -> u32 {
                self.0[0] as u32
            }

            fn u64(self) -> u64 {
                self.0[0]
            }

            fn wrapping_add(self, other: Self) -> Self {
                let mut b = [0u64; $size];
                let mut carry = false;
                for i in 0..$size {
                    let (r0, carry0) = self.0[i].overflowing_add(other.0[i]);
                    let (r1, carry1) = r0.overflowing_add(carry as u64);
                    b[i] = r1;
                    carry = carry0 | carry1
                }
                Self(b)
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
                let mut b = [0u64; $size];
                for i in 0..$size {
                    let mut c = 0u64;
                    let inner_count = $size - i;
                    for j in 0..inner_count {
                        let uv: u128 = (self.0[j] as u128) * other.0[i] as u128 + b[i + j] as u128 + c as u128;
                        b[i + j] = uv as u64;
                        c = (uv >> 64) as u64;
                    }
                    if ((i + inner_count) < $size) {
                        b[i + inner_count] = c;
                    }
                }
                Self(b)
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
                let mut b = [0u64; $size];
                let elem_shift = shamt as usize / 64;
                let bits_shift = shamt as usize % 64;
                for i in elem_shift..$size {
                    b[i] = self.0[i - elem_shift] << bits_shift;
                }
                if bits_shift != 0 {
                    for i in elem_shift + 1..$size {
                        b[i] += self.0[i - 1 - elem_shift] >> (64 - bits_shift);
                    }
                }
                Self(b)
            }

            fn wrapping_shr(self, other: u32) -> Self {
                let shamt = other % Self::BITS;
                let mut b = [0u64; $size];
                let elem_shift = shamt as usize / 64;
                let bits_shift = shamt as usize % 64;
                for i in elem_shift..$size {
                    b[i - elem_shift] = self.0[i] >> bits_shift;
                }
                if bits_shift != 0 {
                    for i in elem_shift + 1..$size {
                        b[i - elem_shift - 1] += self.0[i] << (64 - bits_shift);
                    }
                }
                Self(b)
            }

            fn wrapping_sra(self, other: u32) -> Self {
                let shamt = other % Self::BITS;
                let hi =
                    if self.is_negative() && shamt != 0 { Self::MAX_U << (Self::BITS - shamt) } else { Self::MIN_U };
                let lo = self.wrapping_shr(shamt);
                hi | lo
            }

            fn wrapping_sub(self, other: Self) -> Self {
                let mut b = [0u64; $size];
                let mut borrow = false;
                for i in 0..$size {
                    let (r0, borrow0) = self.0[i].overflowing_sub(other.0[i]);
                    let (r1, borrow1) = r0.overflowing_sub(borrow as u64);
                    b[i] = r1;
                    borrow = borrow0 | borrow1
                }
                Self(b)
            }
        }

        impl $name {
            fn words(bits: usize) -> usize {
                debug_assert!(bits > 0);
                1 + (bits - 1) / 64
            }

            #[inline(always)]
            const fn mul_u64(a: u64, b: u64, carry: u64) -> (u64, u64) {
                let (hi, lo) = Self::split_u128(a as u128 * b as u128 + carry as u128);
                (lo, hi)
            }

            // #[inline(always)]
            // const fn split(a: u64) -> (u64, u64) {
            //     (a >> 32, a & 0xFFFF_FFFF)
            // }

            #[inline(always)]
            const fn split_u128(a: u128) -> (u64, u64) {
                ((a >> 64) as _, (a & 0xFFFFFFFFFFFFFFFF) as _)
            }

            fn overflowing_mul_u64(mut self, other: u64) -> (Self, u64) {
                let mut carry = 0u64;

                for d in self.0.iter_mut() {
                    let (res, c) = Self::mul_u64(*d, other, carry);
                    *d = res;
                    carry = c;
                }

                (self, carry)
            }

            fn full_mul_u64(self, by: u64) -> [u64; $size + 1] {
                let (prod, carry) = self.overflowing_mul_u64(by);
                let mut res = [0u64; $size + 1];
                res[..$size].copy_from_slice(&prod.0[..]);
                res[$size] = carry;
                res
            }

            #[inline(always)]
            fn div_mod_word(hi: u64, lo: u64, y: u64) -> (u64, u64) {
                debug_assert!(hi < y);
                let x = (u128::from(hi) << 64) + u128::from(lo);
                let y = u128::from(y);
                ((x / y) as u64, (x % y) as u64)
            }

            #[inline(always)]
            fn add_slice(a: &mut [u64], b: &[u64]) -> bool {
                Self::binop_slice(a, b, u64::overflowing_add)
            }

            #[inline(always)]
            fn sub_slice(a: &mut [u64], b: &[u64]) -> bool {
                Self::binop_slice(a, b, u64::overflowing_sub)
            }

            #[inline(always)]
            fn binop_slice(a: &mut [u64], b: &[u64], binop: impl Fn(u64, u64) -> (u64, bool) + Copy) -> bool {
                let mut c = false;
                a.iter_mut().zip(b.iter()).for_each(|(x, y)| {
                    let (res, carry) = Self::binop_carry(*x, *y, c, binop);
                    *x = res;
                    c = carry;
                });
                c
            }

            #[inline(always)]
            fn binop_carry(a: u64, b: u64, c: bool, binop: impl Fn(u64, u64) -> (u64, bool)) -> (u64, bool) {
                let (res1, overflow1) = b.overflowing_add(u64::from(c));
                let (res2, overflow2) = binop(a, res1);
                (res2, overflow1 || overflow2)
            }

            #[inline]
            fn fits_word(&self) -> bool {
                let arr = self.0;
                for i in 1..$size {
                    if arr[i] != 0 {
                        return false;
                    }
                }
                return true;
            }

            #[inline]
            pub fn bits(&self) -> usize {
                let arr = self.0;
                for i in 1..$size {
                    if arr[$size - i] > 0 {
                        return (0x40 * ($size - i + 1)) - arr[$size - i].leading_zeros() as usize;
                    }
                }
                0x40 - arr[0].leading_zeros() as usize
            }

            fn full_shl(self, shift: u32) -> [u64; $size + 1] {
                debug_assert!(shift < 64);
                let mut u = [0u64; $size + 1];
                let u_lo = self.0[0] << shift;
                let u_hi = self >> (64 - shift);
                u[0] = u_lo;
                u[1..].copy_from_slice(&u_hi.0[..]);
                u
            }

            fn full_shr(u: [u64; $size + 1], shift: u32) -> Self {
                debug_assert!(shift < 64);
                let mut res = Self::ZERO;
                for i in 0..$size {
                    res.0[i] = u[i] >> shift;
                }
                // carry
                if shift > 0 {
                    for i in 1..=$size {
                        res.0[i - 1] |= u[i] << (64 - shift);
                    }
                }
                res
            }

            fn div_mod_small(mut self, other: u64) -> (Self, Self) {
                let mut rem = 0u64;
                self.0.iter_mut().rev().for_each(|d| {
                    let (q, r) = Self::div_mod_word(rem, *d, other);
                    *d = q;
                    rem = r;
                });
                (self, rem.into())
            }

            // See Knuth, TAOCP, Volume 2, section 4.3.1, Algorithm D.
            fn div_mod_knuth(self, mut v: Self, n: usize, m: usize) -> (Self, Self) {
                debug_assert!(self.bits() >= v.bits() && !v.fits_word());
                debug_assert!(n + m <= $size);
                // D1.
                // Make sure 64th bit in v's highest word is set.
                // If we shift both self and v, it won't affect the quotient
                // and the remainder will only need to be shifted back.
                let shift = v.0[n - 1].leading_zeros();
                v = v << shift;
                // u will store the remainder (shifted)
                let mut u = self.full_shl(shift);

                // quotient
                let mut q = Self::ZERO;
                let v_n_1 = v.0[n - 1];
                let v_n_2 = v.0[n - 2];

                // D2. D7.
                // iterate from m downto 0
                for j in (0..=m).rev() {
                    let u_jn = u[j + n];

                    // D3.
                    // q_hat is our guess for the j-th quotient digit
                    // q_hat = min(b - 1, (u_{j+n} * b + u_{j+n-1}) / v_{n-1})
                    // b = 1 << WORD_BITS
                    // Theorem B: q_hat >= q_j >= q_hat - 2
                    let mut q_hat = if u_jn < v_n_1 {
                        let (mut q_hat, mut r_hat) = Self::div_mod_word(u_jn, u[j + n - 1], v_n_1);
                        // this loop takes at most 2 iterations
                        loop {
                            // check if q_hat * v_{n-2} > b * r_hat + u_{j+n-2}
                            let (hi, lo) = Self::split_u128(u128::from(q_hat) * u128::from(v_n_2));
                            if (hi, lo) <= (r_hat, u[j + n - 2]) {
                                break;
                            }
                            // then iterate till it doesn't hold
                            q_hat -= 1;
                            let (new_r_hat, overflow) = r_hat.overflowing_add(v_n_1);
                            r_hat = new_r_hat;
                            // if r_hat overflowed, we're done
                            if overflow {
                                break;
                            }
                        }
                        q_hat
                    } else {
                        // here q_hat >= q_j >= q_hat - 1
                        u64::max_value()
                    };

                    // ex. 20:
                    // since q_hat * v_{n-2} <= b * r_hat + u_{j+n-2},
                    // either q_hat == q_j, or q_hat == q_j + 1

                    // D4.
                    // let's assume optimistically q_hat == q_j
                    // subtract (q_hat * v) from u[j..]
                    let q_hat_v = v.full_mul_u64(q_hat);
                    // u[j..] -= q_hat_v;
                    let c = Self::sub_slice(&mut u[j..], &q_hat_v[..n + 1]);

                    // D6.
                    // actually, q_hat == q_j + 1 and u[j..] has overflowed
                    // highly unlikely ~ (1 / 2^63)
                    if c {
                        q_hat -= 1;
                        // add v to u[j..]
                        let c = Self::add_slice(&mut u[j..], &v.0[..n]);
                        u[j + n] = u[j + n].wrapping_add(u64::from(c));
                    }

                    // D5.
                    q.0[j] = q_hat;
                }

                // D8.
                let remainder = Self::full_shr(u, shift);

                (q, remainder)
            }

            pub fn div_mod(self, other: Self) -> (Self, Self) {
                // use core::cmp::Ordering;

                let my_bits = self.bits();
                let your_bits = other.bits();

                assert!(your_bits != 0, "division by zero");

                // Early return in case we are dividing by a larger number than us
                if my_bits < your_bits {
                    return (Self::ZERO, self);
                }

                if your_bits <= 64 {
                    return self.div_mod_small(other.u64());
                }

                let (n, m) = {
                    let my_words = Self::words(my_bits);
                    let your_words = Self::words(your_bits);
                    (your_words, my_words - your_words)
                };

                self.div_mod_knuth(other, n, m)
            }

            fn div_u(self, other: Self) -> (Self, Self) {
                self.div_mod(other)
            }

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

#[macro_export]
macro_rules! uint_twin_from_impl {
    ($name:ident, $from:ty) => {
        impl core::convert::From<$from> for $name {
            fn from(small: $from) -> Self {
                let mut b = [0u64; Self::BITS as usize >> 6];
                unsafe {
                    core::ptr::copy_nonoverlapping(
                        &small as *const $from as *const u8,
                        b.as_mut_ptr() as *mut u8,
                        <$from>::BITS as usize >> 3,
                    );
                }
                Self(b)
            }
        }
    };
}

construct_eint_twin!(E256, 4);
construct_eint_twin!(E512, 8);
construct_eint_twin!(E1024, 16);
construct_eint_twin!(E2048, 32);
impl_widening_mul_u_twin!(E256, 4);
impl_widening_mul_u_twin!(E512, 8);
impl_widening_mul_u_twin!(E1024, 16);
impl_widening_mul_u_twin!(E2048, 32);
uint_twin_from_impl!(E256, E8);
uint_twin_from_impl!(E256, E16);
uint_twin_from_impl!(E256, E32);
uint_twin_from_impl!(E256, E64);
uint_twin_from_impl!(E256, E128);
uint_twin_from_impl!(E512, E8);
uint_twin_from_impl!(E512, E16);
uint_twin_from_impl!(E512, E32);
uint_twin_from_impl!(E512, E64);
uint_twin_from_impl!(E512, E128);
uint_twin_from_impl!(E512, E256);
uint_twin_from_impl!(E1024, E8);
uint_twin_from_impl!(E1024, E16);
uint_twin_from_impl!(E1024, E32);
uint_twin_from_impl!(E1024, E64);
uint_twin_from_impl!(E1024, E128);
uint_twin_from_impl!(E1024, E256);
uint_twin_from_impl!(E1024, E512);
uint_twin_from_impl!(E2048, E8);
uint_twin_from_impl!(E2048, E16);
uint_twin_from_impl!(E2048, E32);
uint_twin_from_impl!(E2048, E64);
uint_twin_from_impl!(E2048, E128);
uint_twin_from_impl!(E2048, E256);
uint_twin_from_impl!(E2048, E512);
uint_twin_from_impl!(E2048, E1024);

pub mod ll {
    #[link(name = "eint-c-impl", kind = "static")]
    extern "C" {
        pub fn c_widening_mul_u(w: *mut u64, x: *const u64, y: *const u64, digits_count: usize);
        pub fn c_widening_mul_u_128_batch(w: *mut u64, x: *const u64, y: *const u64, batch: usize);
        pub fn c_widening_mul_u_256_batch(w: *mut u64, x: *const u64, y: *const u64, batch: usize);
        pub fn c_widening_mul_u_512_batch(w: *mut u64, x: *const u64, y: *const u64, batch: usize);
        pub fn c_widening_mul_u_1024_batch(w: *mut u64, x: *const u64, y: *const u64, batch: usize);

        pub fn c_wrapping_mul(w: *mut u64, x: *const u64, y: *const u64, digits_count: usize);
        pub fn c_wrapping_mul_128_batch(w: *mut u64, x: *const u64, y: *const u64, batch: usize);
        pub fn c_wrapping_mul_256_batch(w: *mut u64, x: *const u64, y: *const u64, batch: usize);
        pub fn c_wrapping_mul_512_batch(w: *mut u64, x: *const u64, y: *const u64, batch: usize);
        pub fn c_wrapping_mul_1024_batch(w: *mut u64, x: *const u64, y: *const u64, batch: usize);

        pub fn c_overflowing_add(w: *mut u64, x: *const u64, y: *const u64, digits_count: usize) -> u64;
        pub fn c_overflowing_sub(w: *mut u64, x: *const u64, y: *const u64, digits_count: usize) -> u64;
    }

    pub fn widening_mul_u(w: &mut [u8], x: &[u8], y: &[u8]) {
        let digits_count = x.len() >> 3;
        unsafe {
            c_widening_mul_u(
                w.as_mut_ptr() as *mut u64,
                x.as_ptr() as *const u64,
                y.as_ptr() as *const u64,
                digits_count,
            );
        }
    }

    pub fn widening_mul_u_128(w: &mut [u8], x: &[u8], y: &[u8], batch: usize) {
        unsafe {
            c_widening_mul_u_128_batch(
                w.as_ptr() as *mut u64,
                x.as_ptr() as *const u64,
                y.as_ptr() as *const u64,
                batch,
            );
        }
    }

    pub fn widening_mul_u_256(w: &mut [u8], x: &[u8], y: &[u8], batch: usize) {
        unsafe {
            c_widening_mul_u_256_batch(
                w.as_ptr() as *mut u64,
                x.as_ptr() as *const u64,
                y.as_ptr() as *const u64,
                batch,
            );
        }
    }

    pub fn widening_mul_u_512(w: &mut [u8], x: &[u8], y: &[u8], batch: usize) {
        unsafe {
            c_widening_mul_u_512_batch(
                w.as_ptr() as *mut u64,
                x.as_ptr() as *const u64,
                y.as_ptr() as *const u64,
                batch,
            );
        }
    }

    pub fn widening_mul_u_1024(w: &mut [u8], x: &[u8], y: &[u8], batch: usize) {
        unsafe {
            c_widening_mul_u_1024_batch(
                w.as_ptr() as *mut u64,
                x.as_ptr() as *const u64,
                y.as_ptr() as *const u64,
                batch,
            );
        }
    }

    pub fn wrapping_mul_128(w: &mut [u8], x: &[u8], y: &[u8], batch: usize) {
        unsafe {
            c_wrapping_mul_128_batch(w.as_ptr() as *mut u64, x.as_ptr() as *const u64, y.as_ptr() as *const u64, batch);
        }
    }

    pub fn wrapping_mul_256(w: &mut [u8], x: &[u8], y: &[u8], batch: usize) {
        unsafe {
            c_wrapping_mul_256_batch(w.as_ptr() as *mut u64, x.as_ptr() as *const u64, y.as_ptr() as *const u64, batch);
        }
    }

    pub fn wrapping_mul_512(w: &mut [u8], x: &[u8], y: &[u8], batch: usize) {
        unsafe {
            c_wrapping_mul_512_batch(w.as_ptr() as *mut u64, x.as_ptr() as *const u64, y.as_ptr() as *const u64, batch);
        }
    }

    pub fn wrapping_mul_1024(w: &mut [u8], x: &[u8], y: &[u8], batch: usize) {
        unsafe {
            c_wrapping_mul_1024_batch(
                w.as_ptr() as *mut u64,
                x.as_ptr() as *const u64,
                y.as_ptr() as *const u64,
                batch,
            );
        }
    }

    pub fn overflowing_add_128(w: &mut [u8], x: &[u8], y: &[u8]) -> u64 {
        unsafe { c_overflowing_add(w.as_ptr() as *mut u64, x.as_ptr() as *const u64, y.as_ptr() as *const u64, 2) }
    }

    pub fn overflowing_add_256(w: &mut [u8], x: &[u8], y: &[u8]) -> u64 {
        unsafe { c_overflowing_add(w.as_ptr() as *mut u64, x.as_ptr() as *const u64, y.as_ptr() as *const u64, 4) }
    }

    pub fn overflowing_add_512(w: &mut [u8], x: &[u8], y: &[u8]) -> u64 {
        unsafe { c_overflowing_add(w.as_ptr() as *mut u64, x.as_ptr() as *const u64, y.as_ptr() as *const u64, 8) }
    }

    pub fn overflowing_add_1024(w: &mut [u8], x: &[u8], y: &[u8]) -> u64 {
        unsafe { c_overflowing_add(w.as_ptr() as *mut u64, x.as_ptr() as *const u64, y.as_ptr() as *const u64, 16) }
    }

    pub fn overflowing_sub_128(w: &mut [u8], x: &[u8], y: &[u8]) -> u64 {
        unsafe { c_overflowing_sub(w.as_ptr() as *mut u64, x.as_ptr() as *const u64, y.as_ptr() as *const u64, 2) }
    }

    pub fn overflowing_sub_256(w: &mut [u8], x: &[u8], y: &[u8]) -> u64 {
        unsafe { c_overflowing_sub(w.as_ptr() as *mut u64, x.as_ptr() as *const u64, y.as_ptr() as *const u64, 4) }
    }

    pub fn overflowing_sub_512(w: &mut [u8], x: &[u8], y: &[u8]) -> u64 {
        unsafe { c_overflowing_sub(w.as_ptr() as *mut u64, x.as_ptr() as *const u64, y.as_ptr() as *const u64, 8) }
    }

    pub fn overflowing_sub_1024(w: &mut [u8], x: &[u8], y: &[u8]) -> u64 {
        unsafe { c_overflowing_sub(w.as_ptr() as *mut u64, x.as_ptr() as *const u64, y.as_ptr() as *const u64, 16) }
    }
}
