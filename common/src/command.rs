use std::collections::HashMap;

pub struct ParsedCommand {
    pub command: fn(Vec<String>),
    pub parameters: Vec<String>,
}

/// Parses command line arguments into a `ParsedCommand` struct.
///
/// # Arguments
///
/// * `args` - A vector of strings representing the command line arguments. If the first element is
/// a key in the `commands` map, it is treated as the command string. Otherwise, all elements will
/// be treated as parameters for the detfault command.
/// * `commands` - A map from command strings to the corresponding command functions. The map should
/// contain a default command with the key `""`.
///
/// # Returns
///
/// * `Ok(ParsedCommand)` - If the command is found in the `commands` map, or if a default command
/// is available. The `ParsedCommand` struct contains the command function and its parameters.
/// * `Err(String)` - If the command is not found in the `commands` map and there is no default
/// command. The error string is "No matching or default command found".
///
/// # Examples
///
/// ```
/// let mut commands = HashMap::new();
/// commands.insert("".to_string(), default_command as fn(Vec<String>));
/// commands.insert("test".to_string(), test_command as fn(Vec<String>));
///
/// let args = vec!["test".to_string(), "arg1".to_string(), "arg2".to_string()];
/// let parsed_command = parse_command(args, commands).unwrap();
/// ```
pub fn parse_command(
    args: Vec<String>,
    commands: HashMap<String, fn(Vec<String>)>,
) -> Result<ParsedCommand, String> {
    let mut command_str: String;
    if args.len() == 0 {
        command_str = String::from("");
    } else {
        command_str = args[0].clone();
    }

    let parameters: Vec<String>;
    if commands.contains_key(&command_str) {
        parameters = if args.len() > 1 {
            args[1..].to_vec()
        } else {
            Vec::new()
        }
    } else {
        command_str = String::from("");
        parameters = args.clone();
    }

    match commands.get(&command_str) {
        Some(command) => Ok(ParsedCommand {
            command: *command,
            parameters,
        }),
        None => Err("No matching or default command found".to_string()),
    }
}

/// Parses command line arguments and runs the corresponding command.
///
/// # Arguments
///
/// * `args` - A vector of strings representing the command line arguments. If the first element is
/// a key in the `commands` map, it is treated as the command string. Otherwise, all elements will
/// be treated as parameters for the detfault command.
/// * `commands` - A map from command strings to the corresponding command functions. The map should
/// contain a default command with the key `""`.
///
/// # Examples
///
/// ```
/// let mut commands = HashMap::new();
/// commands.insert("".to_string(), default_command as fn(Vec<String>));
/// commands.insert("test".to_string(), test_command as fn(Vec<String>));
///
/// run_command(vec!["arg1".to_string(), "arg2".to_string()], commands);
/// // This will call default_command with the parameters ["arg1", "arg2"]
///
/// run_command(vec!["test".to_string(), "arg1".to_string(), "arg2".to_string()], commands);
/// // This will call test_command with the parameters ["arg1", "arg2"]
/// ```
pub fn run_command(args: Vec<String>, commands: HashMap<String, fn(Vec<String>)>) {
    let parsed_args: ParsedCommand = parse_command(args, commands).unwrap();
    let cmd = parsed_args.command;
    cmd(parsed_args.parameters);
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use super::*;

    #[test]
    fn test_parse_command() {
        let default_command: fn(Vec<String>) = |_args| {};
        let test_command: fn(Vec<String>) = |_args| {};

        let mut commands = HashMap::new();
        commands.insert("".to_string(), default_command);
        commands.insert("test".to_string(), test_command);
        let args = vec!["test".to_string(), "arg1".to_string(), "arg2".to_string()];
        let parsed_command = parse_command(args, commands).unwrap();
        assert_eq!(parsed_command.command, test_command);
        assert_eq!(
            parsed_command.parameters,
            vec!["arg1".to_string(), "arg2".to_string()]
        );
    }

    #[test]
    fn test_parse_command_default() {
        let default_command: fn(Vec<String>) = |_args| {};
        let test_command: fn(Vec<String>) = |_args| {};

        let mut commands = HashMap::new();
        commands.insert("".to_string(), default_command);
        commands.insert("test".to_string(), test_command);
        let args = vec!["arg1".to_string(), "arg2".to_string()];
        let parsed_command = parse_command(args, commands).unwrap();
        assert_eq!(parsed_command.command, default_command);
        assert_eq!(
            parsed_command.parameters,
            vec!["arg1".to_string(), "arg2".to_string()]
        );
    }

    #[test]
    #[should_panic]
    fn test_parse_command_no_default() {
        let test_command: fn(Vec<String>) = |_args| {};

        let mut commands = HashMap::new();
        commands.insert("test".to_string(), test_command);
        let args = vec!["arg1".to_string(), "arg2".to_string()];
        parse_command(args, commands).unwrap();
    }

    // TODO: The run_command tests could be improved with mocks to ensure that the correct commands
    // are being called
    #[test]
    fn test_run_command() {
        let default_command: fn(Vec<String>) = |_args| {
            panic!("This should not be called");
        };
        let test_command: fn(Vec<String>) = |_args| {
            assert_eq!(vec!["arg1".to_string(), "arg2".to_string()], _args);
        };

        let mut commands = HashMap::new();
        commands.insert("".to_string(), default_command);
        commands.insert("test".to_string(), test_command);
        let args = vec!["test".to_string(), "arg1".to_string(), "arg2".to_string()];
        run_command(args, commands);
    }

    #[test]
    fn test_run_command_no_args() {
        let default_command: fn(Vec<String>) = |_args| {
            panic!("This should not be called");
        };
        let test_command: fn(Vec<String>) = |_args| {
            let empty_args: Vec<String> = Vec::new();
            assert_eq!(empty_args, _args);
        };

        let mut commands = HashMap::new();
        commands.insert("".to_string(), default_command);
        commands.insert("test".to_string(), test_command);
        let args = vec!["test".to_string()];
        run_command(args, commands);
    }

    #[test]
    fn test_run_command_default() {
        let default_command: fn(Vec<String>) = |_args| {
            assert_eq!(vec!["arg1".to_string(), "arg2".to_string()], _args);
        };
        let test_command: fn(Vec<String>) = |_args| {
            panic!("This should not be called");
        };

        let mut commands = HashMap::new();
        commands.insert("".to_string(), default_command);
        commands.insert("test".to_string(), test_command);
        let args = vec!["arg1".to_string(), "arg2".to_string()];
        run_command(args, commands);
    }

    #[test]
    fn test_run_command_default_no_args() {
        let default_command: fn(Vec<String>) = |_args| {
            let empty_args: Vec<String> = Vec::new();
            assert_eq!(empty_args, _args);
        };
        let test_command: fn(Vec<String>) = |_args| {
            panic!("This should not be called");
        };

        let mut commands = HashMap::new();
        commands.insert("".to_string(), default_command);
        commands.insert("test".to_string(), test_command);
        let args: Vec<String> = Vec::new();
        run_command(args, commands);
    }


    #[test]
    #[should_panic]
    fn test_run_command_no_default() {
        let test_command: fn(Vec<String>) = |_args| {};

        let mut commands = HashMap::new();
        commands.insert("test".to_string(), test_command);
        let args = vec!["arg1".to_string(), "arg2".to_string()];
        run_command(args, commands);
    }
}
