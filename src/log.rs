use chrono::{DateTime, Local};
use uuid::Uuid;

use crate::tag::{Tag, parse_tag};
use crate::person::{Person, parse_person};
use crate::place::{Place, parse_location};

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
