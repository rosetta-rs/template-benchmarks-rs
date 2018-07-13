
use criterion;

pub fn big_table(b: &mut criterion::Bencher, size: &usize) {
    let mut table = Vec::with_capacity(*size);
    for _ in 0..*size {
        let mut inner = Vec::with_capacity(*size);
        for i in 0..*size {
            inner.push(i);
        }
        table.push(inner);
    }
    let mut table_str = String::new();
    for r1 in table {
        table_str.push_str("<tr>\n");
        for r2 in r1 {
            table_str.push_str(format!("<td>{col}</td>",col = r2).as_str());
        }
        table_str.push_str("</tr>\n");
    }
    let output = format!(
"<table>
{table_str}
</table>", table_str = table_str);

    b.iter(|| output.to_owned());
}

struct BigTable {
    table: Vec<Vec<usize>>,
}

pub fn teams(b: &mut criterion::Bencher, _: &usize) {
    let teams = Teams {
        year: 2015,
        teams: vec![
            Team { name: "Jiangsu".into(), score: 43 },
            Team { name: "Beijing".into(), score: 27 },
            Team { name: "Guangzhou".into(), score: 22 },
            Team { name: "Shandong".into(), score: 12 },
        ],
    };
    let mut team_str = String::new();
    for (i,team) in (&teams).teams.iter().enumerate() {
        let champion = if i != 0 {
            ""
        } else {
            "champion"
        };
        team_str.push_str(format!("<li class=\"{champion}\">
            <b>{name}</b>: {score}", champion = champion, name = team.name, score = team.score).as_str());
    }
    let output = format!(
"<html>
  <head>
    <title>{year}</title>
  </head>
  <body>
    <h1>CSL {year}</h1>
    <ul>
    {team_str}
    </ul>
  </body>
</html>", year = teams.year, team_str = team_str);
    b.iter(|| output.to_owned());
}

struct Teams {
    year: u16,
    teams: Vec<Team>,
}

struct Team {
    name: String,
    score: u8,
}
