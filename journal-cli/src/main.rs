use std::env;

use lifesuite_common::cli;
use lifesuite_journal_lib::commands;

fn main() {
    let cli_args = env::args().collect();
    let parsed_args: cli::ParsedArguments = cli::parse_args(cli_args, commands::get_commands());
    let cmd = parsed_args.command;
    cmd(parsed_args.parameters);
}
