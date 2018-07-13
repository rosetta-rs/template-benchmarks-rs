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
format: std format!
## Results

Current results:

```
Big table/format        time:   [544.06 ns 551.34 ns 559.22 ns]
Big table/Horrorshow    time:   [70.698 us 71.822 us 72.993 us]
Big table/Askama        time:   [202.15 us 202.69 us 203.31 us]
Big table/Tera          time:   [723.69 us 731.77 us 740.79 us]
Big table/Liquid        time:   [3.5118 ms 3.5522 ms 3.5972 ms]
Big table/Handlebars    time:   [6.2366 ms 6.2983 ms 6.3702 ms]

Teams/format            time:   [23.186 ns 23.250 ns 23.334 ns]                    
Teams/Horrorshow        time:   [345.43 ns 347.31 ns 349.75 ns]
Teams/Askama            time:   [1.2087 us 1.2133 us 1.2196 us]
Teams/Tera              time:   [7.9691 us 7.9936 us 8.0235 us]
Teams/Liquid            time:   [10.348 us 10.378 us 10.414 us]
Teams/Handlebars        time:   [17.202 us 17.510 us 17.831 us]
```
