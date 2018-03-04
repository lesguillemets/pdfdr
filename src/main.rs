use std::process::Command;
use std::env::args;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Write};
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let files_to_read = args().nth(1).expect("arg");
    let file_to_write = args().nth(2).expect("arg2");
    let prefix = args().nth(3).unwrap_or("".to_string());
    let mut bwriter = BufWriter::new(
        OpenOptions::new()
            .append(true)
            .create(true)
            .open(file_to_write)
            .unwrap(),
    );
    for file in read_files(&files_to_read).iter().filter(|p| is_photo(p)) {
        match get_date_taken(&file) {
            None => println!("EEEE Error on {}", &file),
            Some(d) => {
                // because the result from date_taken includes newline at the end
                print!("{} : {}", get_base_name(&file), d);
                bwriter.write_fmt(format_args!("{},{},{}", get_base_name(&file), &file, d));
            }
        }
    }
}

fn get_date_taken(file: &str) -> Option<String> {
    let output = Command::new("exiftool")
        .arg("-datetimeoriginal")
        .arg("-s3")
        .args(&["-d", "%s,%Y:%m:%d-%H:%M:%S"]) // %t for \t is somehow not working
        .arg(file)
        .output()
        .expect("failed");
    if output.status.success() {
        Some(check_empty_entry(
            String::from_utf8(output.stdout).expect("Non valid utf!?"),
        ))
    } else {
        None
    }
}

fn check_empty_entry(ent: String) -> String {
    if ent.is_empty() {
        String::from(",")
    } else {
        ent
    }
}
fn read_files(fname: &str) -> Vec<String> {
    let f = File::open(fname).expect("file not found");
    let reader = BufReader::new(f);
    reader
        .lines()
        .map(|l| l.expect(&format!("unreadable file {}", fname)))
        .collect()
}

fn is_photo(fname: &str) -> bool {
    fname.ends_with(".JPG")
}

fn get_base_name(f: &str) -> &str {
    Path::new(f)
        .file_name()
        .expect("file_name")
        .to_str()
        .expect("non_utf")
}
