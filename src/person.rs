pub struct Person {
    pub name: String,
}

pub fn parse_person(person: &str) -> Person {
    return Person {
        name: person.to_string(),
    };
}

