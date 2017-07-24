#[macro_use]
extern crate clap;

use clap::*;
use std::ascii::AsciiExt;
use std::io::stdin;
use std::path::{Component, Path, Prefix};

fn convert_relative_path(path: &Path) -> String {
    use std::fmt::Write;

    debug_assert!(path.is_relative());

    let mut wsl_path = String::new();

    for (index, component) in path.components().enumerate() {
        if index > 0 {
            write!(&mut wsl_path, "/").unwrap();
        }

        assert_ne!(
            Component::RootDir,
            component,
            "TODO: Not sure how to resolve root directories"
        );

        let comp_str = component.as_os_str().to_str().unwrap();

        write!(&mut wsl_path, "{}", comp_str).unwrap();
    }

    wsl_path
}

fn convert_absolute_path(path: &Path) -> Option<String> {
    use std::fmt::Write;

    debug_assert!(path.is_absolute());

    let mut wsl_path = "/mnt/".to_string();

    for (index, component) in path.components().enumerate() {
        match index {
            0 => {
                match component {
                    Component::Prefix(pre) => {
                        match pre.kind() {
                            Prefix::Disk(disk) |
                            Prefix::VerbatimDisk(disk) => {
                                write!(
                                    &mut wsl_path,
                                    "{}",
                                    disk.to_ascii_lowercase() as char
                                ).unwrap()
                            }
                            _ => {
                                eprintln!(
                                    "Can only convert absolute paths that start with a drive letter."
                                );
                                return None;
                            }
                        }
                    }
                    _ => {
                        assert!(
                            false,
                            "Absolute Windows path must start with a Prefix"
                        )
                    }
                }
            }
            1 => debug_assert_eq!(Component::RootDir, component),
            _ => {
                let comp_str = component.as_os_str().to_str().unwrap();
                write!(&mut wsl_path, "/{}", comp_str).unwrap();
            }
        };
    }

    Some(wsl_path)
}

fn convert_windows_path(path: &Path) -> Option<String> {
    if path.is_relative() {
        Some(convert_relative_path(path))
    } else {
        convert_absolute_path(path)
    }
}

#[test]
fn test_bare() {
    assert_eq!("banana", convert_windows_path(Path::new("banana")).unwrap());
}

#[test]
fn test_relative() {
    assert_eq!(
        "banana/foo",
        convert_windows_path(Path::new("banana\\foo")).unwrap()
    );
}

#[test]
fn test_cwd() {
    assert_eq!(
        "./banana/foo",
        convert_windows_path(Path::new(".\\banana\\foo")).unwrap()
    );
}

#[test]
fn test_parent() {
    assert_eq!(
        "../banana/foo",
        convert_windows_path(Path::new("..\\banana\\foo")).unwrap()
    );
}

#[test]
fn test_absolute() {
    assert_eq!(
        "/mnt/c/foo/bar",
        convert_windows_path(Path::new("C:\\foo\\bar")).unwrap()
    );
}

#[test]
fn test_verbatim_absolute() {
    assert_eq!(
        "/mnt/c/foo/bar",
        convert_windows_path(Path::new("\\\\?\\C:\\foo\\bar")).unwrap()
    );
}

#[test]
fn no_network_paths() {
    assert_eq!(
        None,
        convert_windows_path(Path::new("\\\\net-comp\\netshare\\file\\path.txt"))
    );
}

#[cfg(windows)]
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

        match convert_windows_path(&Path::new(&line)) {
            Some(path) => println!("{}", path),
            None => println!("{}", line),
        }
    }
}
