#![feature(test)]
#[macro_use]
extern crate askama;
extern crate test;

use askama::Template;

#[derive(Template)]
#[template(path = "big-table.html")]
struct BigTable {
    table: Vec<Vec<usize>>,
}

#[bench]
fn big_table(b: &mut test::Bencher) {
    let size = 500;
    let mut table = Vec::with_capacity(size);
    for _ in 0..size {
        let mut inner = Vec::with_capacity(size);
        for i in 0..size {
            inner.push(i);
        }
        table.push(inner);
    }
    let ctx = BigTable { table };
    b.iter(|| ctx.render().unwrap());
}

#[derive(Template)]
#[template(path = "teams.html")]
struct Teams {
    year: u16,
    teams: Vec<Team>,
}

struct Team {
    name: String,
    score: u8,
}

#[bench]
fn teams(b: &mut test::Bencher) {
    let teams = Teams {
        year: 2015,
        teams: vec![
            Team { name: "Jiangsu".into(), score: 43 },
            Team { name: "Beijing".into(), score: 27 },
            Team { name: "Guangzhou".into(), score: 22 },
            Team { name: "Shandong".into(), score: 12 },
        ],
    };
    b.iter(|| teams.render().unwrap());
}
