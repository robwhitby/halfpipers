mod linter;
mod manifest;

use crate::linter::Linter;
use clap::Parser;
use manifest::Manifest;
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::process;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(
        short = 'i',
        long = "input",
        default_value = ".halfpipe.io",
        help = "Path to manifest"
    )]
    path: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let raw_manifest = read_to_string(&args.path)?;

    let manifest = Manifest::from_yaml(&raw_manifest)?;

    let linter = Linter::new();
    let lint_results = linter.lint(&manifest);

    for s in &lint_results.errors {
        eprintln!("  [error] {}", s)
    }

    lint_results
        .warnings
        .iter()
        .for_each(|s| eprintln!("[warning] {}", s));

    if lint_results.has_errors() {
        process::exit(1)
    }

    let yaml = manifest.to_yaml()?;
    println!("{}", yaml);

    Ok(())
}
