use criterion::{self, black_box};
use nate::Nate;

pub fn big_table(b: &mut criterion::Bencher<'_>, size: &usize) {
    let mut table = Vec::with_capacity(*size);
    for _ in 0..*size {
        let mut inner = Vec::with_capacity(*size);
        for i in 0..*size {
            inner.push(i);
        }
        table.push(inner);
    }
    let ctx = BigTable { table };
    b.iter(|| {
        let ctx = black_box(&ctx);
        format!("{}", ctx)
    });
}

#[derive(Nate)]
#[template(path = "templates_nate/big-table.html")]
struct BigTable {
    table: Vec<Vec<usize>>,
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
    b.iter(|| {
        let teams = black_box(&teams);
        format!("{}", teams)
    });
}

#[derive(Nate)]
#[template(path = "templates_nate/teams.html")]
struct Teams {
    year: u16,
    teams: Vec<Team>,
}

struct Team {
    name: String,
    score: u8,
}
