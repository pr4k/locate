extern crate colored;
extern crate fstream;
extern crate walkdir;

use structopt::StructOpt;
use colored::*;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, StructOpt)]
struct Cmd {
    path:String,
    query:String,
    #[structopt(short, long)]
    color: bool,
}

fn check_dir(path: &str, query: &str, color: &bool) {
    let mut total_files_scanned = 0;
    for (fl_no, file) in WalkDir::new(path)
        .into_iter()
        .filter_map(|file| file.ok())
        .enumerate()
    {
        if file.metadata().unwrap().is_file() {
            match fstream::contains(file.path(), query) {
                Some(b) => {
                    if b {
                        check_file(file.path(), query, color);
                    }
                }
                None => println!("Error in walking Dir"),
            }
        }
        total_files_scanned = fl_no;
    }
    if *color == true {
        println!(
            "Total scanned files {}",
            total_files_scanned.to_string().bold()
        );
    } else {
        println!(
            "Total scanned files {}",
            total_files_scanned
        );
    }
}

fn check_file(file_path: &Path, query: &str, color: &bool) {
    println!(
        "In file {}\n",
        file_path.display().to_string().magenta().italic()
    );
    match fstream::read_lines(file_path) {
        Some(lines) => {
            for (pos, line) in &mut lines.iter().enumerate() {
                if line.contains(query) {
                    let line: String = line.trim().chars().take(2000).collect();
                    if *color == true {
                        print!("{}", "Line ".green().bold());
                        print!("{0: <6} ", pos.to_string().cyan());
                        println!("=> {}", line.blue());
                    } else {
                        print!("{}", "Line ");
                        print!("{0: <6} ", pos.to_string());
                        println!("=> {}", line);
                    }
                }
            }
        }
        None => println!("Error in reading File"),
    }
    println!("");
}

fn main() {

    let args = Cmd::from_args();

    let path = args.path;
    let query = args.query;

    if args.color == true {
        println!(
            "Searching '{}' in {}",
            query.green().bold(),
            path.italic()
        );
    }else {
        println!(
            "Searching '{}' in {}",
            query,
            path
        );
    }
    check_dir(&path, &query, &args.color);
}
