#![feature(test)]
#[macro_use]
extern crate horrorshow;
extern crate test;

use horrorshow::prelude::*;
use horrorshow::Error;

fn big_table_render(table: &Vec<Vec<usize>>) -> Result<String, Error> {
    let page = html! {
        table {
            @ for row in table {
                tr {
                    @ for col in row {
                        td { : col }
                    }
                }
            }
        }
    };
    page.into_string()
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
    b.iter(|| big_table_render(&table).unwrap());
}

struct Teams {
    year: u16,
    teams: Vec<Team>,
}

struct Team {
    name: String,
    score: u8,
}

fn teams_render(teams: &Teams) -> Result<String, Error> {
    let page = html! {
        html {
            head {
                title { : teams.year }
            }
            body {
                h1 { : Raw("CSL ");
                     : teams.year
                }
                ul {
                    @ for (idx, team) in teams.teams.iter().enumerate() {
                        li(class? = if idx == 0 { Some(Raw("champion")) } else { None }) {
                            b { : &team.name }
                            : Raw(": ");
                            : team.score
                        }
                    }
                }
            }
        }
    };
    page.into_string()
}

#[bench]
fn teams(b: &mut test::Bencher) {
    let teams = Teams {
        year: 2015,
        teams: vec![
            Team {
                name: "Jiangsu".into(),
                score: 43,
            },
            Team {
                name: "Beijing".into(),
                score: 27,
            },
            Team {
                name: "Guangzhou".into(),
                score: 22,
            },
            Team {
                name: "Shandong".into(),
                score: 12,
            },
        ],
    };

    b.iter(|| teams_render(&teams).unwrap());
}
