use std::collections::HashMap;
use std::fs;

use crate::log::Log;
use crate::settings::EnvVar;
use crate::traits::named::Named;

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

struct Index {
    pub tags: HashMap<String, Vec<String>>,
    pub people: HashMap<String, Vec<String>>,
    pub places: HashMap<String, Vec<String>>,
}

pub enum NamedIndex {
    Tags,
    People,
    Places,
}

impl NamedIndex {
    pub fn to_string(&self) -> &'static str {
        match *self {
            NamedIndex::Tags => "tags",
            NamedIndex::People => "people",
            NamedIndex::Places => "places",
        }
    }
}

fn add_to_index(log: &Log) {
    let tags: Vec<String> = log.tags.iter().map(|n| n.name().to_string()).collect();
    add_to_named_index(NamedIndex::Tags.to_string(), &log.id.to_string(), tags);
    let people: Vec<String> = log.people.iter().map(|n| n.name().to_string()).collect();
    add_to_named_index(NamedIndex::People.to_string(), &log.id.to_string(), people);
    let places: Vec<String> = log.places.iter().map(|n| n.name().to_string()).collect();
    add_to_named_index(NamedIndex::Places.to_string(), &log.id.to_string(), places);
}

fn add_to_named_index(index_type: &str, id: &str, names: Vec<String>) {
    if names.is_empty() {
        return;
    }
    let mut index = get_named_index(index_type);
    for name in names {
        let mut values: Vec<String>;
        match index.get(&name) {
            Some(v) => values = v.clone(),
            None => values = Vec::new(),
        }
        values.push(id.to_string());
        index.insert(name, values);
    }
    write_named_index(index_type, index);
}

pub fn get_full_index() -> Index {
    return Index {
        tags: get_named_index(NamedIndex::Tags.to_string()),
        people: get_named_index(NamedIndex::People.to_string()),
        places: get_named_index(NamedIndex::Places.to_string()),
    };
}

fn write_full_index(index: Index) {
    write_named_index(NamedIndex::Tags.to_string(), index.tags);
    write_named_index(NamedIndex::People.to_string(), index.people);
    write_named_index(NamedIndex::Places.to_string(), index.places);
}

pub fn get_named_index(named_index: &str) -> HashMap<String, Vec<String>> {
    let path = format!("{}/{}.json", EnvVar::HoraceDir.get(), named_index);
    if !fs::metadata(&path).is_ok() {
        return HashMap::new();
    }
    let json = fs::read_to_string(path).expect("Unable to read file");
    return serde_json::from_str(&json).expect("Unable to parse JSON");
}

fn write_named_index(named_index: &str, index: HashMap<String, Vec<String>>) {
    let path = format!("{}/{}.json", EnvVar::HoraceDir.get(), named_index);
    let json = serde_json::to_string(&index).unwrap_or("".to_string());
    fs::write(path, &json).expect("Unable to write file");
}
