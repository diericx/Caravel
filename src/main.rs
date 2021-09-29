#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod mdlib;

use comrak::{markdown_to_html, ComrakOptions};
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

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
    return Template::render("notes/show", &context);
}

#[get("/")]
fn tags() -> Template {
    let mut context = HashMap::new();
    let tags = mdlib::get_tags(ROOT);
    context.insert(String::from("tags"), tags);
    return Template::render("tags/index", &context);
}

#[get("/<tag>")]
fn tag(tag: String) -> Template {
    #[derive(Serialize)]
    struct Context {
        files: Vec<mdlib::File>,
        tag: String,
    }

    let files = mdlib::get_files_with_tag(ROOT, &tag);
    return Template::render("tags/show", Context { files, tag });
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
        .mount("/tags", routes![tags, tag])
        .launch();
}
