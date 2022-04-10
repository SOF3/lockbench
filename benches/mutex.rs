use std::ops::{Deref, DerefMut};
use criterion::*;

criterion_main!(benches);
criterion_group!(benches, mutex_bench);

#[cfg(feature = "std-mutex")]
type Mutex = std::sync::Mutex<String>;
#[cfg(feature = "std-mutex")]
fn lock(mutex: &Mutex) -> impl Deref<Target = String> + DerefMut + '_ {
    mutex.lock().unwrap()
}
#[cfg(feature = "std-mutex")]
fn try_lock(mutex: &Mutex) -> impl Deref<Target = String> + DerefMut + '_ {
    mutex.try_lock().unwrap()
}

#[cfg(feature = "parking-lot-mutex")]
type Mutex = parking_lot::Mutex<String>;
#[cfg(feature = "parking-lot-mutex")]
fn lock(mutex: &Mutex) -> impl Deref<Target = String> + DerefMut + '_ {
    mutex.lock()
}
#[cfg(feature = "parking-lot-mutex")]
fn try_lock(mutex: &Mutex) -> impl Deref<Target = String> + DerefMut + '_ {
    mutex.try_lock().unwrap()
}

fn mutex_bench(c: &mut Criterion) {
    c.bench_function("mutex lock", |b| {
        let mutex = Mutex::new(String::new());
        b.iter(|| {
            let mut guard = lock(&mutex);
            bb(&mut *guard);
        })
    });
    c.bench_function("mutex try_lock", |b| {
        let mutex = Mutex::new(String::new());
        b.iter(|| {
            let mut guard = try_lock(&mutex);
            bb(&mut *guard);
        })
    });
}

fn bb(string: &mut String) {
    black_box(string);
}
