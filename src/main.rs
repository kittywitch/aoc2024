use std::io::{BufRead, BufReader};
use std::fs::File;
use std::env;
use std::path::Path;
use std::str::FromStr;

fn day_1() {
    let mut current_dir = env::current_dir().expect("Current directory is invalid or cannot be accessed").into_os_string();
    let file_path = "data/day_1";
    let data_dir = Path::new(&current_dir).join(file_path);
    
    let file_reader = BufReader::new(File::open(data_dir).expect("Cannot open day 1 data file"));

    let mut list_1 = Vec::new(); 
    let mut list_2 = Vec::new(); 

    for line in file_reader.lines() {
        let words: Vec<u32> = line.unwrap().split_whitespace().map(|word| u32::from_str(word).unwrap()).collect();
        list_1.push(words[0]);
        list_2.push(words[1]);
    }

    list_1.sort();
    list_2.sort();

    let mut uniques = list_1.clone();
    uniques.dedup();
    let zipped_list = list_1.iter().zip(list_2.iter());

    // Part 1 
    let mut total_diff: u32 = 0;
    for (a, b) in zipped_list {
        total_diff += a.abs_diff(*b); 
    }
    println!("{}", total_diff);

    // Part 2
    let mut total_similarity: u32 = 0;
    for unique in uniques {
        let occurences: u32 = list_2.iter().filter(|&item| *item == unique).count().try_into().unwrap();
        total_similarity += unique * occurences
    }

    println!("{}", total_similarity);
}

fn main() {
    day_1();
}