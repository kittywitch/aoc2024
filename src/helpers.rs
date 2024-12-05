use std::io::{BufReader, BufRead};
use std::fs::File;
use std::env;
use std::path::Path;
use std::str::FromStr;

pub fn load_data_file(day: &str) -> BufReader<File> {
    let current_dir = env::current_dir().expect("Current directory is invalid or cannot be accessed").into_os_string();
    let file_path = format!("data/day_{}", day);
    let data_dir = Path::new(&current_dir).join(file_path);
    BufReader::new(File::open(data_dir).expect(format!("Cannot open day {} data file", day).as_str()))
}