fn default(args: Vec<String>) {
    log::display_logs(&args);
}

fn add(args: Vec<String>) {
    let log = log::build_log(&args);

    log::write_log(&log);
    println!("Wrote log {} to file", log.id);
}

fn edit(args: Vec<String>) {
    todo!();
}

fn modify(args: Vec<String>) {
    todo!();
}

pub fn run_command(args: Vec<String>) {
    todo!();
}
