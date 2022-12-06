use std::collections::HashSet;

use anyhow::{Result};
use itertools::Itertools;

fn get_substring(string_vec: Vec<char>, substring_len: usize) -> usize {
    let str_len = string_vec.len() - substring_len;
    let str_iter = 0..str_len;
    let mut message_start = 0;
    for  i in str_iter {
        let buffer = string_vec[i..i+substring_len].to_owned();
        let buffer_set: HashSet<char> = HashSet::from_iter(buffer.iter().cloned());
        if buffer_set.len() == substring_len {
            message_start += i + substring_len;
            break
        }
    }
    message_start
}

pub fn day_6_part_1() -> Result<()> {
    let input: Vec<char> = include_str!("../data/day_6").chars().collect();
    let message_start = get_substring(input, 4);
    println!("Day 6, part 1: {message_start}");
    Ok(())
}

pub fn day_6_part_2() -> Result<()> {
    let input: Vec<char> = include_str!("../data/day_6").chars().collect();
    let message_start = get_substring(input, 14);
    println!("Day 6, part 1: {message_start}");
    Ok(())
}