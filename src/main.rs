use scraper::Html;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://ksmt88.hatenablog.com/")?.text()?;

    let doc = Html::parse_document(&resp);

    let selector = scraper::Selector::parse("li.recent-entries-item > div > a").unwrap();

    // title
    doc.select(&selector).for_each(|e| println!("{:#?}", e.text().next().unwrap()));

    // link
    doc.select(&selector).for_each(|e| println!("{:#?}", e.value().attr("href").unwrap()));

    Ok(())
}
