use std::fs;

use crate::log::Log;
use crate::settings::EnvVar;

pub fn write_log(log: &Log) {
    let json = serde_json::to_string(&log).unwrap_or("".to_string());

    if json == "" {
        println!("Failed to convert log to JSON");
        return;
    }

    let path = format!("{}/{}.json", EnvVar::HoraceDir.get(), log.id.to_string());
    fs::write(path, &json).expect("Unable to write file");
}

pub fn read_log(id: &str) -> Log {
    let path = format!("{}/{}.json", EnvVar::HoraceDir.get(), id);
    let json = fs::read_to_string(path).expect("Unable to read file");
    let log: Log = serde_json::from_str(&json).expect("Unable to parse JSON");
    return log;
}
