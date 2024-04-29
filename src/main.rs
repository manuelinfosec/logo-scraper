mod scrape;
mod utils;

fn main() {
    // collec websites from STDIN
    let websites: Vec<String> = utils::collect_websites();
    println!("Starting scraping...\n");

    for domain in websites {
        // add a schema to the domain
        let website: String = utils::prepend_http(domain);

        // fetch page HTML page source
        let page_source: String = match utils::fetch_page_source(website) {
            Ok(text) => text,
            Err(_) => {
                // TODO: Query Public Logo API
                println!("An error occurred while fetching page source.");

                // skip to next iteration
                continue;
            }
        };
        
        // scrape::parse_logo(page_source);

        println!("{page_source}");
    }
}
