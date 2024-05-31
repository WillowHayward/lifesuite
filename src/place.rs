use crate::traits::named::Named;

#[derive(Serialize, Deserialize)]
pub struct Place {
    pub name: String,
}

impl Named for Place {
    fn name(&self) -> &str {
        &self.name
    }
}


pub fn parse_location(location: &str) -> Place {
    return Place {
        name: location.to_string(),
    };
}
