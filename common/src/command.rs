use std::collections::HashMap;

pub struct ParsedArguments {
    pub command: fn(Vec<String>),
    pub parameters: Vec<String>,
}

pub fn parse_command(
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

pub fn run_command(
    args: Vec<String>,
    commands: HashMap<String, fn(Vec<String>)>, // NOTE: Map must have "" for default command
) {
    let parsed_args: ParsedArguments = parse_command(args, commands);
    let cmd = parsed_args.command;
    cmd(parsed_args.parameters);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_command(_args: Vec<String>) {}

    #[test]
    fn test_parse_command() {
        let mut commands = HashMap::new();
        commands.insert("".to_string(), test_command as fn(Vec<String>));
        commands.insert("test".to_string(), test_command as fn(Vec<String>));

        let args = vec!["program".to_string(), "test".to_string(), "arg1".to_string(), "arg2".to_string()];
        let parsed_args = parse_command(args.clone(), commands.clone());
        assert_eq!(parsed_args.parameters, args[2..]);

        let args = vec!["program".to_string()];
        let parsed_args = parse_command(args.clone(), commands.clone());
        assert_eq!(parsed_args.parameters, args);
    }

    #[test]
    #[should_panic(expected = "Command map must contain \"\" key for default command")]
    fn test_parse_command_no_default() {
        let commands = HashMap::new();
        let args = vec!["program".to_string()];
        let _parsed_args = parse_command(args.clone(), commands.clone());
    }

    #[test]
    fn test_run_command() {
        let mut commands = HashMap::new();
        commands.insert("".to_string(), test_command as fn(Vec<String>));
        commands.insert("test".to_string(), test_command as fn(Vec<String>));

        let args = vec!["program".to_string(), "test".to_string(), "arg1".to_string(), "arg2".to_string()];
        run_command(args.clone(), commands.clone());

        let args = vec!["program".to_string()];
        run_command(args.clone(), commands.clone());
    }

    #[test]
    #[should_panic(expected = "Command map must contain \"\" key for default command")]
    fn test_run_command_no_default() {
        let commands = HashMap::new();
        let args = vec!["program".to_string()];
        run_command(args.clone(), commands.clone());
    }
}
