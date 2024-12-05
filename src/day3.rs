use std::io::{BufReader, BufRead};
use std::fs::File;
use std::str::FromStr;
use nom::IResult;
use nom::Parser;
use nom::character::complete::{digit1,anychar};
use nom::bytes::complete::{take_while,tag,is_not, take};
use nom::multi::many0;
use nom::branch::alt;
use nom::sequence::{tuple,preceded,separated_pair,terminated,delimited};
use nom::combinator::map;

#[derive(Debug)]
enum Token {
    Do,
    Dont,
    Mult(u32, u32),
}

fn lexer(input: &str) -> IResult<&str, Vec<Option<Token>>>{
    let pair = separated_pair(digit1, tag(","), digit1);
    let mut proper_tag = delimited(tag("mul("), pair, tag(")"));
    (many0(alt(
        (
        map(proper_tag, |(m1, m2)| Some(Token::Mult(u32::from_str(m1).unwrap(), u32::from_str(m2).unwrap()))),
        map(tag("don't()"), |_| Some(Token::Dont)),
        map(tag("do()"), |_| Some(Token::Do)),
        map(anychar, |x| None),
        )
    )
    )).parse(input)
}

fn parser(input: &str, enable_try: bool) -> u32 {
    let (input, mult_tuples) = lexer(&input).unwrap();
    mult_tuples.into_iter().filter_map(|x| x).scan((true, 0), |(st, total), elem|
        match elem {
            Token::Do => {
                *st = true;
                Some(*total)
            },
            Token::Dont => {
                *st = false;
                Some(*total)
            },
            Token::Mult(m1, m2) => {
                if *st || !enable_try {
                    *total += m1*m2;
                }
                Some(*total)
            },
        }
    ).last().unwrap()
}
pub fn day_3(file_reader: BufReader<File>) -> (u32, u32) {
    let mut line_1 = String::new();
    for line in file_reader.lines() {
        line_1.push_str(&line.unwrap());
    }
    let part_1 = parser(&line_1, false);
    let part_2 = parser(&line_1, true);
    dbg!(part_1);
    dbg!(part_2);
    return (part_1, part_2)
}