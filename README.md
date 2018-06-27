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
Big table/Askama        time:   [0.3615 ms 0.3633 ms 0.3654 ms]                             
Big table/Handlebars    time:   [7.2553 ms 7.2673 ms 7.2812 ms]
Big table/Horrorshow    time:   [0.1130 ms 0.1135 ms 0.1140 ms]                                 
Big table/Liquid        time:   [5.6857 ms 5.6959 ms 5.7071 ms]                              
Big table/Tera          time:   [1.1846 ms 1.1879 ms 1.1919 ms]                            

Teams/Askama            time:   [ 2.126 us  2.131 us  2.137 us]                          
Teams/Handlebars        time:   [27.815 us 27.884 us 27.971 us]                              
Teams/Horrorshow        time:   [ 0.541 us  0.542 us  0.543 us]                              
Teams/Liquid            time:   [17.445 us 17.551 us 17.675 us]                          
Teams/Tera              time:   [13.104 us 13.161 us 13.226 us]                        
```
