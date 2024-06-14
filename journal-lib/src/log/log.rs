use std::fs;

use chrono::{DateTime, Local};
use lifesuite_common::settings::EnvVar;
use uuid::Uuid;

use crate::index::add_to_index;
use crate::log::search::search_logs;
use crate::tag::{Tag, parse_tag};
use crate::tag::entity::{Entity, parse_entity};
use crate::tag::context::{Context, parse_context};

#[derive(Serialize, Deserialize)]
pub struct Log {
    pub id: Uuid,
    pub date: DateTime<Local>,
    pub text: String,
    pub tags: Vec<Tag>,
    pub entities: Vec<Entity>,
    pub contexts: Vec<Context>,
}

pub fn build_log(log: &[String]) -> Log {
    let id = Uuid::new_v4();
    let date = Local::now();
    let text = log.join(" ");
    let mut tags: Vec<Tag> = Vec::new();
    let mut entities: Vec<Entity> = Vec::new();
    let mut contexts: Vec<Context> = Vec::new();
    for part in log {
        if part.starts_with("+") {
            let tag = parse_tag(&part[1..]);
            tags.push(tag);
        } else if part.starts_with("@") {
            let entity = parse_entity(&part[1..]);
            entities.push(entity);
        } else if part.starts_with("%") {
            let context = parse_context(&part[1..]);
            contexts.push(context);
        }
    }

    return Log {
        id,
        date,
        text,
        tags,
        entities,
        contexts,
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
        for entity in &log.entities {
            println!("    Entity: {}", entity.name);
        }
        for context in &log.contexts {
            println!("    Context: {}", context.name);
        }
    }
}
