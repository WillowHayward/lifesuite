use crate::traits::named::Named;

#[derive(Serialize, Deserialize)]
pub struct Entity {
    pub name: String,
}

impl Named for Entity {
    fn name(&self) -> &str {
        &self.name
    }
}

pub fn parse_entity(entity: &str) -> Entity {
    return Entity {
        name: entity.to_string(),
    };
}

