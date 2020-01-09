#[macro_use]
extern crate criterion;

use criterion::*;

extern crate addcomb_comp;

use std::rc::Rc;

use addcomb_comp::comb::chapter_a::*;

use addcomb_comp::exactset::GElem;
use addcomb_comp::fastset::FastSet;

use addcomb_comp::setlike::HFolds;

fn bench_nus(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);

    let mut group = c.benchmark_group("nu(n, 5, 3)");
    group.sample_size(30);
    group.plot_config(plot_config);

    for n in [10u32, 13u32, 15u32].iter() {
        group.bench_with_input(BenchmarkId::new("Fast", n), n, |b, n| {
            b.iter(|| nu::<FastSet>(black_box(*n), black_box(5), black_box(2), false))
        });
        group.bench_with_input(BenchmarkId::new("Fast<u128>", n), n, |b, n| {
            b.iter(|| nu::<FastSet<u128>>(black_box(*n), black_box(5), black_box(2), false))
        });
        group.bench_with_input(BenchmarkId::new("Exact", n), n, |b, n| {
            b.iter(|| {
                nu::<Vec<GElem>>(
                    black_box(Rc::new(vec![*n])),
                    black_box(5),
                    black_box(2),
                    false,
                )
            })
        });
    }
    group.finish();

    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);

    let mut group = c.benchmark_group("nu_signed_restricted(n, 5, 3)");
    group.sample_size(30);
    group.plot_config(plot_config);

    for n in [10u32, 13u32, 15u32].iter() {
        group.bench_with_input(BenchmarkId::new("Fast", n), n, |b, n| {
            b.iter(|| {
                nu_signed_restricted::<FastSet>(black_box(*n), black_box(5), black_box(2), false)
            })
        });
        group.bench_with_input(BenchmarkId::new("Fast<u128>", n), n, |b, n| {
            b.iter(|| {
                nu_signed_restricted::<FastSet<u128>>(black_box(*n), black_box(5), black_box(2), false)
            })
        });
        group.bench_with_input(BenchmarkId::new("Exact", n), n, |b, n| {
            b.iter(|| {
                nu_signed_restricted::<Vec<GElem>>(
                    black_box(Rc::new(vec![*n])),
                    black_box(5),
                    black_box(2),
                    false,
                )
            })
        });
    }
    group.finish();
    
    let mut group = c.benchmark_group("sumsets");
    let a_fast1: FastSet = (&[1u32, 3, 10, 11, 25]).into();
    let a_fast1_lg: FastSet<u128> = (&[1u32, 3, 10, 11, 25]).into();
    let a_exact1: Vec<GElem> = vec![GElem(vec![1]), GElem(vec![3]), GElem(vec![10]), GElem(vec![11]), GElem(vec![25])];
    let a_fast2: FastSet = (&[1u32, 3, 10, 11, 25, 30, 50, 55, 58, 60]).into();
    let a_fast2_lg: FastSet<u128> = (&[1u32, 3, 10, 11, 25, 30, 50, 55, 58, 60]).into();
    let a_exact2: Vec<GElem> = vec![GElem(vec![1]), GElem(vec![3]), GElem(vec![10]), GElem(vec![11]), GElem(vec![25]), GElem(vec![30]), GElem(vec![50]), GElem(vec![55]), GElem(vec![58]), GElem(vec![60])];
    let g_exact1 = Rc::new(vec![30]);
    let g_exact2 = Rc::new(vec![62]);
    group.sample_size(2000);
    group.bench_function("5-fold sumset of A, |A| = 5, fastset", |b| b.iter(|| black_box(a_fast1.hfold_sumset(black_box(5), 35))));
    group.bench_function("5-fold sumset of A, |A| = 5, fastset<u128>", |b| b.iter(|| black_box(a_fast1_lg.hfold_sumset(black_box(5), 35))));
    group.sample_size(200);
    group.bench_function("5-fold sumset of A, |A| = 5, exactset", |b| b.iter(|| black_box(a_exact1.hfold_sumset(black_box(5), g_exact1.clone()))));
    group.sample_size(2000);
    group.bench_function("5-fold sumset of A, |A| = 10, fastset", |b| b.iter(|| black_box(a_fast2.hfold_sumset(black_box(5), 62))));
    group.bench_function("5-fold sumset of A, |A| = 10, fastset<u128>", |b| b.iter(|| black_box(a_fast2_lg.hfold_sumset(black_box(5), 62))));
    group.sample_size(80);
    group.bench_function("5-fold sumset of A, |A| = 10, exactset", |b| b.iter(|| black_box(a_exact2.hfold_sumset(black_box(5), g_exact2.clone()))));
    group.finish();
}

criterion_group!(benches, bench_nus);
criterion_main!(benches);
