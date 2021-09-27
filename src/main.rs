#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use std::fs;
use std::path::PathBuf;

const ROOT: &str = "/home/zac/Nextcloud/notes";

#[get("/<path..>")]
fn file(path: PathBuf) -> String {
    let mut full_path = PathBuf::from(ROOT);
    full_path.push(path);

    let data = match fs::read_to_string(full_path) {
        Err(e) => String::from("error"),
        Ok(f) => f,
    };
    return data;
}

fn main() {
    rocket::ignite().mount("/", routes![hello, file]).launch();
}
