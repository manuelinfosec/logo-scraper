use reqwest::blocking::{Client, Response};
use reqwest::Error;
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
    let client: Client = Client::new();
    let response: Response = client.get(website)
    .header(reqwest::header::USER_AGENT, 
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36")
        .send()?;
    response.text()
}
