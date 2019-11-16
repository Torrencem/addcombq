#[macro_use]
extern crate criterion;

use criterion::*;

extern crate addcombq;

use std::rc::Rc;

use addcombq::comb::chapter_a::*;

use addcombq::fastset::FastSet;
use addcombq::exactset::GElem;

fn bench_nus(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default()
        .summary_scale(AxisScale::Logarithmic);

    let mut group = c.benchmark_group("nu(n, 5, 3)");
    group.sample_size(15);
    group.plot_config(plot_config);
    
    for n in [10u32, 13u32, 15u32, 17u32, 20u32, 22u32].iter() {
        group.bench_with_input(BenchmarkId::new("Fast", n), n, |b, n| b.iter(|| nu::<FastSet>(black_box(*n), black_box(5), black_box(2), false)));
        group.bench_with_input(BenchmarkId::new("Exact", n), n, |b, n| b.iter(|| nu::<Vec<GElem>>(black_box(Rc::new(vec![*n])), black_box(5), black_box(2), false)));
    }
    group.finish();

    let plot_config = PlotConfiguration::default()
        .summary_scale(AxisScale::Logarithmic);

    let mut group = c.benchmark_group("nu_signed_restricted(n, 5, 3)");
    group.sample_size(15);
    group.plot_config(plot_config);
    
    for n in [10u32, 13u32, 15u32, 17u32, 20u32, 22u32].iter() {
        group.bench_with_input(BenchmarkId::new("Fast", n), n, |b, n| b.iter(|| nu_signed_restricted::<FastSet>(black_box(*n), black_box(5), black_box(2), false)));
        group.bench_with_input(BenchmarkId::new("Exact", n), n, |b, n| b.iter(|| nu_signed_restricted::<Vec<GElem>>(black_box(Rc::new(vec![*n])), black_box(5), black_box(2), false)));
    }
    group.finish();
}

criterion_group!(benches, bench_nus);
criterion_main!(benches);
