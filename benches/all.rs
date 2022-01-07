use criterion::{criterion_group, criterion_main, Criterion, Fun};
use template_benchmarks_rs::{
    askama_bench, fomat, handlebars, horrorshow_bench, liquid, markup_bench, maud_bench, ructe,
    sailfish, std_write, tera,
};

fn big_table(c: &mut Criterion) {
    c.bench_functions(
        "Big table",
        vec![
            Fun::new("Askama", |b, i| askama_bench::big_table(b, i)),
            Fun::new("fomat", |b, i| fomat::big_table(b, i)),
            Fun::new("Handlebars", |b, i| handlebars::big_table(b, i)),
            Fun::new("Horrorshow", |b, i| horrorshow_bench::big_table(b, i)),
            Fun::new("Liquid", |b, i| liquid::big_table(b, i)),
            Fun::new("Markup", |b, i| markup_bench::big_table(b, i)),
            Fun::new("Maud", |b, i| maud_bench::big_table(b, i)),
            Fun::new("Ructe", |b, i| ructe::big_table(b, i)),
            Fun::new("Sailfish", |b, i| sailfish::big_table(b, i)),
            Fun::new("Tera", |b, i| tera::big_table(b, i)),
            Fun::new("write", |b, i| std_write::big_table(b, i)),
        ],
        100,
    );
}

fn teams(c: &mut Criterion) {
    c.bench_functions(
        "Teams",
        vec![
            Fun::new("Askama", |b, i| askama_bench::teams(b, i)),
            Fun::new("fomat", |b, i| fomat::teams(b, i)),
            Fun::new("Handlebars", |b, i| handlebars::teams(b, i)),
            Fun::new("Horrorshow", |b, i| horrorshow_bench::teams(b, i)),
            Fun::new("Liquid", |b, i| liquid::teams(b, i)),
            Fun::new("Markup", |b, i| markup_bench::teams(b, i)),
            Fun::new("Maud", |b, i| maud_bench::teams(b, i)),
            Fun::new("Ructe", |b, i| ructe::teams(b, i)),
            Fun::new("Sailfish", |b, i| sailfish::teams(b, i)),
            Fun::new("Tera", |b, i| tera::teams(b, i)),
            Fun::new("write", |b, i| std_write::teams(b, i)),
        ],
        0,
    );
}

criterion_group!(benches, big_table, teams);
criterion_main!(benches);
