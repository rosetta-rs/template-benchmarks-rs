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

Big table/write         time:   [523.04 ns 525.76 ns 529.26 ns]
Big table/fomat         time:   [546.58 ns 550.28 ns 554.21 ns]
Big table/Horrorshow    time:   [70.697 us 71.594 us 72.523 us]
Big table/Askama        time:   [206.25 us 207.56 us 208.87 us]
Big table/Tera          time:   [764.59 us 770.04 us 776.26 us]
Big table/Liquid        time:   [3.5647 ms 3.5975 ms 3.6311 ms]
Big table/Handlebars    time:   [6.2990 ms 6.4261 ms 6.5593 ms]

Teams/write             time:   [24.129 ns 24.181 ns 24.243 ns]
Teams/fomat             time:   [26.103 ns 26.200 ns 26.336 ns]
Teams/Horrorshow        time:   [343.80 ns 346.20 ns 349.62 ns]
Teams/Askama            time:   [1.2350 us 1.2409 us 1.2473 us]
Teams/Tera              time:   [8.1809 us 8.2071 us 8.2376 us]
Teams/Liquid            time:   [10.473 us 10.504 us 10.538 us]
Teams/Handlebars        time:   [16.225 us 16.293 us 16.370 us]
```
