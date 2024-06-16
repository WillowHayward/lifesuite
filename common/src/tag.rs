use uuid::Uuid;

use crate::{component::ComponentMeta, persona::Persona};

#[derive(Serialize, Deserialize)]
pub enum TagType {
    Tag,
    Entity,
    Context,
}

// A tag, including entities and contexts, without an attached value
#[derive(Serialize, Deserialize)]
pub struct Tag {
    pub meta: ComponentMeta,
    pub tag_type: TagType,
    pub name: String, // The name of the tag, excluding signifier (+/@/%), but including any
    // parent tags (e.g. "tag.subtag.subsubtag")
    pub parent: Uuid, // Either the parent tag (the one to the left by ".") or the persona id
}

impl Tag {
    pub fn new(full_name: &str, persona: &Uuid) -> Tag {
        let parent = if full_name.contains(".") {
            let parts: Vec<&str> = full_name.split(".").collect();
            let parent_name = parts[0..parts.len() - 1].join(".");
            match Tag::get_by_full_name(&parent_name, &persona) {
                Some(tag) => tag.meta.id,
                None => Tag::new(&parent_name, &persona).meta.id,
            }
        } else {
            // TODO: Check dereferencing here
            *persona
        };
        Tag {
            meta: ComponentMeta::new(),
            tag_type: Tag::get_tag_type(full_name),
            name: full_name[1..].to_string(), // Signifier not stored with name
            parent,
        }
    }

    // Find by name, including signifier
    pub fn get_by_full_name(_full_name: &str, persona: &Uuid) -> Option<Tag> {
        let tag_type = Tag::get_tag_type(_full_name);
        let name = &_full_name[1..];
        Self::get_by_name(name, tag_type, persona)
    }

    pub fn get_by_name(_name: &str, _type: TagType, _persona: &Uuid) -> Option<Tag> {
        todo!();
    }

    pub fn get_by_id(_id: Uuid, _persona: &Uuid) -> Option<Tag> {
        todo!();
    }

    // Return true if candidate starts with a valid tag signifier
    pub fn has_signifier(candidate: &str) -> bool {
        match &candidate[0..1] {
            "+" => true,
            "@" => true,
            "%" => true,
            _ => false,
        }
    }

    pub fn get_tag_type(full_name: &str) -> TagType {
        match &full_name[0..1] {
            "+" => TagType::Tag,
            "@" => TagType::Entity,
            "%" => TagType::Context,
            _ => panic!("Invalid tag signifier"),
        }
    }

    // NOTE: Assumes is a valid tag w/ signifier
    pub fn get_tag_name(full_name: &str) -> String {
        full_name[1..].to_string()
    }
}
