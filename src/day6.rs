use std::io::{BufReader, BufRead};
use std::fs::File;
use std::str::FromStr;
use std::collections::{HashMap, HashSet};
use std::slice::Iter;

#[derive(Copy,Clone,Debug,PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [Direction::North, Direction::East, Direction::South, Direction::West];
        DIRECTIONS.iter()
    }
    

    pub fn rotate(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Copy,Clone,Debug,PartialEq, Eq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn move_dir(self, dir: Direction, max_x: usize, max_y: usize) -> Option<Coordinate> {
        let new_coord = match dir {
            Direction::North => Coordinate {
                x: self.x.checked_sub(1)?,
                y: self.y,
            },
            Direction::South => Coordinate {
                x: self.x.checked_add(1)?,
                y: self.y,
            },
            Direction::East => Coordinate {
                x: self.x,
                y: self.y.checked_add(1)?,
            },
            Direction::West => Coordinate {
                x: self.x,
                y: self.y.checked_sub(1)?,
            },
        };

        if new_coord.x < max_x && new_coord.y < max_y {
            Some(new_coord)
        } else {
            None
        }
    }
}


pub fn day_6(file_reader: BufReader<File>) -> (usize, u32) {
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

    let mut objects: HashMap<Coordinate, String> = HashMap::new();
    for (i, row) in matrix.into_iter().enumerate() {
        for (j, column) in row.into_iter().enumerate() {
            match column {
                '#' => {
                    objects.insert(Coordinate { x: i, y: j }, column.to_string());
                },
                '^' => {
                    objects.insert(Coordinate { x: i, y: j }, column.to_string());
                },
                _ => {

                },
            }
        }
    }

    let mut player: Option<Coordinate> = None;
    let mut direction: Option<Direction> = None;
    for (key, value) in objects.clone().into_iter() {
        if value == "^" {
            player = Some(key);
            direction = Some(Direction::North);
        } else if value == ">" {
            player = Some(key);
            direction = Some(Direction::East);
        } else if value == "<" {
            player = Some(key);
            direction = Some(Direction::West);
        } else if value == "v" {
            player = Some(key);
            direction = Some(Direction::South);
        }
    }

    let mut tiles = HashSet::new();
    if player.is_some() && direction.is_some() {
        dbg!(player, direction);
        let mut next_space = None;
        while player.is_some() {
            tiles.insert(player);
            next_space = player.unwrap().move_dir(direction.unwrap(), matrix_height, matrix_width);
            if next_space.is_some() && objects.contains_key(&next_space.unwrap()) {
               let obj = &objects[&next_space.unwrap()].clone();
               if obj == "#" {
                direction = Some(direction.unwrap().rotate());
                continue
               }
            }
            player = next_space;
        }
    }

    return (tiles.len(),0)
}    