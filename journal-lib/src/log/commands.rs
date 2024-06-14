use std::collections::HashMap;

// Default command
fn search(args: Vec<String>) {
    super::log::display_logs(&args);
}

fn add(args: Vec<String>) {
    let log = super::log::build_log(&args);

    super::log::write_log(&log);
    println!("Wrote log {} to file", log.id);
}

fn edit(_args: Vec<String>) {
    todo!();
}

fn modify(_args: Vec<String>) {
    todo!();
}

pub fn run_command(args: Vec<String>) {
    println!("Running log command with args: {:?}", args);
       
    let mut commands: HashMap<String, fn(Vec<String>)> = HashMap::new();
    commands.insert(String::from(""), search);
    commands.insert(String::from("add"), add);
    commands.insert(String::from("edit"), edit);
    commands.insert(String::from("modify"), modify);
    lifesuite_common::command::run_command(args, commands);
}
