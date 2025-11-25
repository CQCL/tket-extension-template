use std::hint::black_box;

use criterion::{BatchSize, Criterion, criterion_group};
use tket::Hugr;
use tket::hugr::ops::FuncDefn;
use tket::hugr::types::Signature;
use tket_extension_template::example_remove_contents;

fn build_sample_hugr() -> Hugr {
    Hugr::new_with_entrypoint(FuncDefn::new("bench", Signature::new(vec![], vec![]))).unwrap()
}

fn bench_example_remove_contents(c: &mut Criterion) {
    c.bench_function("example_remove_contents", |b| {
        b.iter_batched(
            build_sample_hugr,
            |mut hugr| {
                example_remove_contents(black_box(&mut hugr)).expect("failed to remove contents")
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = bench_example_remove_contents,
}
