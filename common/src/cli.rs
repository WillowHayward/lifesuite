use std::collections::HashMap;

pub struct ParsedArguments {
    pub command: fn(Vec<String>),
    pub parameters: Vec<String>,
}

pub fn parse_args(
    args: Vec<String>,
    commands: HashMap<String, fn(Vec<String>)>, // NOTE: Map must have "" for default command
) -> ParsedArguments {
    let command: String;
    if args.len() == 0 {
        command = String::from("");
    } else {
        command = args[1].clone();
    }
    if commands.contains_key(&command) {
        ParsedArguments {
            command: commands.get(&command).unwrap().clone(),
            parameters: args[2..].to_vec(),
        }
    } else {
        panic!("No default command");
            
    }
}
