mod helpers;
use helpers::load_data_file;

mod day1;
mod day2;

use day1::day_1;
use day2::day_2;

fn main() {
    let file_reader_1_test = load_data_file("1_test");
    let (day_1_test_part_1, day_1_test_part_2) = day_1(file_reader_1_test);
    assert!(day_1_test_part_1 == 11);
    assert!(day_1_test_part_2 == 31);

    let file_reader_1 = load_data_file("1");
    day_1(file_reader_1);
    
    let file_reader_2_test = load_data_file("2_test");
    let (day_2_test_part_1, day_2_test_part_2) = day_2(file_reader_2_test);
    assert!(day_2_test_part_1 == 2);
    assert!(day_2_test_part_2 == 4);

    let file_reader_2 = load_data_file("2");
    day_2(file_reader_2);
}
