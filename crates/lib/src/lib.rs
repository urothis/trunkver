extern crate chrono;

use chrono::Utc;
use std::fmt;

pub mod prelude {
    pub use crate::trunkver;
    pub use crate::TrunkVer;
}

#[macro_export]
macro_rules! trunkver {
    () => {
        ::trunkver_lib::TrunkVer::new(None, false)
            .to_string()
            .as_str();
    };
    ($build_ref:expr) => {
        ::trunkver_lib::TrunkVer::new(Some($build_ref.to_string()), false)
            .to_string()
            .as_str();
    };
    ($build_ref:expr, $with_v:expr) => {
        ::trunkver_lib::TrunkVer::new(Some($build_ref.to_string()), $with_v)
            .to_string()
            .as_str();
    };
}

pub struct TrunkVer {
    pub build_ref: String,
    pub time: String,
    pub source_ref: String,
}

impl TrunkVer {
    pub fn new(build_ref: Option<String>, with_v: bool) -> Self {
        let now_str = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let prefix = if with_v { "v" } else { "" };
        let time = format!("{}{}.0.0", prefix, now_str);

        Self {
            build_ref: build_ref.unwrap_or("".to_string()),
            time,
            source_ref: TrunkVer::with_get_git_source_ref().into(),
        }
    }

    pub fn with_get_git_source_ref() -> String {
        // execute `git rev-parse --verify HEAD` and return the output as a string
        let command = std::process::Command::new("git")
            .arg("rev-parse")
            .arg("--verify")
            .arg("HEAD")
            .output()
            .expect("failed to execute process");

        let source_ref = String::from_utf8_lossy(&command.stdout).to_string();
        // trim the ref down to 7 characters
        source_ref[..7].to_string()
    }
}

impl fmt::Display for TrunkVer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.build_ref.is_empty() {
            true => write!(f, "{}-{}", self.time, self.source_ref),
            false => write!(f, "{}-{}-{}", self.time, self.source_ref, self.build_ref),
        }
    }
}
