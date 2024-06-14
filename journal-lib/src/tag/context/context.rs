use crate::traits::named::Named;

#[derive(Serialize, Deserialize)]
pub struct Context {
    pub name: String,
}

impl Named for Context {
    fn name(&self) -> &str {
        &self.name
    }
}


pub fn parse_context(context: &str) -> Context {
    return Context {
        name: context.to_string(),
    };
}
