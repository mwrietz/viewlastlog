use colored::Colorize;
use glob::glob;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod tui_gen;
mod gh_repo_status;

fn main() {

    let mut termstat = tui_gen::TermStat::default();

    tui_gen::cls();
    println!("{}", format!("{} v{}\n", "viewlastlog:", env!("CARGO_PKG_VERSION")).blue());
    termstat.line_check();

    // get last filename in cwd (latest log file)
    let list = glob("*").expect("Failed to read glob pattern");
    let mut listvector = Vec::new();
    for l in list {
        let buf = format!("{}", l.unwrap().to_str().unwrap());
        if buf.ends_with(".log") {
            listvector.push(buf);
        }
    }
    let path = Path::new(listvector.last().unwrap());
    println!("path = {}", &path.display());
    termstat.line_check();
    println!();
    termstat.line_check();

    // read log file into vector of lines
    let mut lines = Vec::new();
    read_file_to_vector(&path, &mut lines);

    // print lines containing filenames
    for line in &lines {
        if termstat.line_count == 0 {
            println!("{}", format!("{} v{}\n", "viewlastlog:", env!("CARGO_PKG_VERSION")).blue());
            termstat.line_check();
        }
        let l = line.clone();
        if !l.ends_with("/") && !l.starts_with(" ") {
            println!("{}", l);
            termstat.line_check();
        }
    }

    // check if latest version is running
    gh_repo_status::check_version()
        .expect("check_version error");
}

fn read_file_to_vector(file_path: &Path, vector: &mut Vec<String>) {
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                vector.push(ip);
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
