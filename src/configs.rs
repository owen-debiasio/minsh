use std::fs::read_to_string;
use toml::Value;

use crate::_env::ENV;

pub fn load(cat: &str, var: &str, or_else: &str) -> String {
    let data: Value = toml::from_str(
        &read_to_string(&format!("{}/.config/minsh/config.toml", ENV.home)).unwrap_or_default(),
    )
    .unwrap();

    data.get(cat)
        .and_then(|pkg| pkg.get(var))
        .and_then(|v| v.as_str())
        .unwrap_or(or_else)
        .to_string()
}
