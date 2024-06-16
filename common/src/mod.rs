use crate::component::ComponentMeta;

#[derive(Serialize, Deserialize)]
pub enum ChangeType {
    Add,
    Remove,
    Edit,
    Purge, // Complete removal
}

#[derive(Serialize, Deserialize)]
pub struct Change<T> {
    pub target: String, // The field being targeted
    pub r#type: ChangeType,
    pub old: Option<T>, // old value of target. remove and modify only - otherwise None
    pub new: Option<T>, // old value of target. add and modify only - otherwise None
}

impl<T> Change<T> {
    pub fn new(target: String, r#type: ChangeType, old: Option<T>, new: Option<T>) -> Change<T> {
        Change {
            target,
            r#type,
            old,
            new,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Mod<T> {
    pub meta: ComponentMeta,
    pub changes: Vec<Change<T>>
}

impl<T> Mod<T> {
    pub fn new(changes: Vec<Change<T>>) -> Mod<T> {
        Mod {
            meta: ComponentMeta::new(),
            changes,
        }
    }
}
