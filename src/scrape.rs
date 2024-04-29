use regex::Regex;
use scraper::{Html, Selector};

pub fn parse_logo(source: String) -> Result<String, regex::Error> {
    let logo_url: String = String::new();

    // create the regex pattern to search for
    let pattern: Regex = Regex::new(r".*logo.*").unwrap();

    // parse page source for querying
    let document: Html = Html::parse_document(source.as_str());

    // CSS selector for elements with class attributes
    let class_selector: Selector = Selector::parse("[class]").unwrap();
    let logo_url = document
        .select(&class_selector)
        .filter_map(|element| {
            let class_attribute: &str = element.value().attr("class").unwrap_or("");
            // println!("{class_attribute}");
            if pattern.is_match(class_attribute) {
                println!("{:?}", element.value());
                element.value().attr("src")
            } else {
                None
            }
        })
        .nth(0);

    println!("{logo_url:?}");

    Ok("Logo".to_string())
}
