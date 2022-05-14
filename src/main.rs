use comfy_table::presets::UTF8_FULL;
use comfy_table::Table;
use reqwest;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <url>", args[0]);
        return;
    }

    let res = reqwest::get(&args[1]).await.unwrap().text().await;
    let doc = Html::parse_document(&res.unwrap().to_string());

    let sel = Selector::parse("a").unwrap();
    let mut links: Vec<String> = doc
        .select(&sel)
        .filter_map(|a| a.value().attr("href"))
        .map(|href| String::from(href))
        .collect();

    links.dedup();

    let mut table = Table::new();

    table.load_preset(UTF8_FULL).set_header(vec!["URL"]);

    links.iter().for_each(|link| {
        table.add_row(vec![link.to_string()]);
    });

    println!("{table}")
}
