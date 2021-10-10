use std::env;

pub struct Config {
    pub root_dir: String,
    pub tag_char: String,
    pub index_file: String,
}

const DEFAULT_TAG_CHAR: &str = "#";
const DEFAULT_ROOT_DIR: &str = "/notes";
const DEFAULT_INDEX_FILE: &str = "index.md";

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let root_dir = match env::var_os("ROOT_DIR") {
            Some(v) => v.into_string().unwrap(),
            None => DEFAULT_ROOT_DIR.to_string(),
        };

        let tag_char = match env::var_os("TAG_CHAR") {
            Some(v) => v.into_string().unwrap(),
            None => DEFAULT_TAG_CHAR.to_string(),
        };

        let index_file = match env::var_os("INDEX_FILE") {
            Some(v) => v.into_string().unwrap(),
            None => DEFAULT_INDEX_FILE.to_string(),
        };

        Ok(Config {
            root_dir,
            tag_char,
            index_file,
        })
    }
}
