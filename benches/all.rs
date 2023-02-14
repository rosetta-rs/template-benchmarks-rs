use criterion::{criterion_group, criterion_main, Criterion};
use template_benchmarks_rs::{
    askama_bench, fomat, handlebars, horrorshow_bench, liquid, markup_bench, maud_bench, ructe,
    sailfish, std_write, tera,
};

fn big_table(c: &mut Criterion) {
    let input = 100;
    let mut group = c.benchmark_group("Big table");
    group.bench_with_input("Askama", &input, |b, i| askama_bench::big_table(b, i));
    group.bench_with_input("fomat", &input, |b, i| fomat::big_table(b, i));
    group.bench_with_input("Handlebars", &input, |b, i| handlebars::big_table(b, i));
    group.bench_with_input("Horrorshow", &input, |b, i| {
        horrorshow_bench::big_table(b, i)
    });
    group.bench_with_input("Liquid", &input, |b, i| liquid::big_table(b, i));
    group.bench_with_input("Markup", &input, |b, i| markup_bench::big_table(b, i));
    group.bench_with_input("Maud", &input, |b, i| maud_bench::big_table(b, i));
    group.bench_with_input("Ructe", &input, |b, i| ructe::big_table(b, i));
    group.bench_with_input("Sailfish", &input, |b, i| sailfish::big_table(b, i));
    group.bench_with_input("Tera", &input, |b, i| tera::big_table(b, i));
    group.bench_with_input("write", &input, |b, i| std_write::big_table(b, i));
    group.finish();
}

fn teams(c: &mut Criterion) {
    let input = 0;
    let mut group = c.benchmark_group("Teams");
    group.bench_with_input("Askama", &input, |b, i| askama_bench::teams(b, i));
    group.bench_with_input("fomat", &input, |b, i| fomat::teams(b, i));
    group.bench_with_input("Handlebars", &input, |b, i| handlebars::teams(b, i));
    group.bench_with_input("Horrorshow", &input, |b, i| horrorshow_bench::teams(b, i));
    group.bench_with_input("Liquid", &input, |b, i| liquid::teams(b, i));
    group.bench_with_input("Markup", &input, |b, i| markup_bench::teams(b, i));
    group.bench_with_input("Maud", &input, |b, i| maud_bench::teams(b, i));
    group.bench_with_input("Ructe", &input, |b, i| ructe::teams(b, i));
    group.bench_with_input("Sailfish", &input, |b, i| sailfish::teams(b, i));
    group.bench_with_input("Tera", &input, |b, i| tera::teams(b, i));
    group.bench_with_input("write", &input, |b, i| std_write::teams(b, i));
    group.finish();
}

criterion_group!(benches, big_table, teams);
criterion_main!(benches);
