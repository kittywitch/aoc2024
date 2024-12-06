mod helpers;
use helpers::load_data_file;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

use day1::day_1;
use day2::day_2;
use day3::day_3;
use day4::day_4;
use day5::day_5;

fn main() {
    let day = 5;

    match day {
        1 => {
            let file_reader_1_test = load_data_file("1_test");
            let (day_1_test_part_1, day_1_test_part_2) = day_1(file_reader_1_test);
            println!("1_test: {} {}", day_1_test_part_1, day_1_test_part_2);
            assert!(day_1_test_part_1 == 11);
            assert!(day_1_test_part_2 == 31);

            let file_reader_1 = load_data_file("1");
            let (day_1_part_1, day_1_part_2) = day_1(file_reader_1);
            
            println!("1: {} {}", day_1_part_1, day_1_part_2)
        },
        2 => {
            let file_reader_2_test = load_data_file("2_test");
            let (day_2_test_part_1, day_2_test_part_2) = day_2(file_reader_2_test);
            println!("2_test: {} {}", day_2_test_part_1, day_2_test_part_2);
            assert!(day_2_test_part_1 == 2);
            assert!(day_2_test_part_2 == 4);

            let file_reader_2 = load_data_file("2");
            let (day_2_part_1, day_2_part_2) = day_2(file_reader_2);
            
            println!("2: {} {}", day_2_part_1, day_2_part_2)

        },
        3 => {
            let file_reader_3_test = load_data_file("3_test");
            let (day_3_test_part_1, day_3_test_part_2) = day_3(file_reader_3_test);
            println!("3_test: {} {}", day_3_test_part_1, day_3_test_part_2);
            assert!(day_3_test_part_1 == 161); 
            
            let file_reader_3 = load_data_file("3");
            let (day_3_part_1, day_3_part_2) = day_3(file_reader_3);

            println!("3: {} {}", day_3_part_1, day_3_part_2)
        },
        4 => {
            let file_reader_4_test = load_data_file("4_test");
            let (day_4_test_part_1, day_4_test_part_2) = day_4(file_reader_4_test);
            println!("4_test: {} {}", day_4_test_part_1, day_4_test_part_2);
            assert!(day_4_test_part_1 == 18);

            let file_reader_4 = load_data_file("4");
            let (day_4_part_1, day_4_part_2) = day_4(file_reader_4);
            
            println!("4: {} {}", day_4_part_1, day_4_part_2)
        },
        5 => {
            let file_reader_5_test = load_data_file("5_test");
            let (day_5_test_part_1, day_5_test_part_2) = day_5(file_reader_5_test);
            println!("5_test: {} {}", day_5_test_part_1, day_5_test_part_2);
            
            let file_reader_5 = load_data_file("5");
            let (day_5_part_1, day_5_part_2) = day_5(file_reader_5);
            
            println!("5: {} {}", day_5_part_1, day_5_part_2)
        },
        _ => {
            todo!()
        },
    }
}
