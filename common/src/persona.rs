use uuid::Uuid;

use crate::component::{ComponentMeta, ComponentType};

#[derive(Serialize, Deserialize)]
pub struct Persona {
    pub meta: ComponentMeta,
    pub name: String,
}

impl Persona {
    pub fn new(name: String) -> Persona {
        Persona { 
            meta: ComponentMeta::new(ComponentType::Persona),
            name,
        }
    }

    pub fn get_by_name(_name: &str) -> Option<Persona> {
        todo!();
    }

    pub fn get_by_id(_id: Uuid) -> Option<Persona> {
        todo!();
    }

    pub fn get_current() -> Persona {
        todo!();
    }
}
