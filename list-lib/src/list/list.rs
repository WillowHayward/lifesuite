
use lifesuite_common::component::{ComponentMeta, ComponentType};
use uuid::Uuid;

use super::path::Path;

pub struct List {
    pub meta: ComponentMeta,
    pub path: Path,
    pub parent: Uuid, // Parent list - either the root list (workbook) or the immediate path parent
}

impl List {
    pub fn new(path: Path, parent: Uuid) -> List {
        List {
            meta: ComponentMeta::new(ComponentType::List),
            path,
            parent,
        }
    }
    pub fn get_by_path(_path: &Path, _parent: &Uuid) -> Option<List> {
        todo!();
    }
    pub fn get_by_id(_id: &Uuid) -> Option<List> {
        todo!();
    }
    pub fn get_current(_parent: &Uuid) -> List {
        todo!();
    }
}
