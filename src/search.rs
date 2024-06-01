use std::collections::{HashMap, HashSet};

use uuid::Uuid;

use crate::io::{get_named_index, read_log, NamedIndex};
use crate::log::Log;

pub fn search_logs(search: &[String]) -> Vec<Log> {
    let ids = search_log_ids(search);
    let mut logs: Vec<Log> = Vec::new();
    for id in ids {
        let log = read_log(id.as_str());
        logs.push(log);
    }

    return logs;
}

pub fn search_log_ids(search: &[String]) -> HashSet<String> {
    let mut ids: HashSet<String> = HashSet::new();
    // TODO: If these could be lazy loaded that'd be neat
    let tags = get_named_index(NamedIndex::Tags.to_string());
    let people = get_named_index(NamedIndex::People.to_string());
    let places = get_named_index(NamedIndex::Places.to_string());
    for term in search {
        println!("Checking term '{:?}'", term);
        let term_str = term.as_str();
        match Uuid::parse_str(term_str) {
            Ok(id) => {
                ids.insert(id.to_string());
            }
            Err(_) => {}
        }

        println!("1{:?}", term_str.chars().nth(0));
        match term_str.chars().nth(0) {
            Some('+') => {
                let found_ids = search_index(tags.clone(), term_str);
                for id in found_ids {
                    ids.insert(id.clone());
                }
            }
            Some('@') => {
                let found_ids = search_index(people.clone(), term_str);
                for id in found_ids {
                    ids.insert(id.clone());
                }
            }
            Some('%') => {
                let found_ids = search_index(places.clone(), term_str);
                println!("2{:?}", found_ids);
                for id in found_ids {
                    ids.insert(id.clone());
                }
            }
            _ => {}
        }
        /*for index in vec![NamedIndex::Tags, NamedIndex::People, NamedIndex::Places] {
            let index = get_named_index(index.to_string());
            match index.get(term) {
                Some(found_ids) => {
                    for id in found_ids {
                        ids.insert(id.clone());
                    }
                }
                None => continue,
            }
        }*/
    }

    return ids;
}

fn search_index(index: HashMap<String, Vec<String>>, term: &str) -> HashSet<String> {
    let mut ids: HashSet<String> = HashSet::new();

    let trimmed_term: String = term.chars().skip(1).collect();
    let found_ids = index.get(&trimmed_term);
    if found_ids.is_some() {
        for id in found_ids.unwrap() {
            ids.insert(id.clone());
        }
    }
    return ids;
}
pub fn display_logs(search: &[String]) {
    println!("Searching for logs with terms: {:?}", search);
    let logs = search_logs(search);
    for log in logs {
        println!("Log {}", log.id);
        println!("    Entry: {}", log.text);
        println!("    Date: {}", log.date.format("%Y-%m-%d %H:%M:%S"));
        for tag in &log.tags {
            println!("    Tag: {}", tag.name);
            for value in &tag.values {
                println!("        Value: {}", value);
            }
        }
        for person in &log.people {
            println!("    Person: {}", person.name);
        }
        for place in &log.places {
            println!("    Place: {}", place.name);
        }
    }
}
