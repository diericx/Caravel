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
use walkdir::WalkDir;

const ROOT: &str = env!("NOTES_ROOT");

fn get_tags(root: String) {
    let mut filenames = HashMap::new();

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let f_name = String::from(entry.file_name().to_string_lossy());
        let counter = filenames.entry(f_name.clone()).or_insert(0);
        *counter += 1;

        if *counter == 2 {
            println!("{}", f_name);
        }
    }
}

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
