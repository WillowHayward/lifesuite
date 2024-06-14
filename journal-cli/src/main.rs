use std::env;

use lifesuite_common::cli::run_command_from_args;
use lifesuite_journal_lib::commands;

fn main() {
    let cli_args = env::args().collect();
    run_command_from_args(cli_args, commands::get_commands());
}
