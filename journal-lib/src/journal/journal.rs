use lifesuite_common::{component::ComponentMeta, persona::Persona};
use uuid::Uuid;

pub fn add(_args: Vec<String>) {
    todo!();
}

pub fn edit(_args: Vec<String>) {
    todo!();
}

pub fn remove(_args: Vec<String>) {
    todo!();
}

pub fn list(_args: Vec<String>) {
    todo!();
}

pub fn set(_args: Vec<String>) {
    todo!();
}

pub struct Journal {
    pub meta: ComponentMeta,
    pub name: String,
    pub parent: Uuid,
}

impl Journal {
    pub fn new(name: String, persona: &Persona) -> Journal {
        Journal {
            meta: ComponentMeta::new(),
            name,
            parent: persona.meta.id,
        }
    }

    pub fn get_by_name(_name: &str, _persona: &Persona) -> Option<Journal> {
        todo!();
    }

    pub fn get_by_id(_id: &Uuid) -> Option<Journal> {
        todo!();
    }


    pub fn get_current(_persona: &Persona) -> Journal {
        todo!();
    }
}
