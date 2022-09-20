mod lint_rules;
mod linter;
mod manifest;

use crate::lint_rules::Issue;
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
    let lint_issues = linter.lint(&manifest);

    let mut has_errors = false;
    for issue in lint_issues {
        match issue {
            Issue::Error(s) => {
                eprintln!("  [error] {}", s);
                has_errors = true;
            }
            Issue::Warning(s) => eprintln!("[warning] {}", s),
        }
    }

    if has_errors {
        process::exit(1)
    }

    let yaml = manifest.to_yaml()?;
    println!("{}", yaml);

    Ok(())
}
