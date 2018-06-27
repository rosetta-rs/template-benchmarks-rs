extern crate tera;
extern crate serde;

use criterion;

use self::tera::{Context, Tera};

// Tera doesn't allow `escape` on number values
static BIG_TABLE_TEMPLATE: &'static str = "<table>
{% for row in table %}
<tr>{% for col in row %}<td>{{ col }}</td>{% endfor %}</tr>
{% endfor %}
</table>";

pub fn big_table(b: &mut criterion::Bencher, size: &usize) {
    let mut table = Vec::with_capacity(*size);
    for _ in 0..*size {
        let mut inner = Vec::with_capacity(*size);
        for i in 0..*size {
            inner.push(i);
        }
        table.push(inner);
    }

    let mut tera = Tera::default();
    tera.add_raw_templates(vec![("big-table.html", BIG_TABLE_TEMPLATE)]).unwrap();
    let mut ctx = Context::new();
    ctx.add("table", &table);

    let _ = tera.render("big-table.html", &ctx).unwrap();
    b.iter(|| tera.render("big-table.html", &ctx));
}

#[derive(Serialize)]
struct Team {
    name: String,
    score: u8,
}

static TEAMS_TEMPLATE: &'static str = "<html>
  <head>
    <title>{{ year }}</title>
  </head>
  <body>
    <h1>CSL {{ year }}</h1>
    <ul>
    {% for team in teams %}
      <li class=\"{% if loop.first %}champion{% endif %}\">
      <b>{{team.name}}</b>: {{team.score}}
      </li>
    {% endfor %}
    </ul>
  </body>
</html>";

pub fn teams(b: &mut criterion::Bencher, _: &usize) {
    let mut tera = Tera::default();
    tera.add_raw_templates(vec![("teams.html", TEAMS_TEMPLATE)]).unwrap();
    let mut ctx = Context::new();
    ctx.add("year", &2015);
    ctx.add("teams", &vec![
        Team { name: "Jiangsu".into(), score: 43 },
        Team { name: "Beijing".into(), score: 27 },
        Team { name: "Guangzhou".into(), score: 22 },
        Team { name: "Shandong".into(), score: 12 },
    ]);

    let _ = tera.render("teams.html", &ctx).unwrap();
    b.iter(|| tera.render("teams.html", &ctx));
}
