#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod log;
mod person;
mod place;
mod tag;

use log::build_log;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        let search = Vec::new();
        display_logs(&search);
        return;
    }

    match args[1].as_str() {
        "log" => {
            let log = build_log(&args[2..]);
            let json = serde_json::to_string(&log).unwrap_or("".to_string());

            if json == "" {
                println!("Failed to convert log to JSON");
                return;
            }
            println!("{}", json);

            println!("Log {}", log.id);
            println!("    Entry: {}", log.text);
            println!("    Date: {}", log.date.format("%Y-%m-%d %H:%M:%S"));
            for tag in log.tags {
                println!("    Tag: {}", tag.name);
                for value in tag.values {
                    println!("        Value: {}", value);
                }
            }
            for person in log.people {
                println!("    Person: {}", person.name);
            }
            for place in log.places {
                println!("    Place: {}", place.name);
            }

            let path = format!("/tmp/life/{}.json", log.id.to_string());
            fs::write(path, &json).expect("Unable to write file");
        }
        _ => display_logs(&args[1..]),
    }
}

fn display_logs(search: &[String]) {
    println!("Displaying logs relating to '{}'", search.join(" "))
}
