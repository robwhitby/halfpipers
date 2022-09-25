mod app;
mod env;
mod linter;
mod manifest;
mod render;

use crate::app::App;
use crate::render::{ConcourseRenderer, Render};
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
    let raw_manifest = read_to_string(&args.path)?;
    let manifest = Manifest::from_yaml(&raw_manifest)?;

    let app = App::new(Env::new(), Linter::new(), ConcourseRenderer);

    let issues = app.lint(&manifest);
    for issue in &issues {
        match issue {
            Issue::Error(s) => eprintln!("{} {}", "  [error]".red(), s),
            Issue::Warning(s) => eprintln!("{} {}", "[warning]".yellow(), s),
        }
    }

    if issues.contains_error() {
        process::exit(1)
    }

    println!("{}", app.render(&manifest));
    Ok(())
}
