mod env;
mod linter;
mod manifest;

use clap::Parser;
use colored::*;
use env::Env;
use linter::*;
use manifest::Manifest;
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::process;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 'i', long = "input", default_value = ".halfpipe.io", help = "Path to manifest")]
    path: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let env = Env::new();
    eprintln!("{}", env); //just trying out Display trait

    let raw_manifest = read_to_string(&args.path)?;
    let manifest = Manifest::from_yaml(&raw_manifest)?;

    let linter = Linter::new(&env);
    let lint_issues = linter.lint(&manifest);

    for issue in &lint_issues {
        match issue {
            Issue::Error(s) => eprintln!("{} {}", "  [error]".red(), s),
            Issue::Warning(s) => eprintln!("{} {}", "[warning]".yellow(), s),
        }
    }

    if lint_issues.contains_error() {
        process::exit(1)
    }

    let yaml = manifest.to_yaml()?;
    println!("{}", yaml);

    Ok(())
}
