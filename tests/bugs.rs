use eint::*;

#[test]
fn test_lo_sext_bug_0() {
    let x = E256([0xb12f7788023e73f4, 0xe2aaa5a70e8d29d2, 0x01f281f891d2d8b6, 0xff000000000923b3]);
    let y = E512([
        0xb12f7788023e73f4,
        0xe2aaa5a70e8d29d2,
        0x01f281f891d2d8b6,
        0xff000000000923b3,
        0xffffffffffffffff,
        0xffffffffffffffff,
        0xffffffffffffffff,
        0xffffffffffffffff,
    ]);
    assert_eq!(E512::from(x).lo_sext(), y);

    let x = E256([0xb12f7788023e73f4, 0xe2aaa5a70e8d29d2, 0x01f281f891d2d8b6, 0x00000000000923b3]);
    let y = E512([
        0xb12f7788023e73f4,
        0xe2aaa5a70e8d29d2,
        0x01f281f891d2d8b6,
        0x00000000000923b3,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
    ]);
    assert_eq!(E512::from(x).lo_sext(), y);
}

#[test]
fn test_lowhex_bug_0() {
    let x = E64(0x000000ffffffff12);
    assert_eq!(format!("{:x}", x), String::from("000000ffffffff12"));
    let x = E256([0x012f7788023e73f4, 0xe2aaa5a70e8d29d2, 0x01f281f891d2d8b6, 0x00000000000923b3]);
    assert_eq!(format!("{:x}", x), String::from("00000000000923b301f281f891d2d8b6e2aaa5a70e8d29d2012f7788023e73f4"));
}

#[test]
fn test_overflowing_mul_s_bug_0() {
    let x: u64 = 0x000000ffffffff12;
    let y: u64 = 0xffffffffffffff00;
    let (r0, b0) = Eint::overflowing_mul_s(E64::from(x), E64::from(y));
    let (r1, b1) = (x as i64).overflowing_mul(y as i64);
    assert_eq!(r0, r1.into());
    assert_eq!(r0, E64(r1 as u64));
    assert_eq!(b0, b1);
}

#[test]
fn test_overflowing_mul_s_bug_1() {
    let x: u64 = 0xffffffff00000000;
    let y: u64 = 0xffffff12;
    let (r0, b0) = Eint::overflowing_mul_s(E64::from(x), E64::from(y));
    let (r1, b1) = (x as i64).overflowing_mul(y as i64);
    assert_eq!(r0, r1.into());
    assert_eq!(b0, b1);
}

#[test]
fn test_saturating_sub_u_bug_0() {
    let (r0, b0) = Eint::saturating_sub_u(E64(1), E64(1));
    assert_eq!(r0, E64(0));
    assert_eq!(b0, false);
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
