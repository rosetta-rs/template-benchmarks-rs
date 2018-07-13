# Rust template engine benchmarks

This repo tries to assess Rust template engine performance. Following the
download ratings from [crates.io][crates], these four projects are assessed:

- [Handlebars][handlebars]
- [Tera][tera]
- [Liquid][liquid]
- [Askama][askama]
- [Fomat][fomat]

[crates]: https://crates.io/categories/template-engine
[handlebars]: https://github.com/sunng87/handlebars-rust
[tera]: https://github.com/Keats/tera
[liquid]: https://github.com/cobalt-org/liquid-rust
[askama]: https://github.com/djc/askama
[fomat]: https://github.com/krdln/fomat-macros
format: rust std format!

## Results

Current results:

```
test on Intel(R) Core(TM) i7-6700K CPU @ 4.00GHz
Big table/fomat         time:   [555.00 ns 559.41 ns 564.66 ns]
Big table/format        time:   [570.29 ns 577.65 ns 585.73 ns]
Big table/Horrorshow    time:   [68.954 us 69.240 us 69.573 us]
Big table/Askama        time:   [206.32 us 207.04 us 207.84 us]
Big table/Tera          time:   [754.16 us 764.86 us 775.89 us]
Big table/Handlebars    time:   [6.2264 ms 6.2349 ms 6.2442 ms]
Big table/Liquid        time:   [3.5014 ms 3.5058 ms 3.5105 ms]

Teams/format            time:   [24.819 ns 24.901 ns 24.997 ns]
Teams/fomat             time:   [27.168 ns 27.302 ns 27.477 ns]
Teams/Horrorshow        time:   [349.35 ns 354.03 ns 359.65 ns]
Teams/Askama            time:   [1.2288 us 1.2460 us 1.2690 us]
Teams/Tera              time:   [8.6426 us 8.7004 us 8.7660 us]
Teams/Liquid            time:   [10.885 us 10.948 us 11.020 us]
Teams/Handlebars        time:   [18.000 us 18.154 us 18.311 us]
```
