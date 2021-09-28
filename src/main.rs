#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod mdlib;

use comrak::{markdown_to_html, ComrakOptions};
use regex::Regex;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use std::fs;
use std::io::BufReader;
use std::io::{self, BufRead};
use std::path::PathBuf;
use walkdir::DirEntry;
use walkdir::WalkDir;

const ROOT: &str = env!("NOTES_ROOT");
const TAG_CHAR: &str = "#";

#[get("/<path..>")]
fn file(path: PathBuf) -> Template {
    let mut full_path = PathBuf::from(ROOT);
    full_path.push(path);

    let data = match fs::read_to_string(full_path) {
        Err(e) => format!("Error: {}", e.to_string()),
        Ok(f) => f,
    };
    let html = markdown_to_html(&data, &ComrakOptions::default());

    let mut context = HashMap::new();
    context.insert("content".to_string(), html.to_string());
    return Template::render("note", &context);
}

#[get("/")]
fn tags() -> Template {
    let mut context = HashMap::new();
    let tags = mdlib::get_tags_from_dir(ROOT);
    context.insert(String::from("tags"), tags);
    return Template::render("tags", &context);
}

fn main() {
    rocket::ignite();

    rocket::ignite()
        .attach(Template::fairing())
        .mount(
            "/public",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .mount("/notes", routes![file])
        .mount("/tags", routes![tags])
        .launch();
}
