#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use comrak::{markdown_to_html, ComrakOptions};
use rocket::response::{content, status};
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

const ROOT: &str = env!("NOTES_ROOT");

#[get("/<path..>")]
fn file(path: PathBuf) -> Template {
    let mut full_path = PathBuf::from(ROOT);
    full_path.push(path);

    let data = match fs::read_to_string(full_path) {
        Err(e) => String::from("error"),
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
    context.insert("tags".to_string(), "".to_string());
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
