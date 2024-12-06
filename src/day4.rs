use std::io::{BufReader, BufRead};
use std::fs::File;
use std::slice::Iter;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Copy,Clone,Debug,PartialEq)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 8] = [Direction::North, Direction::NorthEast, Direction::East, Direction::SouthEast, Direction::South, Direction::SouthWest, Direction::West, Direction::NorthWest];
        DIRECTIONS.iter()
    }

    pub fn opposite(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            Direction::SouthEast => Direction::NorthWest,
            Direction::NorthEast => Direction::SouthWest,
            Direction::NorthWest => Direction::SouthEast,
            Direction::SouthWest => Direction::NorthEast,
        }
    }
}

#[derive(Copy,Clone,Debug,PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn move_dir(self, dir: Direction, max_x: usize, max_y: usize) -> Option<Coordinate> {
        let new_coord = match dir {
            Direction::North => Coordinate {
                x: self.x,
                y: self.y.checked_sub(1)?,
            },
            Direction::South => Coordinate {
                x: self.x,
                y: self.y.checked_add(1)?,
            },
            Direction::East => Coordinate {
                x: self.x.checked_add(1)?,
                y: self.y,
            },
            Direction::West => Coordinate {
                x: self.x.checked_sub(1)?,
                y: self.y,
            },
            Direction::NorthEast => Coordinate {
                x: self.x.checked_add(1)?,
                y: self.y.checked_sub(1)?,
            },
            Direction::SouthEast => Coordinate {
                x: self.x.checked_add(1)?,
                y: self.y.checked_add(1)?,
            },
            Direction::NorthWest => Coordinate {
                x: self.x.checked_sub(1)?,
                y: self.y.checked_sub(1)?,
            },
            Direction::SouthWest => Coordinate {
                x: self.x.checked_sub(1)?,
                y: self.y.checked_add(1)?,
            },
        };

        if new_coord.x < max_x && new_coord.y < max_y {
            Some(new_coord)
        } else {
            None
        }
    }
}





struct WordSearcher<'a> {
    current_pos: Coordinate,
    cursor: Coordinate,
    direction: Option<Direction>,
    partial: Option<&'a str>,
    matrix: Vec<Vec<char>>,
}

#[derive(Copy,Clone,Debug,PartialEq)]
struct Result {
    start_pos: Coordinate,
    end_pos: Coordinate,
}

impl WordSearcher<'_> {
    fn new(matrix: Vec<Vec<char>>) -> Self {
        let myself = Self { current_pos: Coordinate { x: 0, y: 0 }, cursor: Coordinate { x: 0, y: 0 }, direction: None, partial: None, matrix: matrix };
        return myself
    }

    fn begin(&mut self) -> Vec<Result> {
        let matrix_height: usize = self.matrix.len().try_into().unwrap(); 
        let matrix_width: usize = self.matrix[0].len().try_into().unwrap();

        let mut results: Vec<Result> = Vec::new();
        
        for (x,line) in self.matrix.iter().enumerate() {
            for (y,char) in line.iter().enumerate() {
                self.current_pos = Coordinate { x: x, y: y };

                for dir in Direction::iterator() {
                    self.cursor = Coordinate { x: x, y: y };
                    
                    self.direction = Some(*dir);
                    let mut partial_string = "XMAS".to_string();

                    while !partial_string.is_empty() {
                        let char_to_find = partial_string.chars().next().unwrap();
                        if char_to_find == self.matrix[self.cursor.x][self.cursor.y] {
                            partial_string.remove(0);
                            let new_cursor = self.cursor.move_dir(*dir, matrix_height, matrix_width);
                            if new_cursor.is_some() {
                                self.cursor = new_cursor.unwrap();
                            }
                        } else {
                            break
                        }
                    };
                    if partial_string.is_empty() {
                        results.push(Result {start_pos: self.current_pos, end_pos: self.cursor});
                    } 
                }
            }
        }
        results
    }
}

pub fn day_4(file_reader: BufReader<File>) -> (usize, u32) {
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in file_reader.lines() {
        let mut matrix_line: Vec<char> = Vec::new();
        for character in line.expect("Line's empty").chars() {
            matrix_line.push(character)
        }
        matrix.push(matrix_line);
    }
    let mut wordsearcher = WordSearcher::new(matrix); 
    let mut result = wordsearcher.begin();
    dbg!(&result);
    return (result.len(),0)
}