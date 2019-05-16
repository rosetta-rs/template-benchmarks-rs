use maud::html;

pub fn big_table(b: &mut criterion::Bencher, size: &usize) {
    let mut table = Vec::with_capacity(*size);
    for _ in 0..*size {
        let mut inner = Vec::with_capacity(*size);
        for i in 0..*size {
            inner.push(i);
        }
        table.push(inner);
    }
    b.iter(|| {
        html! {
            table {
                @for row in &table {
                    tr {
                        @for col in row {
                            td { (col) }
                        }
                    }
                }
            }
        }
    });
}

struct Entry {
    name: String,
    score: u8,
}

pub fn teams(b: &mut criterion::Bencher) {
    let year = 2015u16;
    let teams = vec![
        Entry {
            name: "Jiangsu".into(),
            score: 43,
        },
        Entry {
            name: "Beijing".into(),
            score: 27,
        },
        Entry {
            name: "Guangzhou".into(),
            score: 22,
        },
        Entry {
            name: "Shandong".into(),
            score: 12,
        },
    ];
    b.iter(|| {
        html! {
            html {
                head {
                    title { (year) }
                }
                body {
                    h1 { "CSL " (year) }
                    ul {
                        @for (i, team) in teams.iter().enumerate() {
                            li.champion[i == 0] {
                                b { (team.name) ": " (team.score) }
                            }
                        }
                    }
                }
            }
        }
    });
}
