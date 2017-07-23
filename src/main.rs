extern crate clap;

use clap::*;

fn main() {
    App::new("win2wsl")
        .version("0.1")
        .about("Converts Windows paths to wsl paths")
        .author("Andrew Gaspar")
        .get_matches();
}
