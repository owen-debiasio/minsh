use std::process::{Command, Stdio};

use crate::{configs, fs::cd};

pub fn run(c: &str) {
    let mut p = c.split_whitespace();
    if let Some("cd") = p.next() {
        cd(p.next().unwrap_or("/"));
        return;
    }

    let output = Command::new(configs::load("commands", "shell", "bash"))
        .arg("-c")
        .arg(c)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::piped())
        .output();

    match output {
        Ok(o) => {
            if let Ok(err) = String::from_utf8(o.stderr) {
                if err.contains("command not found") {
                    println!("minsh: {c} command not found");
                }
            }
        }
        Err(e) => eprintln!("{e}"),
    }
}
