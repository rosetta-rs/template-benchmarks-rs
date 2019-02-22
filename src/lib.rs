#![cfg_attr(feature = "nightly", feature(proc_macro_hygiene))]

#[macro_use]
extern crate askama;
#[macro_use]
extern crate horrorshow;
extern crate criterion;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate fomat_macros;
#[macro_use]
extern crate markup;

#[cfg(feature = "nightly")]
extern crate maud;
extern crate yarte;

pub mod askama_bench;
pub mod fomat;
pub mod handlebars;
pub mod horrorshow_bench;
pub mod liquid;
pub mod markup_bench;
pub mod ructe;
pub mod std_write;
pub mod tera;
pub mod yarte_bench;

#[cfg(feature = "nightly")]
pub mod maud_bench;

include!(concat!(env!("OUT_DIR"), "/templates.rs"));
