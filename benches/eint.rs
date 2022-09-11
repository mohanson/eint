use criterion::{black_box, criterion_group, criterion_main, Criterion};
use eint::*;

pub fn e256_get(c: &mut Criterion) {
    let mem = [
        0xf4, 0x73, 0x3e, 0x02, 0x88, 0x77, 0x2f, 0xb1, 0xd2, 0x29, 0x8d, 0x0e, 0xa7, 0xa5, 0xaa, 0xe2, 0xb6, 0xd8,
        0xd2, 0x91, 0xf8, 0x81, 0xf2, 0x01, 0xb3, 0x23, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    c.bench_function("e256_get", |b| b.iter(|| black_box(E256::get(&mem))));
}

pub fn e256_put(c: &mut Criterion) {
    let one = E256([0xb12f7788023e73f4, 0xe2aaa5a70e8d29d2, 0x01f281f891d2d8b6, 0x00000000000923b3]);
    let mut mem = [0u8; 32];
    c.bench_function("e256_put", |b| b.iter(|| black_box(one.put(&mut mem))));
}

pub fn e256_overflowing_add(c: &mut Criterion) {
    let one = [
        0xf4, 0x73, 0x3e, 0x02, 0x88, 0x77, 0x2f, 0xb1, 0xd2, 0x29, 0x8d, 0x0e, 0xa7, 0xa5, 0xaa, 0xe2, 0xb6, 0xd8,
        0xd2, 0x91, 0xf8, 0x81, 0xf2, 0x01, 0xb3, 0x23, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let two: [u8; 32] = [
        0x5d, 0x57, 0x4f, 0x5f, 0x88, 0xf2, 0x17, 0x1d, 0x97, 0x0c, 0x59, 0x93, 0x64, 0x10, 0xbc, 0x75, 0x41, 0x21,
        0xec, 0x91, 0x62, 0xfc, 0x80, 0x00, 0x7d, 0x0d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let mut w = E256::from(0u128);
    let mut overflowing: bool = false;
    let one = E256::get(&one[..]);
    let two = E256::get(&two[..]);
    c.bench_function("e256_overflowing_add", |b| {
        b.iter(|| black_box((w, overflowing) = E256::overflowing_add_u(one, two)))
    });
}

pub fn e256_overflowing_sub(c: &mut Criterion) {
    let one = [
        0xf4, 0x73, 0x3e, 0x02, 0x88, 0x77, 0x2f, 0xb1, 0xd2, 0x29, 0x8d, 0x0e, 0xa7, 0xa5, 0xaa, 0xe2, 0xb6, 0xd8,
        0xd2, 0x91, 0xf8, 0x81, 0xf2, 0x01, 0xb3, 0x23, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let two: [u8; 32] = [
        0x5d, 0x57, 0x4f, 0x5f, 0x88, 0xf2, 0x17, 0x1d, 0x97, 0x0c, 0x59, 0x93, 0x64, 0x10, 0xbc, 0x75, 0x41, 0x21,
        0xec, 0x91, 0x62, 0xfc, 0x80, 0x00, 0x7d, 0x0d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let mut w = E256::from(0u128);
    let mut overflowing: bool = false;
    let one = E256::get(&one[..]);
    let two = E256::get(&two[..]);
    c.bench_function("e256_overflowing_sub", |b| {
        b.iter(|| black_box((w, overflowing) = E256::overflowing_sub_u(one, two)))
    });
}

pub fn e256_widening_mul_u(c: &mut Criterion) {
    let one = E256([0xb12f7788023e73f4, 0xe2aaa5a70e8d29d2, 0x01f281f891d2d8b6, 0x00000000000923b3]);
    let two = E256([0x1d17f2885f4f575d, 0x75bc106493590c97, 0x0080fc6291ec2141, 0x0000000000000d7d]);
    c.bench_function("e256_widening_mul_u", |b| b.iter(|| black_box(one.widening_mul_u(two))));
}

pub fn e256_wrapping_div_s(c: &mut Criterion) {
    let one = E256([0xb12f7788023e73f4, 0xe2aaa5a70e8d29d2, 0x01f281f891d2d8b6, 0x00000000000923b3]);
    let two = E256([0x1d17f2885f4f575d, 0x75bc106493590c97, 0x0080fc6291ec2141, 0x0000000000000d7d]);
    c.bench_function("e256_wrapping_div_s", |b| b.iter(|| black_box(one.wrapping_div_s(two))));
}

pub fn e256_wrapping_div_u(c: &mut Criterion) {
    let one = E256([0xb12f7788023e73f4, 0xe2aaa5a70e8d29d2, 0x01f281f891d2d8b6, 0x00000000000923b3]);
    let two = E256([0x1d17f2885f4f575d, 0x75bc106493590c97, 0x0080fc6291ec2141, 0x0000000000000d7d]);
    c.bench_function("e256_wrapping_div_u", |b| b.iter(|| black_box(one.wrapping_div_u(two))));
}

pub fn e256_wrapping_mul(c: &mut Criterion) {
    let one = E256([0xb12f7788023e73f4, 0xe2aaa5a70e8d29d2, 0x01f281f891d2d8b6, 0x00000000000923b3]);
    let two = E256([0x1d17f2885f4f575d, 0x75bc106493590c97, 0x0080fc6291ec2141, 0x0000000000000d7d]);
    c.bench_function("e256_wrapping_mul", |b| b.iter(|| black_box(one.wrapping_mul(two))));
}

pub fn e256_wrapping_rem_s(c: &mut Criterion) {
    let one = E256([0xb12f7788023e73f4, 0xe2aaa5a70e8d29d2, 0x01f281f891d2d8b6, 0x00000000000923b3]);
    let two = E256([0x1d17f2885f4f575d, 0x75bc106493590c97, 0x0080fc6291ec2141, 0x0000000000000d7d]);
    c.bench_function("e256_wrapping_rem_s", |b| b.iter(|| black_box(one.wrapping_rem_s(two))));
}

pub fn e256_wrapping_rem_u(c: &mut Criterion) {
    let one = E256([0xb12f7788023e73f4, 0xe2aaa5a70e8d29d2, 0x01f281f891d2d8b6, 0x00000000000923b3]);
    let two = E256([0x1d17f2885f4f575d, 0x75bc106493590c97, 0x0080fc6291ec2141, 0x0000000000000d7d]);
    c.bench_function("e256_wrapping_rem_u", |b| b.iter(|| black_box(one.wrapping_rem_u(two))));
}

criterion_group!(
    benches,
    e256_get,
    e256_put,
    e256_overflowing_add,
    e256_overflowing_sub,
    e256_widening_mul_u,
    e256_wrapping_div_s,
    e256_wrapping_div_u,
    e256_wrapping_mul,
    e256_wrapping_rem_s,
    e256_wrapping_rem_u,
);
criterion_main!(benches);
