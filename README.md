# Templates

- [fmt][fmt]: Formats the value using the given formatter.
- [fomat][fomat]: alternative syntax for print/write/format-like macros with a small templating language
- [handlebars][handlebars]: Rust templating with Handlebars templating language.
- [horrorshow][horrorshow]: a templating library written in rust macros
- [markup][markup]: A blazing fast, type-safe template engine for Rust.
- [maud][maud]: Compile-time HTML templates for Rust
- [write!][write]: the std library `write!` macro


[fmt]: https://doc.rust-lang.org/std/fmt/trait.Display.html#tymethod.fmt
[fomat]: https://github.com/krdln/fomat-macros
[handlebars]: https://github.com/sunng87/handlebars-rust
[horrorshow]: https://github.com/Stebalien/horrorshow-rs
[markup]: https://github.com/utkarshkukreti/markup.rs
[maud]: https://github.com/lfairy/maud
[write]: https://doc.rust-lang.org/std/macro.write.html

## Results

As a [violin plot] generated by [Criterion]:

![Big table violin plot](big-table.svg)
![Teams violin plot](teams.svg)

[violin plot]: https://en.wikipedia.org/wiki/Violin_plot
[criterion]: https://github.com/bheisler/criterion.rs

Numbers, as output by Criterion:

```java
Big Table
Markup                  [163.47 us 164.06 us 164.68 us] 
fomat                   [163.56 us 164.43 us 165.34 us] 
fmt                     [189.84 us 191.97 us 193.77 us] 
Maud                    [215.58 us 216.63 us 217.75 us] 
Horrorshow              [243.60 us 244.46 us 245.29 us] 
write                   [296.67 us 298.13 us 299.58 us] 
Handlebars              [23.853 ms 24.062 ms 24.418 ms] 

Teams
fmt                     [233.58 ns 235.20 ns 236.63 ns] 
Markup                  [241.48 ns 242.55 ns 243.67 ns] 
fomat                   [273.36 ns 274.62 ns 275.94 ns] 
Maud                    [285.82 ns 287.12 ns 288.51 ns] 
Horrorshow              [397.82 ns 401.50 ns 405.21 ns] 
write                   [419.33 ns 421.40 ns 423.53 ns] 
Handlebars              [16.568 us 16.656 us 16.748 us] 
```

## Running the benchmarks

	just bench

## For extracting benchmarks to file
	just log filename
## To sort benchmarks
	just bigtable
	just teams
[Just](https://github.com/casey/just) must be installed to use the just commands.

Plots will be rendered if `gnuplot` is installed and will be available in the `target/criterion` folder.
