use std::io::{BufRead, StdinLock};

fn collect_websites() -> Vec<String> {
    let mut websites: Vec<String> = vec![];
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

fn main() {
    // collec websites from STDIN
    let websites: Vec<String> = collect_websites();
    println!("{websites:?}");
}
