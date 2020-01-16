extern crate clap;

use clap::{App, Arg};

mod print;
mod search;

fn main() {

    let matches = App::new("lsm")
        .version("0.0.1")
        .author("Marwan Liani")
        .about("ls project in Rust")
        .arg(Arg::with_name("DIRECTORY")
            .required(false)
            .takes_value(true)
            .index(1)
            .help("directory to list"))
        .get_matches();
    let directory = matches.value_of("DIRECTORY").unwrap_or(".");
    match search::search(directory) {
        Ok(entries) => print::print_ls(entries),
        Err(err) => panic!(err),
    }
}
