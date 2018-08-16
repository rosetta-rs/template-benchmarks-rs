#[macro_use]
extern crate criterion;
extern crate template_benchmarks_rs;

use criterion::{Criterion, Fun};
use template_benchmarks_rs::{askama_bench, handlebars, horrorshow_bench, liquid, ructe, tera};

fn big_table(c: &mut Criterion) {
    c.bench_functions(
        "Big table",
        vec![
            Fun::new("Askama", |b, i| askama_bench::big_table(b, i)),
            Fun::new("Handlebars", |b, i| handlebars::big_table(b, i)),
            Fun::new("Horrorshow", |b, i| horrorshow_bench::big_table(b, i)),
            Fun::new("Liquid", |b, i| liquid::big_table(b, i)),
            Fun::new("Ructe", |b, i| ructe::big_table(b, i)),
            Fun::new("Tera", |b, i| tera::big_table(b, i)),
        ],
        50,
    );
}

fn teams(c: &mut Criterion) {
    c.bench_functions(
        "Teams",
        vec![
            Fun::new("Askama", |b, i| askama_bench::teams(b, i)),
            Fun::new("Handlebars", |b, i| handlebars::teams(b, i)),
            Fun::new("Horrorshow", |b, i| horrorshow_bench::teams(b, i)),
            Fun::new("Liquid", |b, i| liquid::teams(b, i)),
            Fun::new("Ructe", |b, i| ructe::teams(b, i)),
            Fun::new("Tera", |b, i| tera::teams(b, i)),
        ],
        0,
    );
}

criterion_group!(benches, big_table, teams);
criterion_main!(benches);
