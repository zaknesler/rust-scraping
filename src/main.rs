use comfy_table::presets::UTF8_FULL;
use comfy_table::Table;
use scraper::{Html, Selector};

struct Entry {
    name: String,
    url: String,
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <url>", args[0]);
        return;
    }

    let res = reqwest::get(&args[1]).await.unwrap().text().await;
    let doc = Html::parse_document(&res.unwrap());

    let sel = Selector::parse("a").unwrap();
    let entries: Vec<Entry> = doc
        .select(&sel)
        .filter(|a| a.value().attr("href").is_some())
        .map(|a| {
            let name = a.text().collect::<Vec<_>>().join(" ");
            let url = a.value().attr("href").unwrap().to_string();
            Entry { name, url }
        })
        .collect();

    let mut table = Table::new();
    table.load_preset(UTF8_FULL).set_header(vec!["Name", "URL"]);
    entries.iter().for_each(|entry| {
        table.add_row(vec![&entry.name, &entry.url]);
    });

    println!("{table}")
}
