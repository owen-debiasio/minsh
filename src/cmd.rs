use std::{
    path::Path,
    process::{Command, Stdio},
};

use crate::_env::set_path;

pub fn run(command_line: &str) {
    if command_line.is_empty() {
        return;
    }

    set_path();

    let mut parts = command_line.split('|').map(str::trim);

    let (first_cmd, first_args) = split_command(parts.next().unwrap());

    if let Some(second_part) = parts.next() {
        let (second_cmd, second_args) = split_command(second_part);

        if command_exists(&first_cmd) {
            let mut first_child = Command::new(&first_cmd)
                .args(&first_args)
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();

            if command_exists(&second_cmd) {
                Command::new(&second_cmd)
                    .args(&second_args)
                    .stdin(first_child.stdout.take().unwrap())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();
            } else {
                eprintln!("minsh: {second_cmd} command not found");
            }

            first_child.wait().unwrap();
        } else {
            eprintln!("minsh: {first_cmd} command not found");
        }
    } else {
        exec_if_exists(&first_cmd, &first_args);
    }
}

fn split_command(part: &str) -> (String, Vec<String>) {
    let mut iter = part.split_whitespace();
    let cmd = iter.next().unwrap().to_string();
    let args = iter.map(String::from).collect();
    (cmd, args)
}

fn command_exists(cmd: &str) -> bool {
    Path::new(cmd).exists() || which::which(cmd).is_ok()
}

fn exec_if_exists(cmd: &str, args: &[String]) {
    if command_exists(cmd) {
        Command::new(cmd)
            .args(args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .expect("Failed to execute command.");
    } else {
        eprintln!("minsh: {cmd} command not found");
    }
}
