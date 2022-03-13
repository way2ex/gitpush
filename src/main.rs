#![allow(unused)]
use std::{fs::read_to_string, process::ExitStatus};

use anyhow::{Context, Result};
use gitpush::find_gitpush_command;
use log::{info, warn};
use std::process::{Command,Stdio};
use regex::Regex;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let output = Command::new("git")
        .arg("push")
        .args(&args[1..])
        .stdout(Stdio::inherit())
        .stderr(Stdio::piped())
        .output()
        .expect("failed to execute \"git push\"");

    let output_string = String::from_utf8_lossy(&output.stderr);
    if output.status.success() {
        if (output_string.len() > 0) {
            println!("{}", &output_string);
        }
        return Ok(());
    }

    if !output_string.contains("To push the current branch and set the remote as upstream, use") {
        println!("{}", &output_string);
        return Ok(());
    }

    let cmd_to_run = find_gitpush_command(&output_string).unwrap_or("");
    if (cmd_to_run.eq("")) {
        println!("{}", &output_string);
        return Ok(());
    }

    let args: Vec<&str> = cmd_to_run.split(" ")
        .filter(|x| *x != "git").collect();
    let output =  Command::new("git")
        .args(
            args
        )
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .expect(&(String::from("failed to execute \" git ") + cmd_to_run + &String::from("\"")));

    Ok(())
}
