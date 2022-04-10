use std::ops::{Deref, DerefMut};
use criterion::*;

criterion_main!(benches);
criterion_group!(benches, rwlock_bench);

#[cfg(feature = "std-mutex")]
type RwLock = std::sync::RwLock<String>;
#[cfg(feature = "std-mutex")]
fn write(rwlock: &RwLock) -> impl Deref<Target = String> + DerefMut + '_ {
    rwlock.write().unwrap()
}
#[cfg(feature = "std-mutex")]
fn try_write(rwlock: &RwLock) -> impl Deref<Target = String> + DerefMut + '_ {
    rwlock.try_write().unwrap()
}

#[cfg(feature = "parking-lot-mutex")]
type RwLock = parking_lot::RwLock<String>;
#[cfg(feature = "parking-lot-mutex")]
fn write(rwlock: &RwLock) -> impl Deref<Target = String> + DerefMut + '_ {
    rwlock.write()
}
#[cfg(feature = "parking-lot-mutex")]
fn try_write(rwlock: &RwLock) -> impl Deref<Target = String> + DerefMut + '_ {
    rwlock.try_write().unwrap()
}

fn rwlock_bench(c: &mut Criterion) {
    c.bench_function("rwlock lock", |b| {
        let rwlock = RwLock::new(String::new());
        b.iter(|| {
            let mut guard = write(&rwlock);
            bb(&mut *guard);
        })
    });
    c.bench_function("rwlock try_lock", |b| {
        let rwlock = RwLock::new(String::new());
        b.iter(|| {
            let mut guard = try_write(&rwlock);
            bb(&mut *guard);
        })
    });
}

fn bb(string: &mut String) {
    black_box(string);
}
