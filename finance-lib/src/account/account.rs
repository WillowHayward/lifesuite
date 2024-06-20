use lifesuite_common::{component::{ComponentMeta, ComponentType}, tag::Tag};
use uuid::Uuid;

pub struct Account {
    pub meta: ComponentMeta,
    pub name: String,
    pub tag: Option<Uuid>, // Uuid of linked tag, if this account is linked to a tag (usually by name)
}

// TODO: Should accounts be limited to a ledger?
impl Account {
    pub fn new(name: &str, persona: &Uuid) -> Account {
        let tag: Option<Uuid> = if Tag::has_signifier(name) {
            match Tag::get_by_full_name(name, persona) {
                Some(tag) => Some(tag.meta.id),
                None => Some(Tag::new(name, persona).meta.id),
            }
        } else {
            None
        };
        Account {
            meta: ComponentMeta::new(ComponentType::Account),
            name: name.to_string(),
            tag,
        }
    }
    pub fn get_by_name(_name: &str) -> Option<Account> {
        todo!();
    }
    pub fn get_by_id(_id: &Uuid) -> Option<Account> {
        todo!();
    }
    pub fn get_current() -> Account {
        todo!();
    }

    // TODO: This should return an array of the calculated balance for each account by currency
    pub fn get_balance(&self) -> f64 {
        todo!();
    }
}
