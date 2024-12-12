use chrono::Utc;
use core::fmt;

pub mod prelude {
    pub use crate::TrunkVer;
}

pub struct TrunkVer {
    pub build_ref: String,
    pub time: String,
    pub source_ref: String,
}

impl TrunkVer {
    pub fn new(build_ref: &str, source_ref: &str, with_v: bool) -> Self {
        let now_str = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let prefix = if with_v { "v" } else { "" };
        let time = format!("{}{}.0.0", prefix, now_str);

        Self {
            build_ref: build_ref.into(),
            time,
            source_ref: source_ref.into(),
        }
    }
}

impl fmt::Display for TrunkVer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}-{}", self.time, self.source_ref, self.build_ref)
    }
}
