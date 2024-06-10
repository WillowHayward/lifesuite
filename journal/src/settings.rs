use std::fmt;
use std::env;
use std::path::PathBuf;

pub enum EnvVar {
    HoraceDir,
    HoraceRc,
}


// TODO: Rework this whole thing when you're more familiar with Rust
// const HOME_ERROR: &str = "$HOME could not be converted to a path.";

impl EnvVar {
    fn default(&self) -> String {
        let home_dir = env::var("HOME").expect("HOME environment variable not set");
        match *self {
            EnvVar::HoraceDir => {
                // let msg = format!("{} Set $HOME or $HORACE_DIR to a valid path.", &HOME_ERROR);
                PathBuf::from(&home_dir).join(".horace").to_string_lossy().into_owned()
            },
            EnvVar::HoraceRc => {
                // let msg = format!("{} Set $HOME to a valid path, or $HORACE_DIR to a valid file location.", HOME_ERROR);
                PathBuf::from(&home_dir).join(".horacerc").to_string_lossy().into_owned()
            },
        }
    }
    pub fn get(&self) -> String {
        return std::env::var(self.to_string()).unwrap_or_else(|_| self.default().to_string())
    }
}

impl fmt::Display for EnvVar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnvVar::HoraceDir => write!(f, "HORACE_DIR"),
            EnvVar::HoraceRc => write!(f, "HORACE_CONFIG")
        }
    }
}
