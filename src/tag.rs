pub struct Tag {
    pub name: String,
    pub values: Vec<String>,
}

pub fn parse_tag(full_tag: &str) -> Tag {
    let parts: Vec<String> = full_tag.split(":").map(|s| s.to_string()).collect();
    let tag = &parts[0];
    let values = &parts[1..];
    return Tag {
        name: tag.to_string(),
        values: values.to_vec(),
    };
}

