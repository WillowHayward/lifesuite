use std::collections::HashMap;

pub struct ParsedOpts {
    pub opts: HashMap<String, String>, // <OptName, OptValue>
    pub args: Vec<String>,             // Original args passed to parser
    pub stripped: Vec<String>,         // Original args with opts removed
}

pub fn parse_opts(args: Vec<String>, available_opts: Vec<String>) -> ParsedOpts {
    // NOTE: Rust should handle quoted arguments correctly
    // TODO: Confirm above w/ end-to-end tests
    let mut opts: HashMap<String, String> = HashMap::new();
    let mut stripped: Vec<String> = Vec::new();

    for arg in &args {
        let mut strip = false;
        for opt in &available_opts {
            if arg.starts_with(opt) {
                let parts: Vec<&str> = arg.split(":").collect();
                if parts.len() == 2 {
                    opts.insert(opt.to_string(), parts[1].to_string());
                    strip = true;
                }
            }
        }
        if !strip {
            stripped.push(arg.to_string());
        }
    }

    ParsedOpts {
        opts,
        args,
        stripped,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_opts() {
        let available_opts = vec!["journal".to_string(), "persona".to_string()];
        let args = vec![
            "other:000".to_string(),
            "journal:123".to_string(),
            "persona:456".to_string(),
        ];
        let parsed_opts = parse_opts(args, available_opts);
        assert_eq!(parsed_opts.opts.len(), 2);
        assert_eq!(parsed_opts.opts.get("journal").unwrap(), "123");
        assert_eq!(parsed_opts.opts.get("persona").unwrap(), "456");
        assert_eq!(parsed_opts.args.len(), 3);
        assert_eq!(parsed_opts.stripped.len(), 1);
        assert_eq!(parsed_opts.stripped[0], "other:000");
    }
}
