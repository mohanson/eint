use criterion::{black_box, criterion_group, criterion_main, Criterion};
use eint::*;

pub fn e256_get(c: &mut Criterion) {
    let mem = [
        0xf4, 0x73, 0x3e, 0x02, 0x88, 0x77, 0x2f, 0xb1, 0xd2, 0x29, 0x8d, 0x0e, 0xa7, 0xa5, 0xaa, 0xe2, 0xb6, 0xd8,
        0xd2, 0x91, 0xf8, 0x81, 0xf2, 0x01, 0xb3, 0x23, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    c.bench_function("e256_get", |b| b.iter(|| black_box(E256::get(&mem))));
}

pub fn e256_get_unsafe(c: &mut Criterion) {
    let mem = [
        0xf4, 0x73, 0x3e, 0x02, 0x88, 0x77, 0x2f, 0xb1, 0xd2, 0x29, 0x8d, 0x0e, 0xa7, 0xa5, 0xaa, 0xe2, 0xb6, 0xd8,
        0xd2, 0x91, 0xf8, 0x81, 0xf2, 0x01, 0xb3, 0x23, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    c.bench_function("e256_get_unsafe", |b| b.iter(|| black_box(E256::get_unsafe(&mem))));
}

pub fn e256_put(c: &mut Criterion) {
    let one = E256(
        E128(0xe2aaa5a70e8d29d2b12f7788023e73f4),
        E128(0x00000000000923b301f281f891d2d8b6),
    );
    let mut mem = [0u8; 32];
    c.bench_function("e256_put", |b| b.iter(|| black_box(one.put(&mut mem))));
}

pub fn e256_wrapping_div_s(c: &mut Criterion) {
    let one = E256(
        E128(0xe2aaa5a70e8d29d2b12f7788023e73f4),
        E128(0x00000000000923b301f281f891d2d8b6),
    );
    let two = E256(
        E128(0x75bc106493590c971d17f2885f4f575d),
        E128(0x0000000000000d7d0080fc6291ec2141),
    );
    c.bench_function("e256_wrapping_div_s", |b| b.iter(|| black_box(one.wrapping_div_s(two))));
}

pub fn e256_wrapping_div_u(c: &mut Criterion) {
    let one = E256(
        E128(0xe2aaa5a70e8d29d2b12f7788023e73f4),
        E128(0x00000000000923b301f281f891d2d8b6),
    );
    let two = E256(
        E128(0x75bc106493590c971d17f2885f4f575d),
        E128(0x0000000000000d7d0080fc6291ec2141),
    );
    c.bench_function("e256_wrapping_div_u", |b| b.iter(|| black_box(one.wrapping_div_u(two))));
}

pub fn e256_wrapping_rem_s(c: &mut Criterion) {
    let one = E256(
        E128(0xe2aaa5a70e8d29d2b12f7788023e73f4),
        E128(0x00000000000923b301f281f891d2d8b6),
    );
    let two = E256(
        E128(0x75bc106493590c971d17f2885f4f575d),
        E128(0x0000000000000d7d0080fc6291ec2141),
    );
    c.bench_function("e256_wrapping_rem_s", |b| b.iter(|| black_box(one.wrapping_rem_s(two))));
}

pub fn e256_wrapping_rem_u(c: &mut Criterion) {
    let one = E256(
        E128(0xe2aaa5a70e8d29d2b12f7788023e73f4),
        E128(0x00000000000923b301f281f891d2d8b6),
    );
    let two = E256(
        E128(0x75bc106493590c971d17f2885f4f575d),
        E128(0x0000000000000d7d0080fc6291ec2141),
    );
    c.bench_function("e256_wrapping_rem_u", |b| b.iter(|| black_box(one.wrapping_rem_u(two))));
}

pub fn e256_wrapping_mul(c: &mut Criterion) {
    let one = E256(
        E128(0xe2aaa5a70e8d29d2b12f7788023e73f4),
        E128(0x00000000000923b301f281f891d2d8b6),
    );
    let two = E256(
        E128(0x75bc106493590c971d17f2885f4f575d),
        E128(0x0000000000000d7d0080fc6291ec2141),
    );
    c.bench_function("e256_wrapping_mul", |b| b.iter(|| black_box(one.wrapping_mul(two))));
}

criterion_group!(
    benches,
    e256_get,
    e256_get_unsafe,
    e256_put,
    e256_wrapping_div_s,
    e256_wrapping_div_u,
    e256_wrapping_rem_s,
    e256_wrapping_rem_u,
    e256_wrapping_mul,
);
criterion_main!(benches);
