use lifesuite_common::component::{ComponentMeta, ComponentType};
use uuid::Uuid;

pub struct Almanac {
    pub meta: ComponentMeta,
    pub person: Uuid,
    pub name: String,
}
impl Almanac {
    pub fn new(name: String, person: Uuid) -> Almanac {
        Almanac {
            meta: ComponentMeta::new(ComponentType::Almanac),
            person,
            name,
        }
    }
    pub fn get_by_name(_name: &str, _person: &Uuid) -> Option<Almanac> {
        todo!();
    }
    pub fn get_by_id(_id: &Uuid) -> Option<Almanac> {
        todo!();
    }
    pub fn get_current(_person: &Uuid) -> Almanac {
        todo!();
    }
}
