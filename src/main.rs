#![allow(unused)]
use std::{fs::read_to_string, process::ExitStatus};

use clap::Parser;
use anyhow::{Context, Result};
use log::{info, warn};
use std::process::Command;
use regex::Regex;

#[derive(Parser)]
struct Cli {
    // the pattern to look for
    pattern: String,
    // the path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let output = Command::new("git").arg("push").args(&args[1..]).output().expect("failed to execute \"git push\"");

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
        return Ok(());
    }

    let output_string = String::from_utf8_lossy(&output.stderr);
    if !output_string.contains("To push the current branch and set the remote as upstream, use") {
        println!("{}", &output_string);
        return Ok(());
    }

    let re = Regex::new(r"git push[ \-\w]+\n").unwrap();
    let cmd_to_run = match re.find(&output_string) {
        Some(x) => &output_string[(x.start() + 4)..(x.end() - 1)],
        _ => "",
    };
    println!("cmd_to_run is {}", cmd_to_run);
    let args: Vec<&str> = cmd_to_run.split(" ")
        .filter(|x| *x != "git").collect();
    let output =  Command::new("git")
        .args(
            args
        )
        .output()
        .expect(&(String::from("failed to execute \" git ") + cmd_to_run + &String::from("\"")));

    let str = String::from_utf8_lossy(
        match &output.status.success() {
            true => &output.stdout,
            false => &output.stderr,
        }
    );
    println!("{}", str);
    Ok(())

}
