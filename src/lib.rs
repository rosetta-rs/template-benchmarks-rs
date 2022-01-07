pub mod askama_bench;
pub mod fomat;
pub mod handlebars;
pub mod horrorshow_bench;
pub mod liquid;
pub mod markup_bench;
pub mod maud_bench;
pub mod ructe;
pub mod sailfish;
pub mod std_write;
pub mod tera;

include!(concat!(env!("OUT_DIR"), "/templates.rs"));
