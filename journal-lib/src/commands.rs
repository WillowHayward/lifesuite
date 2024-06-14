use std::collections::HashMap;

use crate::log;

fn add(args: Vec<String>) {
    let log = log::build_log(&args);

    log::write_log(&log);
    println!("Wrote log {} to file", log.id);
}

fn search(args: Vec<String>) {
    log::display_logs(&args);
}

pub fn get_commands() -> HashMap<String, fn(Vec<String>)> {
    let mut commands: HashMap<String, fn(Vec<String>)> = HashMap::new();
    commands.insert(String::from(""), search); // Default
    commands.insert(String::from("add"), add);
    commands
}
