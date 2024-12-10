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

pub fn propagation(antennas: HashMap<String, Vec<Coordinate>>, matrix_width: usize, matrix_height: usize, propagate: bool) -> HashSet<Coordinate> {
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
            let mut distance_x: isize = 0;
            let mut distance_y: isize = 0;
            distance_x = antenna_1_x - antenna_0_x;
            distance_y = antenna_1_y - antenna_0_y;
            let mut antinode_0_x = antenna_0_x - distance_x;
            let mut antinode_0_y = antenna_0_y - distance_y;
            let mut antinode_1_x = antenna_1_x + distance_x;
            let mut antinode_1_y = antenna_1_y + distance_y;
            let mut antinode_0_x_opt = usize::try_from(antinode_0_x);
            let mut antinode_0_y_opt = usize::try_from(antinode_0_y);
            let mut antinode_1_x_opt = usize::try_from(antinode_1_x);
            let mut antinode_1_y_opt = usize::try_from(antinode_1_y);
            if propagate {
                while antinode_0_x_opt.is_ok() && antinode_0_x_opt.unwrap() < matrix_width &&
                    antinode_0_y_opt.is_ok() && antinode_0_y_opt.unwrap() < matrix_height {
                    antinodes.insert(Coordinate { x: antinode_0_x_opt.unwrap(), y: antinode_0_y_opt.unwrap() });
                        if antinode_0_x_opt.unwrap() == antenna_pair[1].x || antinode_0_y_opt.unwrap() == antenna_pair[1].y {
                            break
                        }

                    antinode_0_x -= distance_x;
                    antinode_0_y -= distance_y;

                    antinode_0_x_opt = usize::try_from(antinode_0_x);
                    antinode_0_y_opt = usize::try_from(antinode_0_y);
                }
                while antinode_1_x_opt.is_ok() && antinode_1_x_opt.unwrap() < matrix_width
                    && antinode_1_y_opt.is_ok() && antinode_1_y_opt.unwrap() < matrix_height {
                    antinodes.insert(Coordinate { x: antinode_1_x_opt.unwrap(), y: antinode_1_y_opt.unwrap() });
                        if antinode_1_x_opt.unwrap() == antenna_pair[0].x || antinode_1_y_opt.unwrap() == antenna_pair[0].y {
                            break
                        }
                    
                    antinode_1_x -= distance_x;
                    antinode_1_y -= distance_y;

                    antinode_1_x_opt = usize::try_from(antinode_1_x);
                    antinode_1_y_opt = usize::try_from(antinode_1_y);
                }

            } else {
                if antinode_0_x_opt.is_ok() && antinode_0_x_opt.unwrap() < matrix_width &&
                    antinode_0_y_opt.is_ok() && antinode_0_y_opt.unwrap() < matrix_height
                    && antinode_0_x_opt.unwrap() != antenna_pair[0].x && antinode_0_y_opt.unwrap() != antenna_pair[0].y {
                    antinodes.insert(Coordinate { x: antinode_0_x_opt.unwrap(), y: antinode_0_y_opt.unwrap() });
                }
                if antinode_1_x_opt.is_ok() && antinode_1_x_opt.unwrap() < matrix_width
                    && antinode_1_y_opt.is_ok() && antinode_1_y_opt.unwrap() < matrix_height
                    && antinode_1_x_opt.unwrap() != antenna_pair[1].x && antinode_1_y_opt.unwrap() != antenna_pair[1].y{
                    antinodes.insert(Coordinate { x: antinode_1_x_opt.unwrap(), y: antinode_1_y_opt.unwrap() });
                }
            }
        }
    }
    return antinodes
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


    let antinodes_p1 = propagation(antennas.clone(), matrix_width, matrix_height, false);
    let mut new_matrix = matrix.clone();
    for antinode in &antinodes_p1 {
        new_matrix[antinode.x][antinode.y] = '#';
    }
    for column in new_matrix {
        println!("{}", column.into_iter().collect::<String>());
    }
    println!("");
    
    let antinodes_p2 = propagation(antennas, matrix_width, matrix_height, true);
    let mut new_matrix_p2 = matrix.clone();
    for antinode in &antinodes_p2 {
        new_matrix_p2[antinode.x][antinode.y] = '#';
    }
    for column in new_matrix_p2 {
        println!("{}", column.into_iter().collect::<String>());
    }
    println!("");


    return (antinodes_p1.len(), antinodes_p2.len())
}