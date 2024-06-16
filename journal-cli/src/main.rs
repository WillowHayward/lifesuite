use std::env;

use lifesuite_common::commands::run_command;
use lifesuite_journal_lib::commands;

fn main() {
    let cli_args: Vec<String> = env::args().collect();
    let command_args = if cli_args.len() > 1 {
        cli_args[1..].to_vec() // Trim program name
    } else {
        Vec::new() // No arguments were passed
    };
    run_command(command_args, commands::get_commands());
}
