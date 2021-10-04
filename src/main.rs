#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod config;
mod mdlib;

use comrak::{markdown_to_html, ComrakOptions};
use rocket::State;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use serde::Serialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

const TAG_CHAR: &str = "#";

#[get("/<path..>")]
fn file(config: State<config::Config>, path: PathBuf) -> Template {
    let mut full_path = PathBuf::from(&config.root_dir);
    full_path.push(path);

    let data = match fs::read_to_string(full_path) {
        Err(e) => format!("Error: {}", e.to_string()),
        Ok(f) => f,
    };
    let html = markdown_to_html(&data, &ComrakOptions::default());

    let mut context = HashMap::new();
    context.insert("content".to_string(), html.to_string());
    return Template::render("notes/show", &context);
}

#[get("/")]
fn tags(config: State<config::Config>) -> Template {
    let mut context = HashMap::new();
    let tags = mdlib::get_tags(&config.root_dir);
    context.insert(String::from("tags"), tags);
    return Template::render("tags/index", &context);
}

#[get("/<tag>")]
fn tag(config: State<config::Config>, tag: String) -> Template {
    #[derive(Serialize)]
    struct Context {
        files: Vec<mdlib::File>,
        tag: String,
    }

    let files = mdlib::get_files_with_tag(&config.root_dir, &tag);
    return Template::render("tags/show", Context { files, tag });
}

fn main() {
    let config = match config::Config::new() {
        Err(e) => panic!("Error loading config: {}", e),
        Ok(c) => c,
    };

    rocket::ignite()
        .attach(Template::fairing())
        .mount(
            "/public",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .mount("/notes", routes![file])
        .mount("/tags", routes![tags, tag])
        .manage(config)
        .launch();
}
