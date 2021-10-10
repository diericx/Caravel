#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod config;
mod mdlib;

use comrak::{markdown_to_html, ComrakOptions};
use regex::Captures;
use regex::Regex;
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[get("/")]
fn index(_config: State<config::Config>) -> Redirect {
    Redirect::to("/file/index.md")
}

#[get("/<path..>")]
fn file(config: State<config::Config>, path: PathBuf) -> Template {
    let mut full_path = PathBuf::from(&config.root_dir);
    full_path.push(path);

    let mut data = match fs::read_to_string(full_path) {
        Err(e) => format!("Error: {}", e.to_string()),
        Ok(f) => f,
    };
    data = tags_to_links_from_string(data, &config.tag_char);

    let html = markdown_to_html(&data, &ComrakOptions::default());

    let mut context = HashMap::new();
    context.insert("content".to_string(), html.to_string());
    return Template::render("notes/show", &context);
}

#[get("/")]
fn tags(config: State<config::Config>) -> Template {
    let mut context = HashMap::new();
    let tags = mdlib::get_tags(&config.root_dir, &config.tag_char);
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

    let files = mdlib::get_files_with_tag(&config.root_dir, &tag, &config.tag_char);
    return Template::render("tags/show", Context { files, tag });
}

fn main() {
    let config = match config::Config::new() {
        Err(e) => panic!("Error loading config: {}", e),
        Ok(c) => c,
    };

    rocket::ignite()
        .attach(Template::fairing())
        .mount("/public", StaticFiles::from("./static"))
        .mount("/tags", routes![tags, tag])
        .mount("/file", routes![file])
        .mount("/", routes![index])
        .manage(config)
        .launch();
}

fn tags_to_links_from_string(orig: String, tag_char: &String) -> String {
    let inline_code_regex =
        Regex::new(format!("`([A-Za-z0-9{} _-]+)`", tag_char).as_str()).unwrap();
    let tags_regex = Regex::new(format!("({}[A-Za-z0-9_-]+)", tag_char).as_str()).unwrap();
    let result = inline_code_regex.replace_all(&orig, |caps: &Captures| {
        let result = tags_regex.replace_all(&caps[1], |caps: &Captures| {
            let raw_tag = &caps[1].replace(tag_char, "");
            format!("[{}](/tags/{})", &caps[1], raw_tag)
        });
        return format!("{}", result);
    });
    return result.to_string();
}
