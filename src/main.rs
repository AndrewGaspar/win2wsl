#[macro_use]
extern crate clap;

use clap::*;
use std::io::stdin;
use std::path::{Component, Path};

fn convert_windows_path(path: &Path) -> String {
    use std::fmt::Write;

    if path.is_relative() {
        let mut wsl_path = String::new();

        for (index, component) in path.components().enumerate() {
            if index > 0 {
                write!(&mut wsl_path, "/").expect("out failure");
            }

            assert_ne!(
                Component::RootDir,
                component,
                "TODO: Not sure how to resolve root directories"
            );

            let comp_str = component.as_os_str().to_str().expect("");

            write!(&mut wsl_path, "{}", comp_str).expect("out failure");
        }

        wsl_path
    } else {
        unimplemented!();
    }
}

#[test]
fn test_bare() {
    assert_eq!("banana", convert_windows_path(Path::new("banana")));
}

#[test]
fn test_relative() {
    assert_eq!("banana/foo", convert_windows_path(Path::new("banana\\foo")));
}

#[test]
fn test_cwd() {
    assert_eq!(
        "./banana/foo",
        convert_windows_path(Path::new(".\\banana\\foo"))
    );
}

#[test]
fn test_parent() {
    assert_eq!(
        "../banana/foo",
        convert_windows_path(Path::new("..\\banana\\foo"))
    );
}

fn main() {
    App::new("win2wsl")
        .version(crate_version!())
        .about("Converts Windows paths to wsl paths")
        .author("Andrew Gaspar")
        .arg(
            Arg::with_name("rev")
                .help("Converts wsl paths to Windows paths")
                .long("reverse")
                .short("r"),
        )
        .get_matches();

    loop {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("stdin terminated");

        println!("{}", convert_windows_path(&Path::new(&line)));
    }
}
