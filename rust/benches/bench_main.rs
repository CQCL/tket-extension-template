//! Benchmarks for the tket extension crate.

#[allow(dead_code)]
mod benchmarks;

use criterion::criterion_main;

criterion_main! {
    benchmarks::example::benches,
}
