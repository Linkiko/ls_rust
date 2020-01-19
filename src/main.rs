mod app;
mod print;
mod search;

fn main() {
    app::launch_app();

    /*let directory = matches.value_of("DIRECTORY").unwrap_or(".");
    match search::search(directory) {
        Ok(entries) => print::print_ls(entries),
        Err(err) => panic!(err),
    }*/
}
