use lifesuite_common::tag::{Tag, TagType};

use crate::journal::Journal;

#[derive(Serialize, Deserialize)]
pub struct JournalTag {
    pub tag: Tag,
    pub value: Option<String>, // The string representation of the tag value, or None if tag is a
                               // SimpleTag. As this is stored as a string, it will be parsed on demand.
}

impl JournalTag {
    // Parse tag from string. If the underlying tag doesn't exist, one will be created.
    pub fn new(raw: &str, journal: &Journal) -> Self {
        let parts: Vec<&str> = raw.split(":").collect();
        let full_name: &str = parts[0];

        let tag: Tag = match Tag::get_by_full_name(full_name, &journal.persona) {
            Some(existing_tag) => existing_tag,
            None => Tag::new(full_name, &journal.persona)
        };

        let value = if parts.len() > 1 {
            Some(parts[1..].join(":"))
        } else {
            None
        };

        JournalTag { tag, value }
    }

    pub fn display(&self) {
        let tag_type = match self.tag.tag_type {
            TagType::Tag => "Tag",
            TagType::Entity => "Entity",
            TagType::Context => "Context",
        };
        println!("{}: {}", tag_type, self.tag.name);
        match &self.value {
            Some(v) => println!("Value: {}", v),
            None => println!("Value: None"),
        }
    }
}
