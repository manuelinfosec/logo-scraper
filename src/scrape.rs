use regex::Regex;
use scraper::{ElementRef, Html, Selector};

pub fn parse_logo(source: String) -> &'static str {
    let mut logo_url: Option<&str> = None;

    // regex pattern for matching substring "logo"
    let pattern: Regex = Regex::new(r".*logo.*").unwrap();

    // parse page source for querying
    let document: Html = Html::parse_document(source.as_str());

    // 1. Check for the first class that has the substring "logo"
    let class_selector: Selector = Selector::parse("[class]").unwrap();

    // assign element `Some(src)` attribute
    logo_url = match document
        // query document with selector
        .select(&class_selector)
        // iterate all elements till the first `true` condition
        // returns an Option if condition is met or None
        .find(|element: &ElementRef<'_>| -> bool {
            // collect the class attribute of each element
            let class_attribute: &str = element.value().attr("class").unwrap_or("");
            // check if it matches the regex pattern
            pattern.is_match(class_attribute)
        }) {
        // match if an element is found
        Some(tag) => tag.value().attr("src"),
        // or return `None` if nothing is found
        _ => None,
    };

    // 2. Check the first value that has the string "logo" in src.
    if logo_url.is_none() {
        // CSS Selector for img elements with a src attribute
        let value_selector: Selector = Selector::parse("img[src]").unwrap();
        
        logo_url = match document
            .select(&value_selector)
            .find(|element: &ElementRef<'_>| -> bool {
                let class_attribute: &str = element.value().attr("src").unwrap_or("");
                pattern.is_match(class_attribute)
            }) {
            Some(tag) => tag.value().attr("src"),
            _ => None,
        };
    }

    println!("{logo_url:?}");

    // `logo_url` will eventually become populated
    logo_url.unwrap()
}
