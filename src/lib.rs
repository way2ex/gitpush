// use std::error::Error;
use regex::Regex;

/// find `git push xxx` command to execute
pub fn find_gitpush_command<'a>(content: &'a str) -> Option<&'a str> {
    let re = Regex::new(r"git push[^\n]+").expect("Failed to create regular expression.");
    let ma = re.find(content)?;
    Some(ma.as_str())
}