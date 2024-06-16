use lifesuite_common::persona::Persona;
use std::collections::HashMap;

use crate::{journal::Journal, log::search::search_logs};

use super::log::Log;

// Default command
fn search(args: Vec<String>) {
    println!("Searching for logs with terms: {:?}", args);
    let logs = search_logs(&args);
    for log in logs {
        log.display();
    }
}

fn add(args: Vec<String>) {
    let available_opts = vec!["journal".to_string(), "persona".to_string()];
    let mut parsed_opts = lifesuite_common::opts::parse_opts(args, available_opts);

    let persona = if parsed_opts.opts.get("persona").is_none() {
        let persona = Persona::get_current();
        parsed_opts
            .opts
            .insert("persona".to_string(), persona.meta.id.to_string());
        persona
    } else {
        let persona_name = parsed_opts.opts.get("persona").unwrap();
        let persona = match Persona::get_by_name(persona_name) {
            Some(persona) => persona,
            None => Persona::new(persona_name.to_string()),
        };

        persona
    };

    let journal = if parsed_opts.opts.get("journal").is_none() {
        let journal = Journal::get_current(&persona);
        parsed_opts
            .opts
            .insert("journal".to_string(), journal.meta.id.to_string());
        journal
    } else {
        let journal_name = parsed_opts.opts.get("journal").unwrap();
        let journal = match Journal::get_by_name(journal_name, &persona) {
            Some(journal) => journal,
            None => Journal::new(journal_name.to_string(), &persona),
        };

        journal
    };

    parsed_opts
        .opts
        .insert("foo".to_string(), "bar".to_string());
    let log = Log::new(parsed_opts.stripped, &journal);
    log.save();
}

fn edit(_args: Vec<String>) {
    todo!();
}

fn modify(_args: Vec<String>) {
    todo!();
}

fn list(_args: Vec<String>) {
    todo!();
}

pub fn run_command(args: Vec<String>) {
    println!("Running log command with args: {:?}", args);

    let mut commands: HashMap<String, fn(Vec<String>)> = HashMap::new();
    commands.insert(String::from(""), search);
    commands.insert(String::from("add"), add);
    commands.insert(String::from("edit"), edit);
    commands.insert(String::from("modify"), modify);
    commands.insert(String::from("list"), list);
    lifesuite_common::commands::run_command(args, commands);
}
