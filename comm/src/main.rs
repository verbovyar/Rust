#![forbid(unsafe_code)]

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn eprintln_usage() {
    eprintln!("comm <file1> <file2>");
    eprintln!("Compares file1 and file2 line by line.");
}

fn file_to_lines(path: &String) -> Result<HashSet<String>, std::io::Error> {
    let file1 = File::open(path)?;
    let reader = BufReader::new(file1);
    let mut lines: HashSet<String> = HashSet::<String>::new();
    for line_or_error in reader.lines() {
        let line = line_or_error?;
        lines.insert(line);
    }

    Ok(lines)
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 3 {
        eprintln!("Invalid number of arguments.");
        eprintln_usage();

        return;
    }

    let file1_path = args.get(1).unwrap();
    let file2_path = args.get(2).unwrap();

    let file1_lines = file_to_lines(file1_path).expect("error while reading file 1");
    let file2_lines = file_to_lines(file2_path).expect("error while reading file 2");

    for line in file1_lines {
        if file2_lines.contains(&line) {
            println!("{}", line);
        }
    }
}
