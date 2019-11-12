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

include!(concat!(env!("OUT_DIR"), "/templates.rs"));
