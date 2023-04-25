use criterion::Bencher;
use rusty_html::html;

pub fn big_table(b: &mut Bencher<'_>, size: &usize) {
    let mut table = Vec::with_capacity(*size);
    for _ in 0..*size {
        let mut inner = Vec::with_capacity(*size);
        for i in 0..*size {
            inner.push(i);
        }
        table.push(inner);
    }

    b.iter(|| {
        html!(
            <table>{
                for row in &table {
                    <tr>{
                        for col in row {
                            <td>{col}</td>
                        }
                    }</tr>
                }
            }</table>
        )
    });
}

pub fn teams(b: &mut Bencher<'_>, _: &usize) {
    let year = 2015;
    let teams = vec![
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
    ];

    b.iter(|| {
        html!(
            <html>
              <head>
                <title>{ year }</title>
              </head>
              <body>
                <h1>CSL { year }</h1>
                <ul>{
                    for (index, team) in teams.iter().enumerate() {
                      <li class={if index == 0 { "champion" } else { "" }}>
                        <b>{ &team.name }</b>: { &team.score }
                      </li>
                    }
                }
                </ul>
              </body>
            </html>
        )
    });
}

struct Team {
    name: String,
    score: u8,
}
