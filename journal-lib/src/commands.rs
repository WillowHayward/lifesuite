use std::collections::HashMap;

// Root level commands - invoked by `log <args>` or `life log <args>`
pub fn get_commands() -> HashMap<String, fn(Vec<String>)> {
    let mut commands: HashMap<String, fn(Vec<String>)> = HashMap::new();
    commands.insert(String::from(""), crate::log::commands::run_command); // Redirect all default input to log command set

    commands.insert(
        String::from("journal"),
        crate::journal::commands::run_command,
    );

    commands.insert(String::from("tag"), crate::tag::commands::run_command);
    commands.insert(
        String::from("entity"),
        crate::tag::entity::commands::run_command,
    );
    commands.insert(
        String::from("context"),
        crate::tag::context::commands::run_command,
    );

    commands.insert(
        String::from("template"),
        crate::template::commands::run_command,
    );

    commands.insert(String::from("event"), crate::event::commands::run_command);

    commands.insert(String::from("report"), crate::report::commands::run_command);

    commands.insert(String::from("sync"), crate::sync::commands::run_command);

    commands
}
