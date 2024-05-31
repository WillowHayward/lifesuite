use crate::tag::{Tag, parse_tag};
use crate::person::{Person, parse_person};
use crate::place::{Place, parse_location};

pub struct Log {
    pub text: String,
    pub tags: Vec<Tag>,
    pub people: Vec<Person>,
    pub places: Vec<Place>,
}

pub fn build_log(log: &[String]) -> Log {
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
        text: log.join(" "),
        tags,
        people,
        places,
    };
}
