use chrono::{DateTime, Local};
use lifesuite_common::component::{ComponentMeta, ComponentType};
use lifesuite_common::tag::{Tag, TagType};
use uuid::Uuid;

use crate::journal::Journal;
use crate::tag::JournalTag;

#[derive(Serialize, Deserialize)]
pub struct Log {
    pub meta: ComponentMeta,
    pub date: DateTime<Local>,
    pub entry: String,
    pub tags: Vec<JournalTag>,
    pub journal: Uuid, // id of journal that owns this log
}

impl Log {
    pub fn new(entry: Vec<String>, journal: &Journal) -> Log {
        let mut tags: Vec<JournalTag> = Vec::new();

        for part in &entry {
            if Tag::has_signifier(part) {
                let tag = JournalTag::new(part, journal);
                tags.push(tag);
            }
        }

        Log {
            meta: ComponentMeta::new(ComponentType::Log),
            date: Local::now(),
            entry: entry.join(" "),
            tags,
            journal: journal.meta.id,
        }
    }

    pub fn get_by_id(_id: &str) -> Option<Log> {
        todo!();
    }

    pub fn save(&self) {
        todo!();
    }

    pub fn display(&self) {
        println!("Log {}", self.meta.id);
        println!("    Entry: {}", self.entry);
        println!("    Date: {}", self.date.format("%Y-%m-%d %H:%M:%S"));
        let mut tags: Vec<&JournalTag> = Vec::new();
        let mut entities: Vec<&JournalTag> = Vec::new();
        let mut contexts: Vec<&JournalTag> = Vec::new();
        for tag in &self.tags {
            match tag.tag.tag_type {
                TagType::Tag => tags.push(tag),
                TagType::Entity => entities.push(tag),
                TagType::Context => contexts.push(tag),
            }
        }

        for tag in tags {
            tag.display();
        }

        for entity in entities {
            entity.display();
        }

        for context in contexts {
            context.display();
        }
    }
}
