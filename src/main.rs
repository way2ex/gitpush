#![allow(unused)]
use std::fs::read_to_string;

use clap::Parser;
use anyhow::{Context, Result};
use log::{info, warn};

#[derive(Parser)]
struct Cli {
    // the pattern to look for
    pattern: String,
    // the path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}
fn main() -> Result<()> {
    let pattern = std::env::args().nth(1).expect("no pattern giver");
    let path = std::env::args().nth(2).expect("no path giver");
    let args = Cli {
        pattern,
        path: std::path::PathBuf::from(path),
    };
    let args = Cli::parse();

    println!("Your pattern is {}, and your path is {:?}", args.pattern, args.path);

    // let content = std::fs::read_to_string(&args.path)
    //     .expect("could not read file");

    let content = read_to_string(&args.path)
        .with_context(|| format!("could not read file `{:?}`", args.path))?;

    gitpush::find_matches(&content, &args.pattern, &mut std::io::stdout());

    // env_logger::init();
    // info!("starting up");
    // warn!("oops, nothing implemented!");

    Ok(())

}
