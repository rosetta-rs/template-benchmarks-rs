use criterion::{criterion_group, criterion_main, Criterion};
use template_benchmarks_rs::{
    askama_bench, fomat, handlebars, horrorshow_bench, liquid, markup_bench, maud_bench, ructe,
    sailfish, std_write, tera,
};

fn big_table(c: &mut Criterion) {
    let input = 100;

    let mut group = c.benchmark_group("Big table");

    group.bench_with_input("Askama", &input, askama_bench::big_table);
    group.bench_with_input("fomat", &input, fomat::big_table);
    group.bench_with_input("Handlebars", &input, handlebars::big_table);
    group.bench_with_input("Horrorshow", &input, horrorshow_bench::big_table);
    group.bench_with_input("Liquid", &input, liquid::big_table);
    group.bench_with_input("Markup", &input, markup_bench::big_table);
    group.bench_with_input("Maud", &input, maud_bench::big_table);
    group.bench_with_input("Ructe", &input, ructe::big_table);
    group.bench_with_input("Sailfish", &input, sailfish::big_table);
    group.bench_with_input("Tera", &input, tera::big_table);
    group.bench_with_input("write", &input, std_write::big_table);

    group.finish();
}

fn teams(c: &mut Criterion) {
    let input = 0;

    let mut group = c.benchmark_group("Teams");

    group.bench_with_input("Askama", &input, askama_bench::teams);
    group.bench_with_input("fomat", &input, fomat::teams);
    group.bench_with_input("Handlebars", &input, handlebars::teams);
    group.bench_with_input("Horrorshow", &input, horrorshow_bench::teams);
    group.bench_with_input("Liquid", &input, liquid::teams);
    group.bench_with_input("Markup", &input, markup_bench::teams);
    group.bench_with_input("Maud", &input, maud_bench::teams);
    group.bench_with_input("Ructe", &input, ructe::teams);
    group.bench_with_input("Sailfish", &input, sailfish::teams);
    group.bench_with_input("Tera", &input, tera::teams);
    group.bench_with_input("write", &input, std_write::teams);

    group.finish();
}

criterion_group!(benches, big_table, teams);
criterion_main!(benches);
