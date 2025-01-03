use trunkver_lib::prelude::*;

use clap::Parser;

#[derive(Parser)]
#[command(name = "TrunkVer")]
#[command(about = "A CLI for generating trunk versioning")]
struct Cli {
    #[arg(short, long)]
    build_ref: Option<String>,
    #[arg(short = 'v', long, action)]
    with_v: bool,
}

fn main() {
    let cli = Cli::parse();

    match cli.build_ref {
        Some(build_ref) => match cli.with_v {
            true => println!("{:#?}", trunkver!(build_ref, true)),
            false => println!("{:#?}", trunkver!(build_ref)),
        },
        None => println!("{:#?}", trunkver!()),
    }
}
