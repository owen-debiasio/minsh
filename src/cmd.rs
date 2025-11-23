use std::process::{Command, Stdio};

use crate::fs::cd;

pub fn run(c: &str) {
    let mut p = c.split_whitespace();
    if let Some("cd") = p.next() {
        cd(p.next().unwrap_or("/"));
        return;
    }
    let _ = Command::new("bash")
        .arg("-c")
        .arg(c)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();
}
