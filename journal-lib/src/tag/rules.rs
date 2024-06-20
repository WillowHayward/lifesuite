use std::collections::HashMap;

// TODO: Rework this whole thing. Rules are too complicated right now.

use super::values::{JournalTagValue, ValueType};

pub enum AllowedValues {
    Number(AllowedNumberValues),
    String(AllowedStringValues),
    Date(AllowedDateValues), // TODO: Replace string w/ chrono::Date (or best match)
    List(Vec<AllowedListAndMapItemValues>),
    Map(HashMap<String, AllowedListAndMapItemValues>),
}

pub struct AllowedNumberValues {
    pub min: Option<i32>,
    pub max: Option<i32>,
    pub values: Option<Vec<i32>>,
}

pub struct AllowedStringValues {
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub values: Option<Vec<String>>,
}

pub struct AllowedDateValues {
    pub min: Option<String>, // TODO: Replace string w/ chrono::Date (or best match)
    pub max: Option<String>, // TODO: Replace string w/ chrono::Date (or best match)
    pub values: Option<Vec<String>>, // TODO: Replace string w/ chrono::Date (or best match)
}

pub struct AllowedListValues {
    min_length: Option<usize>,
    max_length: Option<usize>,
}

pub struct AllowedListItemValues {
    pub index: usize,
    pub default: bool,
    pub prompt: bool,
    pub value: AllowedListAndMapItemValues,
}

pub enum AllowedListAndMapItemValues {
    Number(i32),
    String(String),
    Boolean(bool),
    Date(String),
    Scalar(String), // TODO: enforce or reconsider
}

pub struct JournalTagRules {
    pub name: String,
    pub value_type: Vec<ValueType>,
    pub required: bool,
    pub default: Option<JournalTagValue>,
    pub allowed_values: Option<Vec<JournalTagValue>>,
    pub linked_tags: Option<Vec<String>>,
}
