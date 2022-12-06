#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_must_use))]
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
use day_1::day_1;
use day_2::day_2;
use day_3::day_3_part_2 as day_3;
use day_4::{day_4_part_2 as day_4, parse_input};
use day_5::day_5_part_2 as day_5;
use day_6::day_6_part_2 as day_6;

fn main() {
    // day_1().expect("to be correct")
    day_6();
}
