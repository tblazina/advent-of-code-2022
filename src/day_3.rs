use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
fn create_priority_map() -> HashMap<char, i32> {
    let mut map = HashMap::<char, i32>::new();
    for i in 0..26 {
        let letter = (b'a' + i) as char;
        map.insert(letter, i as i32 + 1);

        let letter = (b'A' + i) as char;
        map.insert(letter, i as i32 + 27);
    }
    map
}

fn read_lines(path: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);
    Ok(reader.lines())
}

pub fn day_3_part_1() -> Result<(), Box<dyn Error>> {
    let priority_map = create_priority_map();
    let mut score = 0;
    if let Ok(lines) = read_lines("./data/day_3") {
        for line in lines {
            if let Ok(line) = line {
                let length = line.len();
                let midpoint = length / 2;
                let compartment_1: HashSet<char> = line.as_str()[0..midpoint].chars().collect();
                let compartment_2: HashSet<char>= line.as_str()[midpoint..length.to_owned()].chars().collect();
                let intersection = compartment_1.intersection(&compartment_2);
                let priority_key = intersection.map(|i| *i).collect::<Vec<char>>();
                if priority_key.len() > 0 {
                    let priority = priority_map.get(&priority_key[0]);
                    score += priority.unwrap();
                }
                else if priority_key.len() > 1 {
                    dbg!(priority_key);
                }
                else {
                    println!("WTF");
                    println!("{}", line);
                    println!("{}", line.as_str()[1..midpoint].to_string());
                    println!("{}", line.as_str()[midpoint..length.to_owned()].to_string());

                }
            }
        }
    }
   println!("Day 3, part 1: {}", score);
   Ok(())
}

pub fn day_3_part_2() -> Result<(), Box<dyn Error>> {
    let priority_map = create_priority_map();
    let mut score = 0;
    let contents = std::fs::read_to_string("./data/day_3").expect("could not read files");
    let lines: Vec<&str> = contents.lines().collect();
    for chunk in lines.chunks(3) {
        let line_1: HashSet<char> = chunk[0].chars().collect();
        let line_2: HashSet<char> = chunk[1].chars().collect();
        let line_3: HashSet<char> = chunk[2].chars().collect();
        let intersection_1 = line_1.intersection(&line_2).cloned().collect();
        let common_intersection = line_3.intersection(&intersection_1);
        let common_char: Vec<char> = common_intersection.map(|i| *i).collect();
        let badge_char = &common_char[0];
        let value = priority_map.get(badge_char).unwrap();
        score += value;
       }
    println!("Day 3, part 2: {}", score);
    Ok(())
}