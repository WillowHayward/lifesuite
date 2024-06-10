use lifesuite_common::cli;
use lifesuite_journal_lib::{io, log, search};
fn main() {
    let parsed_args: cli::ParsedArguments = cli::parse_args();

    match parsed_args.command {
        cli::Command::Log => {
            let log = log::build_log(&parsed_args.parameters);

            io::write_log(&log);
            println!("Wrote log {} to file", log.id);
        }
        cli::Command::Search => {
            search::display_logs(&parsed_args.parameters);
        }
    }
}
