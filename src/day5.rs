use std::io::{BufReader, BufRead};
use std::fs::File;
use std::str::FromStr;
use std::collections::{HashSet,HashMap};
use std::cmp::Ordering;

pub fn splitter(line: String, split_char: &str) -> Vec<String> {
    line.split(split_char).map(String::from).collect()
}

pub fn day_5(file_reader: BufReader<File>) -> (u32, u32) {
    let mut lines = Vec::new();

    for line in file_reader.lines() {
        lines.push(line.unwrap());        
    }
    
    let mut order_rules = Vec::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();


    for line in lines {
        if line.contains("|") {
            order_rules.push(splitter(line, "|"));
        } else if line.contains(",") {
            updates.push(splitter(line, ",").into_iter().map(|x| x.parse::<u32>().unwrap()).collect());
        }
    }

    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();
    for numvec in order_rules.iter() {
        let num1: u32 = numvec[0].parse().unwrap();
        let num2: u32 = numvec[1].parse().unwrap();
        if !rules.contains_key(&num2) {
            rules.insert(num2,vec![num1].into_iter().collect());
        } else {
            rules.entry(num2).or_default().insert(num1);
        }
    }; 

    let mut incorrect_updates: Vec<Vec<u32>> = Vec::new();
    for update in &updates {
        'outer: for (i, page) in update.clone().into_iter().enumerate() {
            let prior_pages = &update[0..i];
            for prior_page in prior_pages {
                if rules.contains_key(&prior_page) {
                    if rules[prior_page].contains(&page) {
                        incorrect_updates.push(update.clone());
                        break 'outer; 
                    }
                }
            }
        }
    }

    let correct_updates = updates.into_iter().filter(|item| !incorrect_updates.contains(item));

    let mut middles = Vec::new();
    for update in correct_updates {
        let length = update.len();
        let middle_index = length / 2;
        middles.push(update[middle_index]);
    }

    let mut corrected_updates = Vec::new();

    for update in incorrect_updates {
        let mut corrected_update = update.clone();
        for (i, page) in update.clone().into_iter().enumerate() {
            corrected_update.sort_by(|&a, &b|
                if rules.contains_key(&a) && rules[&a].contains(&b) {
                    Ordering::Less
                } else if rules.contains_key(&b) && rules[&b].contains(&a) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            );
        }
        corrected_updates.push(corrected_update);
    }
    
    let mut corrected_middles = Vec::new();
    for update in corrected_updates {
        let length = update.len();
        let middle_index = length / 2;
        corrected_middles.push(update[middle_index]);
    }

    dbg!(&corrected_middles);
    
    return (middles.into_iter().sum::<u32>(), corrected_middles.into_iter().sum::<u32>())
}