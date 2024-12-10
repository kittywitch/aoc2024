use std::io::{BufReader, BufRead};
use std::fs::File;
use std::str::FromStr;
use std::collections::{HashMap, HashSet};
use std::slice::Iter;
use itertools::{Itertools, repeat_n};

#[derive(Copy,Clone,Debug,PartialEq, Eq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

pub fn day_8(file_reader: BufReader<File>) -> (usize, usize) {
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in file_reader.lines() {
        let mut matrix_line: Vec<char> = Vec::new();
        for character in line.expect("Line's empty").chars() {
            matrix_line.push(character)
        }
        matrix.push(matrix_line);
    }
    
    let matrix_height: usize = matrix.len().try_into().unwrap(); 
    let matrix_width: usize = matrix[0].len().try_into().unwrap();

    let mut antennas: HashMap<String, Vec<Coordinate>> = HashMap::new();
    let mut typeless_antennas: Vec<Coordinate> = Vec::new();

    for (i, row) in matrix.clone().into_iter().enumerate() {
        for (j, column) in row.into_iter().enumerate() {
            match column {
                '.' => {
                },
                _ => {
                    let loc = Coordinate {x: i, y: j};
                    if antennas.contains_key(&column.to_string()) {
                        antennas.get_mut(&column.to_string()).unwrap().push(loc);
                    } else {
                        antennas.insert(column.to_string(), vec![loc]);
                    }
                    typeless_antennas.push(loc);
                },
            }
        }
    }

    let processed: HashSet<Coordinate> = HashSet::new();
    let mut antinodes: HashSet<Coordinate> = HashSet::new();
    for (antenna_type, locations) in antennas.clone().into_iter() {
        let antenna_pairs: Vec<Vec<Coordinate>> = repeat_n(locations.into_iter(), 2)
            .multi_cartesian_product()
            .dedup()
            .collect();

        for antenna_pair in antenna_pairs {
            let antenna_0_x = isize::try_from(antenna_pair[0].x).ok().unwrap();
            let antenna_0_y = isize::try_from(antenna_pair[0].y).ok().unwrap();
            let antenna_1_x = isize::try_from(antenna_pair[1].x).ok().unwrap();
            let antenna_1_y = isize::try_from(antenna_pair[1].y).ok().unwrap();

            let distance_x = antenna_1_x - antenna_0_x;
            let distance_y = antenna_1_y - antenna_0_y;

            let antinode_0_x = antenna_0_x - distance_x;
            let antinode_0_y = antenna_0_y - distance_y;
            let antinode_1_x = antenna_1_x + distance_x;
            let antinode_1_y = antenna_1_y + distance_y;

            let antinode_0_x_opt = usize::try_from(antinode_0_x);
            let antinode_0_y_opt = usize::try_from(antinode_0_y);
            let antinode_1_x_opt = usize::try_from(antinode_1_x);
            let antinode_1_y_opt = usize::try_from(antinode_1_y);

            if antinode_0_x_opt.is_ok() && antinode_0_x_opt.unwrap() < matrix_width &&
                antinode_0_y_opt.is_ok() && antinode_0_y_opt.unwrap() < matrix_height
                 && antinode_0_x_opt.unwrap() != antenna_pair[0].x && antinode_0_y_opt.unwrap() != antenna_pair[0].y {
                dbg!(&antinode_0_x_opt, &antinode_0_y_opt);
                antinodes.insert(Coordinate { x: antinode_0_x_opt.unwrap(), y: antinode_0_y_opt.unwrap() });
            }
            if antinode_1_x_opt.is_ok() && antinode_1_x_opt.unwrap() < matrix_width
                && antinode_1_y_opt.is_ok() && antinode_1_y_opt.unwrap() < matrix_height
                 && antinode_1_x_opt.unwrap() != antenna_pair[1].x && antinode_1_y_opt.unwrap() != antenna_pair[1].y{
                dbg!(&antinode_1_x_opt, &antinode_1_y_opt);
                antinodes.insert(Coordinate { x: antinode_1_x_opt.unwrap(), y: antinode_1_y_opt.unwrap() });
            }
        }
    }

    let mut new_matrix = matrix.clone();
    for antinode in &antinodes {
        new_matrix[antinode.x][antinode.y] = '#';
    }

    for column in new_matrix {
        print!("{}\n", column.into_iter().collect::<String>());
    }

    return (antinodes.len(), 0)
}