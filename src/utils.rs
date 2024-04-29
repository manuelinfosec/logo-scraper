use reqwest::{self, Error};
use std::io::{BufRead, StdinLock};

pub fn collect_websites() -> Vec<String> {
    let mut websites: Vec<String> = vec![];

    // prompt
    println!("Enter a list of websites to scrape (seperated by newline): ");

    // lock STDIN for synchronous use
    let reader: StdinLock = std::io::stdin().lock();

    for line in reader.lines() {
        let line: String = line.unwrap();
        // check for an empty line
        if line.trim().is_empty() {
            break;
        }
        websites.push(line.trim().to_owned());
    }

    websites
}

pub fn prepend_http(website: String) -> String {
    if !website.starts_with("http") {
        return format!("http://{}", website);
    }
    website
}

pub fn fetch_page_source(website: String) -> Result<String, Error> {
    let response: reqwest::blocking::Response = reqwest::blocking::get(website)?;
    response.text()
}
