use std::io::{prelude::*, BufReader, Error};
use std::fs::File;
use std::vec::Vec;

pub fn day_1() -> Result<(), Error> {
    let f = File::open("./data/day_1")?;
    let buf = BufReader::new(f);
    let mut sums: Vec<i32> = Vec::new();
    let mut running_sum = 0;
    let lines = buf.lines();
    for line in lines {
        match line.unwrap().parse::<i32>() {
            Ok(line) => {
                running_sum += line;
            },
            Err(_) => {
                sums.push(running_sum);
                running_sum = 0;
                continue
            },
        };
    }
    // Part 1 answer
    println!("Answer to part 1: {}", sums.iter().max().unwrap());

    //Part 2 answer
    sums.sort();
    sums.reverse();
    // Part 2 answer
    println!("Answer to part 2: {}", sums[0..3].iter().sum::<i32>());
    Ok(())
}