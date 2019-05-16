#![feature(proc_macro_hygiene)]

extern crate horrorshow;
extern crate criterion;
#[macro_use]
extern crate fomat_macros;
#[macro_use]
extern crate markup;
extern crate maud;

#[macro_use]
extern crate serde_derive;

pub mod fmt;
pub mod fomat;
pub mod handlebars;
pub mod horrorshow_bench;
pub mod markup_bench;
pub mod maud_bench;
pub mod std_write;
