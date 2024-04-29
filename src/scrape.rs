use regex::Regex;
use scraper::{Html, Selector};

pub fn parse_logo(source: String) -> Result<String, regex::Error> {
    let mut logo_url: Option<&str> = None;

    // create the regex pattern to search for
    let pattern: Regex = Regex::new(r".*logo.*").unwrap();

    // parse page source for querying
    let document: Html = Html::parse_document(source.as_str());

    // 1. Check for the first class that has the substring "logo"
    let class_selector: Selector = Selector::parse("[class]").unwrap();

    // 1. Check for the first class that has the substring "logo"
    logo_url = match document.select(&class_selector).find(|element| {
        let class_attribute: &str = element.value().attr("class").unwrap_or("");
        pattern.is_match(class_attribute)
    }) {
        Some(tag) => tag.value().attr("class"),
        _ => None,
    };

    if logo_url.is_none() {
        let value_selector: Selector = Selector::parse("img[src]").unwrap();

        logo_url = match document.select(&value_selector).find(|element| {
            let class_attribute: &str = element.value().attr("src").unwrap_or("");
            pattern.is_match(class_attribute)
        }) {
            Some(tag) => tag.value().attr("src"),
            _ => None,
        };
    }

    println!("{logo_url:?}");

    Ok("Logo".to_string())
}
