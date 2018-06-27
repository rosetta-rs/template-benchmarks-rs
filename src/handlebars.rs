extern crate handlebars;
extern crate serde_json;

use self::handlebars::{Handlebars, to_json};

use self::serde_json::value::Value as Json;

use std::collections::BTreeMap;

use test;

static TEAMS_TEMPLATE: &'static str = "<html>
  <head>
    <title>{{year}}</title>
  </head>
  <body>
    <h1>CSL {{year}}</h1>
    <ul>
    {{#each teams}}
      <li class=\"{{#if @first}}champion{{/if}}\">
      <b>{{name}}</b>: {{score}}
      </li>
    {{/each}}
    </ul>
  </body>
</html>";

fn teams_data() -> BTreeMap<String, Json> {
    let mut data = BTreeMap::new();

    data.insert("year".to_string(), to_json(&"2015".to_owned()));

    let mut teams = Vec::new();

    for v in vec![
        ("Jiangsu", 43u16),
        ("Beijing", 27u16),
        ("Guangzhou", 22u16),
        ("Shandong", 12u16),
    ].iter()
    {
        let (name, score) = *v;
        let mut t = BTreeMap::new();
        t.insert("name".to_string(), to_json(&name));
        t.insert("score".to_string(), to_json(&score));
        teams.push(t)
    }

    data.insert("teams".to_string(), to_json(&teams));
    data
}

#[bench]
fn teams(b: &mut test::Bencher) {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string("table", TEAMS_TEMPLATE)
        .ok()
        .expect("Invalid template format");

    let data = teams_data();
    b.iter(|| handlebars.render("table", &data).ok().unwrap())
}

/*
This throws an error that I cannot quite figure out:

Err(RenderError {
  desc: "invalid digit found in string",
  template_name: Some("big-table.html"),
  line_no: Some(1),
  column_no: Some(7),
  cause: Some(ParseIntError { kind: InvalidDigit }),
})

static SOURCE: &'static str = "<html>
    {{#each table as |n|}}
        <tr>{{#each n as |v|}}<td>{{v}}</td>{{/each}}</tr>
    {{/each}}
</html>";

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

    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("big-table.html", SOURCE).unwrap();
    handlebars.render("big-table.html", &table).unwrap();

    //b.iter(|| handlebars.render("big-table.html", &table).ok().unwrap());
}

*/
