#[macro_use]
extern crate askama;
#[macro_use]
extern crate horrorshow;
extern crate criterion;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate fomat_macros;

pub mod askama_bench;
pub mod handlebars;
pub mod horrorshow_bench;
pub mod liquid;
pub mod tera;
pub mod write_bench;
pub mod fomat_bench;