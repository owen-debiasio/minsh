use hostname::get;
use once_cell::sync::Lazy;
use std::env::var;

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
