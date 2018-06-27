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
Big table/Askama        time:   [0.3985 ms 0.3999 ms 0.4014 ms]
Big table/Liquid        time:   [5.8400 ms 5.8505 ms 5.8623 ms]
Big table/Tera          time:   [1.1939 ms 1.1995 ms 1.2053 ms]

Teams/Askama            time:   [ 2.277 us  2.282 us  2.288 us]
Teams/Handlebars        time:   [27.889 us 27.992 us 28.105 us]
Teams/Liquid            time:   [17.601 us 17.678 us 17.756 us]
Teams/Tera              time:   [13.849 us 13.977 us 14.108 us]
```
