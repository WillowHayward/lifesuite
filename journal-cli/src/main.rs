use std::env;

use lifesuite_common::command::run_command;
use lifesuite_journal_lib::commands;

fn main() {
    let cli_args = env::args().collect();
    run_command(cli_args, commands::get_commands());
}
