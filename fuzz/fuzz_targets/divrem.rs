#![no_main]
use eint::*;
use libfuzzer_sys::fuzz_target;

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

fuzz_target!(|data: (u64, u64)| {
    let x = data.0;
    let y = data.1;
    {
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
    {
        let r0 = Eint::wrapping_div_u(E64::from(x), E64::from(y));
        let r1 = Eint::wrapping_div_u(T64::recv(x), T64::recv(y));
        let r2 = if y == 0 { u64::MAX } else { x / y };
        assert_eq!(r0, r1.into());
        assert_eq!(r0, E64(r2));
    }
    {
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
    {
        let r0 = Eint::wrapping_rem_u(E64::from(x), E64::from(y));
        let r1 = Eint::wrapping_rem_u(T64::recv(x), T64::recv(y));
        let r2 = if y == 0 { x } else { x % y };
        assert_eq!(r0, r1.into());
        assert_eq!(r0, E64(r2));
    }
});
