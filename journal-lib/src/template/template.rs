use lifesuite_common::component::{ComponentMeta, ComponentType};

pub struct Template {
    pub meta: ComponentMeta,
    pub name: String,
    pub content: String,
}

impl Template {
    pub fn new(name: String, content: String) -> Template {
        Template {
            meta: ComponentMeta::new(ComponentType::Template),
            name,
            content,
        }
    }
}
