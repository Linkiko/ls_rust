extern crate clap;

use crate::print;
use crate::search;
use clap::{App, Arg, ArgMatches};

struct Params<'a> {
    long: bool,
    arg: &'a str,
}

fn get_app() -> ArgMatches<'static> {
    App::new("lsm")
        .version("0.0.1")
        .author("Marwan Liani")
        .about("ls project in Rust")
        .arg(Arg::with_name("LONG").short("l"))
        .arg(
            Arg::with_name("DIRECTORY")
                .required(false)
                .takes_value(true)
                .index(1)
                .help("directory to list"),
        )
        .get_matches()
}

fn handle_params(params: Params) {
    let result = search::search(params.arg).unwrap();
    if params.long == true {
        println!("Is long")
    }
    print::print_ls(result);
}

pub fn launch_app() {
    let matches = get_app();
    println!("{}", matches.is_present("LONG"));
    let params = Params {
        long: matches.is_present("LONG"),
        arg: matches.value_of("DIRECTORY").unwrap_or("."),
    };
    handle_params(params)
}
