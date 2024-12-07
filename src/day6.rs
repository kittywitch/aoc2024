use std::io::{BufReader, BufRead};
use std::fs::File;
use std::str::FromStr;
use std::collections::{HashMap, HashSet};
use std::slice::Iter;

#[derive(Copy,Clone,Debug,PartialEq, Hash, Eq)]
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

pub fn pathing2(objects: HashMap<Coordinate, String>, matrix_height: usize, matrix_width: usize) -> usize {
    let mut player: Option<Coordinate> = None;
    let mut direction: Option<Direction> = None;
    for (key, value) in objects.clone().into_iter() {
        if value == "^" {
            player = Some(key);
            direction = Some(Direction::North);
            println!("Player starts at {:?}", player);
        }
    }

    let mut checkpoint: (Option<Direction>, Option<Coordinate>) = (direction, player);
    let mut loop_hits = 0;
    let mut tiles = pathing(objects.clone(), matrix_height, matrix_width);
    let tiles_set: HashSet<Coordinate> = HashSet::from_iter(tiles.iter().map(|x| x.0));

    'outer: for tile in tiles_set {
        let mut objects_copy = objects.clone();
        if !objects_copy.contains_key(&Coordinate {x: tile.x, y: tile.y}) {
            objects_copy.insert(Coordinate { x: tile.x, y: tile.y }, '#'.to_string());
            let loopy = pathing_loopy(objects_copy, matrix_height, matrix_width);
            if loopy {
                loop_hits += 1;
            }
        } else {
        }
    }

    return loop_hits;
}

pub fn pathing_loopy(objects: HashMap<Coordinate, String>, matrix_height: usize, matrix_width: usize) -> bool {
    let mut player: Option<Coordinate> = None;
    let mut direction: Option<Direction> = None;
    let mut visited: HashSet<(Coordinate,Direction)> = HashSet::new();

    for (key, value) in objects.clone().into_iter() {
        if value == "^" {
            player = Some(key);
            direction = Some(Direction::North);
        }
    }

    if player.is_some() && direction.is_some() {
        let mut next_space = None;
        while player.is_some() {
            next_space = player.unwrap().move_dir(direction.unwrap(), matrix_height, matrix_width);
            if !visited.insert((player.unwrap(), direction.unwrap())) {
                return true
            }
            if next_space.is_some() {
                if objects.contains_key(&next_space.unwrap()) {
                    let obj = &objects[&next_space.clone().unwrap()].clone();
                    if obj == "#" {
                        direction = Some(direction.unwrap().rotate());
                        continue
                    }
                }
            }
            player = next_space;
        }
    }

    return false
}

pub fn pathing(objects: HashMap<Coordinate, String>, matrix_height: usize, matrix_width: usize) -> Vec<(Coordinate, Direction)> {
    let mut player: Option<Coordinate> = None;
    let mut direction: Option<Direction> = None;
    for (key, value) in objects.clone().into_iter() {
        if value == "^" {
            player = Some(key);
            direction = Some(Direction::North);
        }
    }

    let mut tiles = Vec::new();
    if player.is_some() && direction.is_some() {
        let mut next_space = None;
        while player.is_some() {
            tiles.push((player.unwrap(), direction.unwrap()));
            next_space = player.unwrap().move_dir(direction.unwrap(), matrix_height, matrix_width);
            if next_space.is_some() && objects.contains_key(&next_space.unwrap()) {
               let obj = &objects[&next_space.clone().unwrap()].clone();
               if obj == "#" {
                direction = Some(direction.unwrap().rotate());
                continue
               }
            }
            player = next_space;
        }
    }

    return tiles
}

pub fn day_6(file_reader: BufReader<File>) -> (usize, usize) {
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

    let current_obstacles: Vec<Coordinate> = objects.clone().into_iter().filter_map(|(x, y)| match y.as_str() {
        "#" => Some(x),
        _ => None,
    }).collect();

    let tiles = pathing(objects.clone(), matrix_height, matrix_width);

    let tiles_set: HashSet<Coordinate> = HashSet::from_iter(tiles.iter().map(|x| x.0));

    let pathing_mewp = pathing2(objects.clone(), matrix_height, matrix_width);

    dbg!(&pathing_mewp);

    return (tiles_set.len(),pathing_mewp)
}    