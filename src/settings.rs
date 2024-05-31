use std::fmt;

pub enum EnvVar {
    HoraceDir,
    HoraceRc,
}

impl EnvVar {
    fn default(&self) -> &'static str {
        match *self {
            EnvVar::HoraceDir => "$HOME/.horace",
            EnvVar::HoraceRc => "$HOME/.horacerc"
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
