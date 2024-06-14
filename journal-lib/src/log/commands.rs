fn default(args: Vec<String>) {
    log::display_logs(&args);
}

fn add(args: Vec<String>) {
    let log = log::build_log(&args);

    log::write_log(&log);
    println!("Wrote log {} to file", log.id);
}

fn edit(_args: Vec<String>) {
    todo!();
}

fn modify(_args: Vec<String>) {
    todo!();
}

pub fn run_command(args: Vec<String>) {
    todo!();
}
