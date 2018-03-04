use std::process::Command;
use std::env::args;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let files_to_read = args().nth(1).expect("arg");
    for file in read_files(&files_to_read).iter().filter(|p| is_photo(p)) {
        match get_date_taken(&file) {
            None => println!("Error on {}", &file),
            Some(d) => println!("{},{},{}", get_base_name(&file), &file, d),
        }
    }
}

fn get_date_taken(file: &str) -> Option<String> {
    let output = Command::new("exiftool")
        .arg("-datetimeoriginal")
        .arg("-s3")
        .args(&["-d", "%s,%Y:%m:%H:%M:%S"]) // %t for \t is somehow not working
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
    reader.lines().map(|l| l.unwrap()).collect()
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
