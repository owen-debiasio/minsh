use rustyline::DefaultEditor;
use std::{path::PathBuf, process::exit};

pub mod _env;
pub mod cmd;
pub mod configs;
pub mod fs;

use _env::*;
use fs::*;

static VER: &str = "v0.1.3";

fn main() {
    let mut editor = DefaultEditor::new().unwrap();
    let history = PathBuf::from(&ENV.home).join(".minsh_history");
    let _ = editor.load_history(&history);

    cd(&configs::load("path", "open_to", &ENV.home));

    loop {
        let line = editor
            .readline(&format!("{}@{} {} $: ", ENV.hostname, ENV.username, cwd()))
            .unwrap_or_default();
        let _ = editor.add_history_entry(&line);
        let _ = editor.save_history(&history);

        match line.as_str() {
            "exit" => exit(0),
            "help" => println!("minsh {VER}\n\nCommands:\n\nexit\nhelp\nhistory\n"),
            "history" => {
                for (i, x) in editor.history().iter().enumerate() {
                    println!("{i}: {x}");
                }
            }
            _ => cmd::run(&line),
        }
    }
}
