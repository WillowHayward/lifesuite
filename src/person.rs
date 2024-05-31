use crate::traits::named::Named;

#[derive(Serialize, Deserialize)]
pub struct Person {
    pub name: String,
}

impl Named for Person {
    fn name(&self) -> &str {
        &self.name
    }
}

pub fn parse_person(person: &str) -> Person {
    return Person {
        name: person.to_string(),
    };
}

