use std::collections::HashMap;

// Root level commands - invoked by `log <args>` or `life log <args>`
pub fn get_commands() -> HashMap<String, fn(Vec<String>)> {
    let mut commands: HashMap<String, fn(Vec<String>)> = HashMap::new();
    commands.insert("".to_string(), crate::log::commands::run_command); // Redirect all default input to log command set

    commands.insert(
        "journal".to_string(),
        crate::journal::commands::run_command,
    );

    commands.insert("tag".to_string(), crate::tag::commands::run_command);
    commands.insert(
        "entity".to_string(),
        crate::tag::entity::commands::run_command,
    );
    commands.insert(
        "context".to_string(),
        crate::tag::context::commands::run_command,
    );

    commands.insert(
        "template".to_string(),
        crate::template::commands::run_command,
    );

    commands.insert("event".to_string(), crate::event::commands::run_command);

    commands.insert("report".to_string(), crate::report::commands::run_command);

    commands.insert("sync".to_string(), crate::sync::commands::run_command);

    commands
}
