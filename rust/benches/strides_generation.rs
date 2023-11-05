use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[allow(unused_variables)]
fn cumulative_product(shape: &[usize]) {
    let mut strides: Vec<usize> = shape
        .iter()
        .rev()
        .scan(1, |state, &x| {
            *state *= x;
            Some(*state)
        })
        .collect();
    let size = strides.pop().unwrap_or_default();
    strides.reverse();
    strides.push(1);
}

#[allow(unused_variables)]
fn cumulative_division(shape: &[usize]) {
    let mut strides: Vec<usize> = Vec::with_capacity(shape.len());
    let size: usize = shape.iter().product();
    let mut stride = size;
    for &dimsize in shape.iter() {
        stride /= dimsize;
        strides.push(stride);
    }
}

fn benchmark(c: &mut Criterion) {
    let mut shape: Vec<usize> = (40..50).collect();
    c.bench_function("cumprod", |b| {
        b.iter(|| cumulative_product(black_box(&shape)))
    });
    c.bench_function("cumdiv", |b| {
        b.iter(|| cumulative_division(black_box(&shape)))
    });

    shape = (400..500).collect();
    c.bench_function("cumprod (large volume)", |b| {
        b.iter(|| cumulative_product(black_box(&shape)))
    });
    c.bench_function("cumdiv (large volume)", |b| {
        b.iter(|| cumulative_division(black_box(&shape)))
    });

    shape = (4000..5000).collect();
    c.bench_function("cumprod (extreme)", |b| {
        b.iter(|| cumulative_product(black_box(&shape)))
    });
    c.bench_function("cumdiv (extreme)", |b| {
        b.iter(|| cumulative_division(black_box(&shape)))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
