use std::path::PathBuf;

extern crate termion;

use termion::color;

enum FileType {
    Normal(String),
    Directory(String),
}

fn get_element_name(path: &str) -> &str {
    let start = path.rfind("/").map_or(0, |x| x + 1);
    &(path[start..path.len()])
}

fn file_type_to_print(file_type: FileType) {
    match file_type {
        FileType::Normal(x) => print!("{}{}  ", color::Fg(color::White), get_element_name(&x)),
        FileType::Directory(x) => print!("{}{}  ", color::Fg(color::Blue), get_element_name(&x)),
    };
}

pub fn print_ls(mut entries: Vec<PathBuf>) {
    entries.sort();
    for entry in entries {
        if entry.is_dir() {
            file_type_to_print(FileType::Directory(String::from(entry.to_str().unwrap())))
        } else {
            file_type_to_print(FileType::Normal(String::from(entry.to_str().unwrap())))
        }
    }
}
