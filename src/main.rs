use std::io::{BufReader, BufRead};
use std::fs::File;
use std::env;
use std::path::Path;
use std::str::FromStr;

fn load_data_file(day: &str) -> BufReader<File> {
    let current_dir = env::current_dir().expect("Current directory is invalid or cannot be accessed").into_os_string();
    let file_path = format!("data/day_{}", day);
    let data_dir = Path::new(&current_dir).join(file_path);
    BufReader::new(File::open(data_dir).expect(format!("Cannot open day {} data file", day).as_str()))
}

fn day_1(file_reader: BufReader<File>) -> (u32, u32) {
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

fn day_2(file_reader: BufReader<File>) -> usize {
    let mut reports = Vec::new();
    for line in file_reader.lines() {
        let levels: Vec<i32> = line.unwrap().split_whitespace().map(|word| i32::from_str(word).unwrap()).collect();
        reports.push(levels);
    }

    // Part 1
    let mut safe = 0;
    for report in reports {
        let abs_differences: Vec<u32> = report.windows(2).map(|window|
            window[0].abs_diff(window[1])
        ).collect();
        let differences: Vec<i32> = report.windows(2).map(|window|
            window[0] - window[1]
        ).collect();
        let safe_1 = abs_differences.iter().all(|x| *x <= 3 && *x >= 1);
        let mut safe_2: Vec<i32> = differences.iter().map(|x| x.signum()).collect();
        safe_2.sort();
        safe_2.dedup();
        let safe_3 = safe_2.len() == 1;
        if safe_3 && safe_1 {
            safe += 1
        };
    }

    return safe
}

fn main() {
    let file_reader_1_test = load_data_file("1_test");
    let (day_1_test_part_1, day_1_test_part_2) = day_1(file_reader_1_test);
    assert!(day_1_test_part_1 == 11);
    assert!(day_1_test_part_2 == 31);

    let file_reader_1 = load_data_file("1");
    day_1(file_reader_1);
    
    let file_reader_2_test = load_data_file("2_test");
    let day_2_test = day_2(file_reader_2_test);
    assert!(day_2_test == 2);

    let file_reader_2 = load_data_file("2");
    day_2(file_reader_2);
}
