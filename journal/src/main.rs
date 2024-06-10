#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod cli;
mod io;
mod log;
mod person;
mod place;
mod search;
mod settings;
mod tag;

mod traits {
    pub mod named;
}

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

