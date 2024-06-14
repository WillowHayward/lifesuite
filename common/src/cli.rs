use std::collections::HashMap;

pub struct ParsedArguments {
    pub command: fn(Vec<String>),
    pub parameters: Vec<String>,
}

pub fn parse_args(
    args: Vec<String>,
    commands: HashMap<String, fn(Vec<String>)>, // NOTE: Map must have "" for default command
) -> ParsedArguments {
    let mut command_str: String;
    if args.len() == 0 {
        command_str = String::from("");
    } else {
        command_str = args[1].clone();
    }

    let parameters: Vec<String>;
    if commands.contains_key(&command_str) {
        parameters = args[2..].to_vec();
    } else {
        command_str = String::from("");
        parameters = args.clone();
    }

    match commands.get(&command_str) {
        Some(command) => ParsedArguments {
            command: *command,
            parameters,
        },
        None => panic!("Command map must contain \"\" key for default command"),
    }
}

pub fn run_command_from_args(
    args: Vec<String>,
    commands: HashMap<String, fn(Vec<String>)>, // NOTE: Map must have "" for default command
) {
    let parsed_args: ParsedArguments = parse_args(args, commands);
    let cmd = parsed_args.command;
    cmd(parsed_args.parameters);
}
