extern crate argparse;
extern crate colored;
extern crate fstream;
extern crate walkdir;

use argparse::{ArgumentParser, Store};
use colored::*;
use std::path::Path;
use walkdir::WalkDir;

fn check_dir(path: &str, query: &str) {
    for e in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if e.metadata().unwrap().is_file() {
            match fstream::contains(e.path(), query) {
                Some(b) => {
                    if b {
                        check_file(e.path(), query);
                    }
                }
                None => println!("nothing"),
            }
        }
    }
}

fn check_file(file_path: &Path, query: &str) {
    match fstream::read_lines(file_path) {
        Some(s) => {
            for (pos, s) in s.iter().enumerate() {
                if s.contains(query) {
                    print!("{}", "Line ".green());
                    print!("{} ", pos.to_string().yellow());
                    println!("=> {}", s.trim().red());
                }
            }
        }
        None => println!("Nothing 2"),
    }
}

fn main() {
    let mut path = ".".to_string();
    let mut query = "query".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Recursive string locater in files");
        ap.refer(&mut path)
            .add_option(&["-p", "--path"], Store, "Path to folder");
        ap.refer(&mut query)
            .add_option(&["-q", "--query"], Store, "Query string to find");
        ap.parse_args_or_exit();
    }
    println!("{} {}", path, query);
    check_dir(&path, &query);
}
