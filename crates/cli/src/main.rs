use trunkver::TrunkVer;

use clap::Parser;

#[derive(Parser)]
#[command(name = "TrunkVer")]
#[command(about = "A CLI for generating trunk versioning")]
struct Cli {
    #[arg(short, long)]
    build_ref: String,
    #[arg(short, long)]
    source_ref: String,
    #[arg(short = 'v', long, action)]
    with_v: bool,
}

fn main() {
    let cli = Cli::parse();
    println!(
        "{}",
        TrunkVer::new(&cli.build_ref, &cli.source_ref, cli.with_v)
    );
}
