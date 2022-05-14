extern crate reqwest;
extern crate scraper;

// importation syntax
use scraper::{Html, Selector};

fn main() {
    let html = r#"
        <ul>
            <li><a href='/url/foo'>Foo</a></li>
            <li>Bar</li>
            <li><a href="/url/baz"><span>Baz</span></a></li>
        </ul>
    "#;

    let fragment = Html::parse_fragment(html);

    let sel = Selector::parse("a").unwrap();
    let links: Vec<String> = fragment
        .select(&sel)
        .filter_map(|a| a.value().attr("href"))
        .map(|href| String::from(href))
        .collect();

    println!("{:?}", links);
}
