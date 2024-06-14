use std::{collections::{HashMap, HashSet}, fs};

use lifesuite_common::settings::EnvVar;

use crate::{log::Log, traits::named::Named};


pub struct Index {
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

pub fn add_to_index(log: &Log) {
    let tags: Vec<String> = log.tags.iter().map(|n| n.name().to_string()).collect();
    add_to_named_index(NamedIndex::Tags.to_string(), &log.id.to_string(), tags);
    let people: Vec<String> = log.entities.iter().map(|n| n.name().to_string()).collect();
    add_to_named_index(NamedIndex::People.to_string(), &log.id.to_string(), people);
    let places: Vec<String> = log.contexts.iter().map(|n| n.name().to_string()).collect();
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

pub fn search_index(index: HashMap<String, Vec<String>>, term: &str) -> HashSet<String> {
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
