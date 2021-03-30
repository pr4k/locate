extern crate colored;
extern crate fstream;
extern crate walkdir;
extern crate regex;

use clap::{Clap, IntoApp};
use clap_generate::{generate, generators::*};
use colored::*;
use regex::Regex;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Clap)]
#[clap(author, about, version)]
struct Cmd {
    #[clap(short, long)]
    path: Option<String>,
    #[clap(short, long)]
    query: Option<String>,
    #[clap(short, long)]
    regexp: bool,
    #[clap(short, long)]
    color: bool,
    #[clap(short, long, arg_enum, value_name = "SHELL")]
    shellcompletions: Option<Shell>,
}

#[derive(Clap, Copy, Clone)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Elvish,
}

impl Shell {
    pub fn generate(&self) {
        let mut app = Cmd::into_app();
        let mut fd = std::io::stdout();
        match self {
            Shell::Bash => generate::<Bash, _>(&mut app, "locate", &mut fd),
            Shell::Zsh => generate::<Zsh, _>(&mut app, "locate", &mut fd),
            Shell::Fish => generate::<Fish, _>(&mut app, "locate", &mut fd),
            Shell::PowerShell => generate::<PowerShell, _>(&mut app, "locate", &mut fd),
            Shell::Elvish => generate::<Elvish, _>(&mut app, "locate", &mut fd),
        }
    }
}

fn check_dir(path: &str, query: &str, color: &bool, regexp: &bool) {
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
                        check_file(file.path(), query, color, regexp);
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
        println!("Total scanned files {}", total_files_scanned);
    }
}

fn check_file(file_path: &Path, query: &str, color: &bool, regexp:&bool) {
    if *color == true {
        println!(
            "In file {}\n",
            file_path.display().to_string().magenta().italic()
        );
    } else {
        println!("In file {}\n", file_path.display().to_string());
    }
    match fstream::read_lines(file_path) {
        Some(lines) => {
            for (pos, line) in &mut lines.iter().enumerate() {
                if line.contains(query) {
                    let line: String = line.trim().chars().take(2000).collect();
                    if *color {
                        print!("{}", "Line ".green().bold());
                        print!("{0: <6} ", pos.to_string().cyan());
                        println!("=> {}", line.blue());
                    } else {
                        print!("{}", "Line ");
                        print!("{0: <6} ", pos.to_string());
                        println!("=> {}", line);
                    }
                    if *regexp {
                        let re = Regex::new(query).unwrap();
                        if re.is_match(&line){
                            let line: String = line.trim().chars().take(2000).collect();
                            if *color {
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
            }
        }
        None => println!("Error in reading File"),
    }
    println!("");
}

fn main() {

    let mut regexp= false;
    let args = Cmd::parse();


    if let Some(shell) = args.shellcompletions {
        shell.generate();
        std::process::exit(0);
    }

    let path = args.path.unwrap();
    let query = args.query.unwrap();
    regexp = args.regexp;

    if args.color == true {
        println!("Searching '{}' in {}", query.green().bold(), path.italic());
    } else {
        println!("Searching '{}' in {}", query, path);
    }
    check_dir(&path, &query, &args.color, &regexp);

}
