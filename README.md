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

Current results:

```
Big table/Askama        time:   [0.3763 ms 0.3791 ms 0.3819 ms]
Big table/Handlebars    time:   [9.8669 ms 9.9035 ms 9.9468 ms]
Big table/Horrorshow    time:   [0.1192 ms 0.1194 ms 0.1197 ms]
Big table/Liquid        time:   [5.6370 ms 5.6470 ms 5.6573 ms]
Big table/Ructe         time:   [0.4243 ms 0.4266 ms 0.4292 ms]
Big table/Tera          time:   [1.1595 ms 1.1648 ms 1.1704 ms]

Teams/Askama            time:   [ 2.093 us  2.103 us  2.116 us]
Teams/Handlebars        time:   [26.413 us 26.479 us 26.562 us]
Teams/Horrorshow        time:   [ 0.574 us  0.576 us  0.578 us]
Teams/Liquid            time:   [17.542 us 17.715 us 17.926 us]
Teams/Ructe             time:   [ 2.504 us  2.510 us  2.516 us]
Teams/Tera              time:   [12.630 us 12.732 us 12.843 us]
```
