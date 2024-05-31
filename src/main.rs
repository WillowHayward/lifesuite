use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        let search = Vec::new();
        display_logs(&search);
        return;
    }

    match args[1].as_str() {
        "log" => build_log(&args[2..]),
        _ => display_logs(&args[1..]),
    }
}

fn display_logs(search: &[String]) {
    println!("Displaying logs relating to '{}'", search.join(" "));
}

struct Log {
    text: String,
    tags: Vec<Tag>,
    people: Vec<Person>,
    places: Vec<Place>,
}

fn build_log(log: &[String]) {
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

    let log = Log {
        text: log.join(" "),
        tags,
        people,
        places,
    };

    println!("Log: {}", log.text);
    for tag in log.tags {
        println!("    Tag: {}", tag.name);
        for value in tag.values {
            println!("        Value: {}", value);
        }
    }
    for person in log.people {
        println!("    tPerson: {}", person.name);
    }
    for place in log.places {
        println!("    tPlace: {}", place.name);
    }
}

struct Tag {
    name: String,
    values: Vec<String>,
}

fn parse_tag(full_tag: &str) -> Tag {
    let parts: Vec<String> = full_tag.split(":").map(|s| s.to_string()).collect();
    let tag = &parts[0];
    let values = &parts[1..];
    return Tag {
        name: tag.to_string(),
        values: values.to_vec(),
    };
}

struct Person {
    name: String,
}

fn parse_person(person: &str) -> Person {
    return Person {
        name: person.to_string(),
    };
}

struct Place {
    name: String,
}

fn parse_location(location: &str) -> Place {
    return Place {
        name: location.to_string(),
    };
}
