use hostname::get;
use once_cell::sync::Lazy;
use std::env::{self, set_var, var};

pub struct Env {
    pub username: String,
    pub home: String,
    pub hostname: String,
}

pub static ENV: Lazy<Env> = Lazy::new(|| Env {
    username: var("USER").unwrap_or_else(|_| "unknown".into()),
    home: var("HOME").unwrap_or_else(|_| "/".into()),
    hostname: get()
        .ok()
        .map(|h| h.to_string_lossy().into_owned())
        .unwrap_or_else(|| "unknown".into()),
});

pub fn set_path() {
    let mut path = env::var("PATH").unwrap_or_default();

    let defaults = [
        "/usr/local/sbin",
        "/usr/local/bin",
        "/usr/sbin",
        "/usr/bin",
        "/sbin",
        "/bin",
        &format!("{}/.local/bin", env::var("HOME").unwrap_or_default()),
    ];

    for dir in defaults {
        if !path.split(':').any(|p| p == dir) {
            if !path.is_empty() {
                path.push(':');
            }
            path.push_str(dir);
        }
    }

    unsafe { set_var("PATH", path) };
}
