use hostname::get;
use once_cell::sync::Lazy;
use rustyline::DefaultEditor;
use std::{
    env::{self, current_dir, set_current_dir},
    path::PathBuf,
    process::{exit, Command, Stdio},
};

struct Env {
    u: String,
    h: String,
    hn: String,
}

static ENV: Lazy<Env> = Lazy::new(|| Env {
    u: env::var("USER").unwrap_or("unknown".into()),
    h: env::var("HOME").unwrap_or('/'.into()),
    hn: get()
        .map(|h| h.to_string_lossy().into_owned())
        .unwrap_or("unknown".into()),
});

fn run(c: &str) {
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

fn cwd() -> String {
    let c = current_dir()
        .ok()
        .and_then(|p| p.to_str().map(String::from))
        .unwrap_or('~'.into());
    if c == ENV.h {
        '~'.into()
    } else if let Some(r) = c.strip_prefix(&(ENV.h.clone() + "/")) {
        format!("~/{r}")
    } else {
        c
    }
}

fn cd(d: &str) {
    let d = match d {
        "~" | "$HOME" => ENV.h.clone(),
        _ if d.starts_with("~/") => format!("{}/{}", ENV.h, &d[2..]),
        _ if d.starts_with("$HOME/") => format!("{}/{}", ENV.h, &d[6..]),
        _ => d.to_string(),
    };
    if let Err(e) = set_current_dir(&d) {
        eprintln!("cd: {d}: {e}");
    }
}

fn main() {
    let mut e = DefaultEditor::new().unwrap();
    let h = PathBuf::from(&ENV.h).join(".minsh_history");
    let _ = e.load_history(&h);

    cd(&ENV.h);

    loop {
        let l = e
            .readline(&format!("{}@{} {} $: ", ENV.hn, ENV.u, cwd()))
            .unwrap_or_default();
        let _ = e.add_history_entry(&l);
        let _ = e.save_history(&h);
        if l == "exit" {
            exit(0);
        }
        run(&l);
    }
}
