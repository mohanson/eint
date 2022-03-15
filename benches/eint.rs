use criterion::{black_box, criterion_group, criterion_main, Criterion};
use eint::*;

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
    e256_wrapping_div_s,
    e256_wrapping_div_u,
    e256_wrapping_rem_s,
    e256_wrapping_rem_u,
    e256_wrapping_mul,
);
criterion_main!(benches);
