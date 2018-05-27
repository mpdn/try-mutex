extern crate try_mutex;
#[macro_use] extern crate criterion;

use std::sync::Mutex;
use try_mutex::TryMutex;
use criterion::{Criterion, black_box};

fn build_try() {
    for _ in 0..100000 {
        black_box(TryMutex::new(false));
    }
}

fn build_std() {
    for _ in 0..100000 {
        black_box(Mutex::new(false));
    }
}

fn lock_try() {
    let m = TryMutex::new(false);
    for _ in 0..100000 {
        let mut g = m.try_lock().unwrap();
        *g = !*g;
    }
}

fn lock_std() {
    let m = Mutex::new(false);
    for _ in 0..100000 {
        let mut g = m.try_lock().unwrap();
        *g = !*g;
    }
}

fn contested_try() {
    let m = TryMutex::new(false);
    for _ in 0..100000 {
        let mut g = m.try_lock().unwrap();
        black_box(m.try_lock());
        *g = !*g;
    }
}

fn contested_std() {
    let m = Mutex::new(false);
    for _ in 0..100000 {
        let mut g = m.try_lock().unwrap();
        std::mem::drop(black_box(m.try_lock()));
        *g = !*g;
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("build_try", |b| b.iter(build_try));
    c.bench_function("build_std", |b| b.iter(build_std));
    c.bench_function("lock_try", |b| b.iter(lock_try));
    c.bench_function("lock_std", |b| b.iter(lock_std));
    c.bench_function("contested_try", |b| b.iter(contested_try));
    c.bench_function("contested_std", |b| b.iter(contested_std));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
