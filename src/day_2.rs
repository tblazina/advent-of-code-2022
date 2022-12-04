use std::collections::HashMap;
use std::error::Error;
use std::io::{prelude::*, BufReader};

pub fn day_2() -> Result<(), Box<dyn Error>> {
    let f = std::fs::File::open("./data/day_2")?;
    let reader = BufReader::new(&f);
    let score_map = HashMap::from([
        ("A X", 1 + 3),
        ("A Y", 2 + 6),
        ("A Z", 3 + 0),
        ("B X", 1 + 0),
        ("B Y", 2 + 3),
        ("B Z", 3 + 6),
        ("C X", 1 + 6),
        ("C Y", 2 + 0),
        ("C Z", 3 + 3),
    ]);
    let lines = reader.lines();
    let mut score: i16 = 0;
    for line in lines {
        score += score_map.get(&line.as_deref().unwrap()).unwrap();
    }

    // Part 1:
    println!("Day 2, part 1: {score}");

    //Part 2:
    let f = std::fs::File::open("./data/day_2")?;
    let reader = BufReader::new(f);
    let lines = reader.lines();
    let move_map = HashMap::from([('A', 1), ('B', 2), ('C', 3)]);
    let outcome_map = HashMap::from([('X', 0), ('Y', 3), ('Z', 6)]);
    let outcome_strategy_map = HashMap::from([
        ('X', HashMap::from([('A', 'C'), ('B', 'A'), ('C', 'B')])),
        ('Y', HashMap::from([('A', 'A'), ('B', 'B'), ('C', 'C')])),
        ('Z', HashMap::from([('A', 'B'), ('B', 'C'), ('C', 'A')])),
    ]);
    score = 0;
    for line in lines {
        let chars: Vec<char> = line.unwrap().chars().take(3).collect();
        let opponent_move = &chars[0];
        let outcome = &chars[2];
        let score_part_2_1 = outcome_strategy_map
            .get(&outcome)
            .unwrap()
            .get(&opponent_move)
            .unwrap();
        let score_part_1 = outcome_map.get(&outcome).unwrap();
        let score_part_2 = move_map.get(&score_part_2_1).unwrap();
        score += (score_part_1 + score_part_2)
    }
    // Part 2:
    println!("Day 2, part 2: {}", score);
    Ok(())
}
