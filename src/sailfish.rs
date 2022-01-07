use criterion::black_box;
use sailfish::TemplateOnce;

pub fn big_table(b: &mut criterion::Bencher<'_>, size: &usize) {
    let mut table = Vec::with_capacity(*size);
    for _ in 0..*size {
        let mut inner = Vec::with_capacity(*size);
        for i in 0..*size {
            inner.push(i);
        }
        table.push(inner);
    }
    b.iter(|| {
        let table = black_box(&table);
        let ctx = BigTable { table };
        ctx.render_once().unwrap()
    });
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
        let (year, teams) = black_box((teams.year, &teams.teams));
        let teams = TeamsTemplate { year, teams };
        teams.render_once().unwrap()
    });
}

#[derive(TemplateOnce)]
#[template(path = "big-table.stpl")]
struct BigTable<'a> {
    table: &'a [Vec<usize>],
}

#[derive(TemplateOnce)]
#[template(path = "teams.stpl")]
struct TeamsTemplate<'a> {
    year: u16,
    teams: &'a [Team],
}

struct Teams {
    year: u16,
    teams: Vec<Team>,
}

struct Team {
    name: String,
    score: u8,
}
