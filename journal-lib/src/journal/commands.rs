use std::collections::HashMap;

use super::{add, edit, list, remove, set};

pub fn run_command(_args: Vec<String>) {
    let mut commands: HashMap<String, fn(Vec<String>)> = HashMap::new();
    commands.insert(String::from("add"), add);
    commands.insert(String::from("edit"), edit);
    commands.insert(String::from("remove"), remove);
    commands.insert(String::from("list"), list);
    commands.insert(String::from(""), set);

    lifesuite_common::commands::run_command(_args, commands);
}
