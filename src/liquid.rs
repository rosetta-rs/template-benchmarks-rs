use ::liquid::{model::Value, Object, ParserBuilder};
use criterion::{self, black_box};
use serde_yaml;

pub fn big_table(b: &mut criterion::Bencher<'_>, size: &usize) {
    let mut table = Vec::with_capacity(*size);
    for _ in 0..*size {
        let mut inner = Vec::with_capacity(*size);
        for i in 0..*size {
            inner.push(Value::Scalar((i as i32).into()));
        }
        table.push(Value::Array(inner));
    }

    let template = ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(
            "<table>
{% for row in table %}
<tr>{% for col in row %}<td>{{ col|escape }}</td>{% endfor %}</tr>
{% endfor %}
</table>",
        )
        .unwrap();

    let mut globals = Object::new();
    globals.insert("table".into(), Value::Array(table));

    b.iter(|| {
        let globals = black_box(&globals);
        template.render(globals)
    });
}

pub fn teams(b: &mut criterion::Bencher<'_>, _: &usize) {
    let parser = ParserBuilder::with_stdlib().build().unwrap();
    let template = parser.parse(TEAMS_TEMPLATE).unwrap();

    let data: Object = self::serde_yaml::from_str(TEAMS_DATA).unwrap();

    b.iter(|| {
        let data = black_box(&data);
        template.render(data)
    });
}

static TEAMS_TEMPLATE: &'static str = "<html>
  <head>
    <title>{{year}}</title>
  </head>
  <body>
    <h1>CSL {{year}}</h1>
    <ul>
    {% for team in teams %}
      <li class=\"{% if forloop.first %}champion{% endif %}\">
      <b>{{team.name}}</b>: {{team.score}}
      </li>
    {% endfor %}
    </ul>
  </body>
</html>";

static TEAMS_DATA: &'static str = "
year: 2015
teams:
  - name: Jiangsu
    score: 43
  - name: Beijing
    score: 27
  - name: Guangzhou
    score: 22
  - name: Shandong
    score: 12
";
