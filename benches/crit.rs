#![feature(allocator_api)]
use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use exper_allocator_api::{ma_init, MyAllocator, Protocol};

#[inline(never)]
fn validate(i: usize, left: u64, right: u64) {
    let i_u64 = i as u64;
    if left != i_u64 || right != i_u64 + 1 {
        panic!(
            "Expected left={i} found {left}, right={} found {right}",
            i + 1
        )
    }
}

#[inline(never)]
fn ma_test_1() {
    let i = 0;
    let m = black_box(Protocol::Add {
        left: i,
        right: i + 1,
    });
    let msg = black_box(Box::new_in(m, MyAllocator));
    match *msg {
        Protocol::Add { left, right } => {
            validate(i as usize, left, right);
        }
    }
}

#[inline(never)]
fn ma_test(count: u64) {
    let mut allocated = Vec::<Box<Protocol, MyAllocator>>::with_capacity(count as usize);
    for i in 0..count {
        let m = Protocol::Add {
            left: i,
            right: i + 1,
        };
        let msg = Box::new_in(m, MyAllocator);
        allocated.push(msg);
    }
    for (i, msg) in allocated.into_iter().enumerate() {
        match *msg {
            Protocol::Add { left, right } => {
                validate(i, left, right);
            }
        }
    }
}

#[allow(unused)]
fn crit_ma_test_1(c: &mut Criterion) {
    c.bench_function("crit_ma_test_1", |b| {
        ma_init(100);
        b.iter(|| {
            ma_test_1();
        });
    });
}

#[inline(never)]
fn ga_test_1() {
    let i = 0;
    let m = black_box(Protocol::Add {
        left: i,
        right: i + 1,
    });
    let msg = black_box(Box::new(m));
    match *msg {
        Protocol::Add { left, right } => {
            validate(i as usize, left, right);
        }
    }
}

#[allow(unused)]
fn crit_ga_test_1(c: &mut Criterion) {
    c.bench_function("crit_ga_test_1", |b| {
        ma_init(100);
        b.iter(|| {
            ga_test_1();
        });
    });
}

#[inline(never)]
fn ga_test(count: u64) {
    let mut allocated = Vec::<Box<Protocol>>::with_capacity(count as usize);
    for i in 0..count {
        let m = Protocol::Add {
            left: i,
            right: i + 1,
        };
        let msg = black_box(Box::new(m));
        allocated.push(msg);
    }
    for (i, msg) in allocated.into_iter().enumerate() {
        match *msg {
            Protocol::Add { left, right } => {
                validate(i, left, right);
            }
        }
    }
}


#[allow(unused)]
fn crit_ma_init_100_outside_iter(c: &mut Criterion) {
    c.bench_function("crit_ma_100_outside_iter", |b| {
        ma_init(100);
        b.iter(|| {
            ma_test(100);
        });
    });
}

#[allow(unused)]
fn crit_ma_init_100_inside_iter(c: &mut Criterion) {
    c.bench_function("crit_ma_100_inside_iter", |b| {
        b.iter(|| {
            ma_init(100);
            ma_test(100);
        });
    });
}

#[allow(unused)]
fn crit_ga_100(c: &mut Criterion) {
    c.bench_function("crit_ga_100", |b| {
        b.iter(|| {
            ga_test(100);
        });
    });
}

criterion_group!(
    benches,
    crit_ma_test_1,
    crit_ga_test_1,
    crit_ma_init_100_outside_iter,
    crit_ma_init_100_inside_iter,
    crit_ga_100
);
criterion_main!(benches);
