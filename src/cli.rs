use std::env;

pub enum Command {
    Log,
    Search,
}

pub struct ParsedArguments {
    pub command: Command,
    pub parameters: Vec<String>,
}

pub fn parse_args() -> ParsedArguments {
    let args: Vec<String> = env::args().collect();
    let command: Command;
    let mut parameters: Vec<String> = Vec::new();
    if args.len() == 1 {
        command = Command::Search;
    } else {
        match args[1].as_str() {
            "log" => {
                command = Command::Log;
                parameters.extend_from_slice(&args[2..]);
            }
            _ => {
                command = Command::Search;
                parameters.extend_from_slice(&args[1..]);
            }
        }
    }

    ParsedArguments { command, parameters }
}
