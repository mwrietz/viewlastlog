//use colored::Colorize;
use glob::glob;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod tui_gen;

struct TermStat {
    line_count: usize,
    height: usize,
}

impl Default for TermStat {
    fn default() -> TermStat {
        TermStat {
            line_count: 0,
            height: 0,
        }
    }
}

impl TermStat {
    fn line_check(&mut self) {
        self.line_count += 1;
        if self.line_count > (self.height - 5) {
            tui_gen::pause();
            self.line_count = 0;
            tui_gen::cls();
            tui_gen::cmove(0, 0);
        }
    }
}


fn main() {

    let (_width, height) = tui_gen::tsize();
    let mut termstat = TermStat::default();
    termstat.height = height;

    tui_gen::cls();
    println!("viewlastlog: v{}\n", env!("CARGO_PKG_VERSION"));
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
        let l = line.clone();
        if !l.ends_with("/") && !l.starts_with(" ") {
            println!("{}", l);
            termstat.line_check();
        }
    }
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
