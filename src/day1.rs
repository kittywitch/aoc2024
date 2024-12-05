use std::io::{BufReader, BufRead};
use std::fs::File;
use std::str::FromStr;

pub fn day_1(file_reader: BufReader<File>) -> (u32, u32) {
    let mut list_1 = Vec::new(); 
    let mut list_2 = Vec::new(); 

    for line in file_reader.lines() {
        let words: Vec<u32> = line.unwrap().split_whitespace().map(|word| u32::from_str(word).unwrap()).collect();
        list_1.push(words[0]);
        list_2.push(words[1]);
    }

    list_1.sort();
    list_2.sort();

    let zipped_list = list_1.iter().zip(list_2.iter());

    // Part 1 
    let total_diff: u32 = zipped_list.map(|item| item.0.abs_diff(*item.1)).sum();
    println!("{}", total_diff);

    // Part 2
    let uniques = list_1.clone();
    let total_similarity: u32 = uniques.iter().map(|unique| unique * <usize as TryInto<u32>>::try_into(list_2.iter().filter(
            |&item| *item == *unique
        ).count()).unwrap()
    ).sum();
    println!("{}", total_similarity);

    return (total_diff, total_similarity)
}