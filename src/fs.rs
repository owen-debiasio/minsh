use std::env::{current_dir, set_current_dir};

use crate::ENV;

pub fn cd(dir: &str) {
    if dir.trim().is_empty() {
        return;
    }

    let dir = match dir {
        "~" | "$HOME" => ENV.home.clone(),
        _ if dir.starts_with("~/") => format!("{}/{}", ENV.home, &dir[2..]),
        _ if dir.starts_with("$HOME/") => format!("{}/{}", ENV.home, &dir[6..]),
        _ => dir.to_string(),
    };

    if let Err(e) = set_current_dir(&dir) {
        eprintln!("cd: {dir}: {e}");
    }
}

pub fn cwd() -> String {
    let cwd = current_dir()
        .ok()
        .and_then(|p| p.to_str().map(String::from))
        .unwrap_or('~'.into());
    if cwd == ENV.home {
        "~".into()
    } else if let Some(r) = cwd.strip_prefix(&(ENV.home.clone() + "/")) {
        format!("~/{r}")
    } else {
        cwd
    }
}
