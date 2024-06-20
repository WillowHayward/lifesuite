use std::collections::HashMap;

pub fn get_commands() > HashMap<String, fn(Vec<String>)> {
    let mut commands = HashMap::new();
    commands.insert("".to_string(), crate::list::commands::run_command); // Redirect all default
    // input to list command set

    commands
}
