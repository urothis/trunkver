use std::{fs, io::Write, path::PathBuf};

use chrono::Utc;
use quote::quote;
use syn::parse_file;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let when = Utc::now().format("%Y%m%d%H%M%S").to_string();
    let what = {
        let out = std::process::Command::new("git")
            .args(["rev-parse", "--verify", "HEAD"])
            .output()
            .expect("failed to run git");
        let s = String::from_utf8_lossy(&out.stdout).to_string();
        s[..7].to_string()
    };
    let how = "unknown";

    let trunkver_value = format!("v{}.0.0-{}-{}", when, what, how);
    let trunker_value = format!("{}.0.0-{}-{}", when, what, how);
    let trunk_why_doc = format!("Why this version was created: {}", when);
    let trunk_how_doc = format!("Why this version was created: {}", when);
    let trunk_what_doc = format!("What this version was created for: {}", what);
    let trunkver_doc = format!("Standard Trunk Version: {}", trunkver_value);
    let trunker_doc = format!(
        "Standard Trunk Version without leading v: {}",
        trunkver_value
    );

    let trunkver = quote! {
        #[doc = #trunk_why_doc]
        pub const TRUNKWHY: &str = #when;
        #[doc = #trunk_how_doc]
        pub const TRUNKHOW: &str = #how;
        #[doc = #trunk_what_doc]
        pub const TRUNKWHAT: &str = #what;
        #[doc = #trunkver_doc]
        pub const TRUNKVER: &str = #trunkver_value;
        #[doc = #trunker_doc]
        pub const TRUNKER: &str = #trunker_value;
    };

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let out_file = out_dir.join("output.rs");

    let mut file = fs::File::create(&out_file).expect("Unable to create trunkver file");
    file.write_all(prettyplease::unparse(&parse_file(&trunkver.to_string())?).as_bytes())?;

    Ok(())
}
