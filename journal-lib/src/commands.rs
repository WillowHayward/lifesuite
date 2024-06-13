use std::collections::HashMap;

use crate::io;
use crate::log;
use crate::search;

pub fn log(args: Vec<String>) {
    let log = log::build_log(&args);

    io::write_log(&log);
    println!("Wrote log {} to file", log.id);
}

pub fn search(args: Vec<String>) {
    search::display_logs(&args);
}

pub fn get_commands() -> HashMap<String, fn(Vec<String>)> {
    let mut commands: HashMap<String, fn(Vec<String>)> = HashMap::new();
    commands.insert(String::from(""), search); // Default
    commands.insert(String::from("add"), log);
    commands
}
