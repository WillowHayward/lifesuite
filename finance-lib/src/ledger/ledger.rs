use lifesuite_common::component::{ComponentMeta, ComponentType};
use uuid::Uuid;

pub struct Ledger {
    pub meta: ComponentMeta,
    pub name: String,
}

impl Ledger {
    pub fn new(name: &str) -> Ledger {
        Ledger {
            meta: ComponentMeta::new(ComponentType::Ledger),
            name: name.to_string(),
        }
    }
    pub fn get_by_name(_name: &str) -> Option<Ledger> {
        todo!();
    }
    pub fn get_by_id(_id: &Uuid) -> Option<Ledger> {
        todo!();
    }
    pub fn get_current() -> Ledger {
        todo!();
    }
}
