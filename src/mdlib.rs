// mdlib is a crate that helps explore a markdown library (read only)
use crate::TAG_CHAR;

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::BufReader;
use std::io::{self, BufRead};
use std::path::PathBuf;
use walkdir::DirEntry;
use walkdir::WalkDir;

#[derive(Serialize)]
pub struct File {
    name: String,
    path: String,
    tags: Vec<String>,
}

const MD_EXTENSIONS: [&str; 5] = ["md", "markdown", "mdown", "mkdn", "mkd"];

// Checks if a walkdir::DirEntry is a hidden file
fn is_file_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

// Checks if a walkdir::DirEntry is a markdown file
fn is_file_markdown(entry: &DirEntry) -> bool {
    let ext = match entry.path().extension() {
        Some(ext) => ext,
        None => return false,
    };
    return MD_EXTENSIONS.contains(&ext.to_str().unwrap_or(""));
}

// Returns a vector of markdown files recursively searched in the provided directory
fn get_markdown_files_recursive(root: String) -> Vec<DirEntry> {
    return WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
        .filter(|e| !is_file_hidden(e))
        .filter(|e| is_file_markdown(e))
        .collect();
}

fn get_tags_from_file(file: &DirEntry) -> Vec<String> {
    let tags: Vec<String> = Vec::new();
    // Open the file
    let file = match fs::File::open(file.path()) {
        Ok(file) => file,
        Err(_) => panic!("Unable to read title from {}", file.path().display()),
    };
    let mut buffer = BufReader::new(file);

    // Get the first line
    let mut first_line = String::new();
    match buffer.read_line(&mut first_line) {
        Ok(n) => n,
        Err(_) => return Vec::new(),
    };

    return get_tags_from_line(first_line);
}

fn get_tags_from_line(line: String) -> Vec<String> {
    let mut tags: Vec<String> = Vec::new();

    // Opinionated rule, tag line is the first line if and only if it is a simple code line
    let re = Regex::new(r"^`([A-Za-z0-9# _-]+)`").unwrap();
    let line_str = line.to_string();
    let tags_captures = match re.captures(&line_str) {
        None => return tags,
        Some(c) => c,
    };
    let tags_str = match tags_captures.get(1) {
        None => return tags,
        Some(c) => c.as_str(),
    };

    tags_str
        .split(" ")
        .map(|t| t.replace(TAG_CHAR, ""))
        .for_each(|t| tags.push(t.to_string()));

    return tags;
}

pub fn get_tags(dir: &str) -> Vec<String> {
    let mut tags: Vec<String> = Vec::new();
    let root: String = dir.to_string();
    let files = get_markdown_files_recursive(dir.to_string());
    for f in files.into_iter() {
        tags.extend(get_tags_from_file(&f));
    }
    tags.sort_unstable();
    tags.dedup();
    return tags;
}

pub fn get_files_with_tag(dir: &str, tag: &String) -> Vec<File> {
    let mut files: Vec<File> = Vec::new();
    let root: String = dir.to_string();
    let dir_entries = get_markdown_files_recursive(dir.to_string());
    for f in dir_entries.into_iter() {
        let tags = get_tags_from_file(&f);
        let relative_path = f.path().strip_prefix(crate::ROOT).unwrap();
        if tags.contains(&tag) {
            files.push(File {
                name: f.file_name().to_string_lossy().to_string(),
                path: relative_path.to_string_lossy().to_string(),
                tags,
            });
        }
    }
    return files;
}
