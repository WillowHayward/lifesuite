pub struct Place {
    pub name: String,
}

pub fn parse_location(location: &str) -> Place {
    return Place {
        name: location.to_string(),
    };
}
