mod manifest;

use clap::Parser;
use manifest::Manifest;
use std::error::Error;
use std::path::PathBuf;

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
    let raw_manifest = std::fs::read_to_string(&args.path)?;
    let manifest = Manifest::from_string(&raw_manifest)?;

    println!("pipeline = {}, team = {}", manifest.pipeline, manifest.team);

    Ok(())
}
