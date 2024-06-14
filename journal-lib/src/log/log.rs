use std::fs;

use chrono::{DateTime, Local};
use lifesuite_common::settings::EnvVar;
use uuid::Uuid;

use crate::index::add_to_index;
use crate::log::search::search_logs;
use crate::tag::{Tag, parse_tag};
use crate::tag::entity::{Person, parse_person};
use crate::tag::context::{Place, parse_location};

#[derive(Serialize, Deserialize)]
pub struct Log {
    pub id: Uuid,
    pub date: DateTime<Local>,
    pub text: String,
    pub tags: Vec<Tag>,
    pub people: Vec<Person>,
    pub places: Vec<Place>,
}

pub fn build_log(log: &[String]) -> Log {
    let id = Uuid::new_v4();
    let date = Local::now();
    let text = log.join(" ");
    let mut tags: Vec<Tag> = Vec::new();
    let mut people: Vec<Person> = Vec::new();
    let mut places: Vec<Place> = Vec::new();
    for part in log {
        if part.starts_with("+") {
            let tag = parse_tag(&part[1..]);
            tags.push(tag);
        } else if part.starts_with("@") {
            let person = parse_person(&part[1..]);
            people.push(person);
        } else if part.starts_with("%") {
            let place = parse_location(&part[1..]);
            places.push(place);
        }
    }

    return Log {
        id,
        date,
        text,
        tags,
        people,
        places,
    };
}

pub fn write_log(log: &Log) {
    let json = serde_json::to_string(&log).unwrap_or("".to_string());

    if json == "" {
        println!("Failed to convert log to JSON");
        return;
    }

    let path = format!("{}/{}.json", EnvVar::HoraceDir.get(), log.id.to_string());
    fs::write(path, &json).expect("Unable to write file");
    add_to_index(log);
}

pub fn read_log(id: &str) -> Log {
    let path = format!("{}/{}.json", EnvVar::HoraceDir.get(), id);
    let json = fs::read_to_string(path).expect("Unable to read file");
    let log: Log = serde_json::from_str(&json).expect("Unable to parse JSON");
    return log;
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
