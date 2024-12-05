use std::io::{BufReader, BufRead};
use std::fs::File;
use std::str::FromStr;

fn safe_layer_1(i: i32, j: i32) -> (bool, i32) {
    let difference = i-j;
    let abs_difference = difference.abs();
    let difference_check = abs_difference <= 3 && abs_difference >= 1;
    let signum = difference.signum();
    return (difference_check, signum)
}

fn safety_check(levels: &Vec<i32>, accepted_failures: usize) -> bool {
    // figure out the difference now, also provide the sign of the difference
    let differences: Vec<(usize,(bool,i32))> = levels.windows(2).enumerate().map(|(index, window)|
        (index, safe_layer_1(window[0], window[1]))
    ).collect();

    // figure out the window difference logic, make sure the sign between the current window and previous window are the same, if they're not you have an unsafe entry
    let directions: Vec<(usize, (bool, bool))> = differences.windows(2).map(|window| {
        let (prev_index, (prev_diff, prev_sign)) = window[0];
        let (cur_index, (cur_diff, cur_sign)) = window[1];
        (cur_index, (cur_diff && prev_diff, prev_sign == cur_sign))   
    }).collect();
    
    // for part 1, the result is this
    let result: Vec<(&usize,  bool)> = directions.iter().map(|(index, (diff, sign))|
        (index, (*diff && *sign))
    ).collect();

    // count the failures
    let failures = result.iter().filter(|(index, res)|
            !*res
        ).count();

    // if there are no failures, go ahead. if we also accept no failures, return here.
    // this is our base case under a recursive paradigm, or what occurs in part 1.
    if failures == 0 || accepted_failures == 0 {
        result.iter().filter(|(index, res)|
            !*res
        ).count() == 0 
    } else { 
        // take a copy of the levels so that we can remove the faulty index from itt
        let mut copied_levels = levels.clone();
        let mut diffs = 1000;
        let mut signs = 1000;
        // find the first element that fails the difference magnitude check
        let diffs_check = differences.iter().find(|(index, (diff, sign))|
            !*diff
        );
        // turn that Option into an index, plus increase it by 1 because that targets the correct index based upon the window position
        if let Some((index, _)) = diffs_check { diffs = index+1; }
        // find the first element that fails the difference sign check
        let signs_check = directions.iter().find(|(index, (diff, sign))|           
            !*sign
        );
        // turn that Option into an index, plus increase it by 1 because that targets the correct index based upon the window position
        if let Some((index, _)) = signs_check { signs = index+1; }
        // the index to remove is the smaller of the two
        let index_to_remove = diffs.min(signs);
        dbg!(failures, diffs, signs, index_to_remove);
        dbg!(&levels);
        // remove that index
        copied_levels.remove(index_to_remove);
        dbg!(&copied_levels);
        // run the safety check again, this time with one less accepted failure since we've removed that failure
        let fixed = safety_check(&copied_levels, accepted_failures-1);
        dbg!(fixed);
        fixed
    }
}

pub fn day_2(file_reader: BufReader<File>) -> (usize, usize) {
    let mut reports = Vec::new();
    for line in file_reader.lines() {
        let levels: Vec<i32> = line.unwrap().split_whitespace().map(|word| i32::from_str(word).unwrap()).collect();
        reports.push(levels);
    }
 
    let mut safes = Vec::new();

    for report in &reports {
        safes.push(safety_check(report, 0));
    }

    let part_1_result = safes.iter().filter(|b| **b).count();
    
    let mut safes_p2 = Vec::new();
    
    for (i, report) in reports.iter().enumerate() {
        println!("Report {}", i+1);
        safes_p2.push(safety_check(report, 1));
    }

    let part_2_result = safes_p2.iter().filter(|b| **b).count();

    println!("{}", part_1_result);
    println!("{}", part_2_result);
    return (part_1_result, part_2_result)
}