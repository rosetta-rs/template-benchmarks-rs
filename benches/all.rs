#[macro_use]
extern crate criterion;
extern crate template_benchmarks_rs;

use criterion::{Criterion, Fun};
use template_benchmarks_rs::{
    fmt, fomat, handlebars, horrorshow_bench, markup_bench, maud_bench, std_write,
};

fn big_table(c: &mut Criterion) {
    c.bench_functions(
        "Big table",
        vec![
            Fun::new("fmt", |b, i| fmt::big_table(b, i)),
            Fun::new("fomat", |b, i| fomat::big_table(b, i)),
            Fun::new("Handlebars", |b, i| handlebars::big_table(b, i)),
            Fun::new("Horrorshow", |b, i| horrorshow_bench::big_table(b, i)),
            Fun::new("Markup", |b, i| markup_bench::big_table(b, i)),
            Fun::new("Maud", |b, i| maud_bench::big_table(b, i)),
            Fun::new("write", |b, i| std_write::big_table(b, i)),
        ],
        100,
    );
}

fn teams(c: &mut Criterion) {
    c.bench_functions(
        "Teams",
        vec![
            Fun::new("fmt", |b, _| fmt::teams(b)),
            Fun::new("fomat", |b, i| fomat::teams(b, i)),
            Fun::new("Handlebars", |b, i| handlebars::teams(b, i)),
            Fun::new("Horrorshow", |b, i| horrorshow_bench::teams(b, i)),
            Fun::new("Markup", |b, i| markup_bench::teams(b, i)),
            Fun::new("Maud", |b, _| maud_bench::teams(b)),
            Fun::new("write", |b, _| std_write::teams(b)),
        ],
        0,
    );
}

criterion_group!(benches, big_table, teams);
criterion_main!(benches);
