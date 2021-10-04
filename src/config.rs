use std::env;

pub struct Config {
    pub root_dir: String,
}

const DEFAULT_ROOT_DIR: &str = "/notes";

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let root_dir = match env::var_os("ROOT_DIR") {
            Some(v) => v.into_string().unwrap(),
            None => DEFAULT_ROOT_DIR.to_string(),
        };

        Ok(Config { root_dir })
    }
}
