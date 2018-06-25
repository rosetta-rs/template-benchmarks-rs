# Rust template engine benchmarks

This repo tries to assess Rust template engine performance. Following the
download ratings from [crates.io][crates], these four projects are assessed:

- [Handlebars][handlebars]
- [Tera][tera]
- [Liquid][liquid]
- [Askama][askama]

[crates]: https://crates.io/categories/template-engine
[handlebars]: https://github.com/sunng87/handlebars-rust
[tera]: https://github.com/Keats/tera
[liquid]: https://github.com/cobalt-org/liquid-rust
[askama]: https://github.com/djc/askama

## Results

These are my initial results:

```
     Running target/release/deps/askama_bench-b6ab9b85757821a1

test big_table ... bench:  36,143,579 ns/iter (+/- 1,550,844)
test teams     ... bench:       1,831 ns/iter (+/- 141)

     Running target/release/deps/handlebars_bench-448282773f8f8ba6

test teams ... bench:      29,668 ns/iter (+/- 2,373)

     Running target/release/deps/liquid_bench-8d19f9e196f258b5

test big_table ... bench: 595,751,935 ns/iter (+/- 10,601,412)
test teams     ... bench:      17,235 ns/iter (+/- 3,446)

     Running target/release/deps/tera_bench-7ee6d2a5480335dd

test big_table ... bench: 138,884,726 ns/iter (+/- 3,584,415)
test teams     ... bench:      12,481 ns/iter (+/- 1,691)
```
