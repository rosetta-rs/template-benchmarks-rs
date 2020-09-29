// https://github.com/utkarshkukreti/markup.rs/blob/master/markup/benches/tbr.rs

use criterion;

markup::define! {
    BigTable(table: Vec<Vec<usize>>) {
        table {
            @for r1 in table {
                tr {
                    @for r2 in r1 {
                        td { @r2 }
                    }
                }
            }
        }
    }
}

pub fn big_table(b: &mut criterion::Bencher<'_>, size: &usize) {
    let mut table = Vec::with_capacity(*size);
    for _ in 0..*size {
        let mut inner = Vec::with_capacity(*size);
        for i in 0..*size {
            inner.push(i);
        }
        table.push(inner);
    }
    let table = BigTable { table };
    b.iter(|| table.to_string());
}

pub struct Team {
    name: String,
    score: u8,
}

markup::define! {
    Teams(year: u16, teams: Vec<Team>) {
        html {
            head {
                title { @year }
            }
            body {
                h1 { "CSL " @year }
                ul {
                    @for (index, team) in teams.iter().enumerate() {
                        li.{if index == 0 { Some("champion") } else { None } } {
                            b { @team.name } ": " @team.score
                        }
                    }
                }
            }
        }
    }
}

pub fn teams(b: &mut criterion::Bencher<'_>, _: &usize) {
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
    b.iter(|| teams.to_string());
}
