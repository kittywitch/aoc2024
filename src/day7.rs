use std::io::{BufReader, BufRead};
use std::fs::File;
use std::str::FromStr;
use std::collections::HashMap;
use itertools::{Itertools, repeat_n};

#[derive(Debug, Clone, Copy)]
enum Operations {
    Multiply,
    Divide,
    Add,
    Subtract
}

pub fn day_7(file_reader: BufReader<File>) -> (usize, usize) {
    let mut results: Vec<(usize, Vec<usize>)> = Vec::new();
    for line in file_reader.lines() {
        let real_line = line.unwrap();
        let split_line: Vec<&str> = real_line.split(":").collect();
        let result: usize = split_line[0].parse().unwrap();
        let build_values: Vec<usize> = split_line[1].split_whitespace().map(|val| val.parse().unwrap()).collect();
        results.push((result, build_values));
    }

    fn do_operation(operation: Operations, operand_1: usize, operand_2: usize) -> usize {
        match operation {
            Operations::Multiply => return operand_1 * operand_2,
            Operations::Add => return operand_1 + operand_2,
            Operations::Divide => return operand_1 / operand_2,
            Operations::Subtract => return operand_1 - operand_2,
        }
    }

    fn evaluate_if_true(res: usize, builds: Vec<usize>) -> Option<usize> {
        let allowed_operations = vec![
            Operations::Multiply,
            Operations::Add,
        ].into_iter();

        let positions_between_items = builds.len() - 1;
    
        let permutations: Vec<Vec<Operations>> = repeat_n(allowed_operations, positions_between_items)
            .multi_cartesian_product()
            .collect();

        for permutation in permutations {
            let remains = &builds[1..];
            let potential_result: usize = remains.into_iter().enumerate().fold(builds[0], |acc, (idx, i)| 
                do_operation(permutation[idx], acc, *i)
            );

            if res == potential_result {
                return Some(res)
            }
        }
        return None
    }

    let mut true_results = Vec::new();
    for result in results {
        if let Some(true_result) = evaluate_if_true(result.0, result.1) {
            true_results.push(true_result);
        }
    }

    return (true_results.into_iter().sum(), 0)
}