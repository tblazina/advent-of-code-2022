use anyhow::Result;
use std::{
    collections::{HashMap, VecDeque},
    iter::zip,
    vec,
};

#[derive(Debug)]
struct Move {
    moves: i32,
    source: i32,
    destination: i32,
}

pub fn parse_input(path: &str) -> Result<String> {
    let input = std::fs::read_to_string(path)?;
    Ok(input)
}

fn parse_move(row: &str) -> Result<Move> {
    let split_string: Vec<&str> = row.split_whitespace().collect();
    let moves: i32 = split_string[1].parse()?;
    let source: i32 = split_string[3].parse()?;
    let destination: i32 = split_string[5].parse()?;

    Ok(Move {
        moves,
        source,
        destination,
    })
}

fn parse_header(header: Vec<&str>) -> Result<HashMap<i32, VecDeque<char>>> {
    let positions: Vec<(i32, VecDeque<char>)> =
        (1..9).step_by(1).map(|r| (r, VecDeque::new())).collect();
    let mut parsed_header: HashMap<i32, VecDeque<char>> = HashMap::from_iter(positions);
    for row in header.iter() {
        let i1 = (1..10).step_by(1);
        let i2 = (0..33).step_by(4);
        let i3 = (3..36).step_by(4);
        for (i, (j, k)) in zip(i1, zip(i2, i3)) {
            let char_vec = row[j..k].to_owned().chars().collect::<Vec<_>>();
            if char_vec.len() > 0 {
                let char = char_vec[1];
                if char != ' ' {
                    parsed_header
                        .entry(i)
                        .or_insert(VecDeque::new())
                        .push_back(char);
                }
            }
        }
    }
    Ok(parsed_header)
}

fn make_moves(
    mut container: HashMap<i32, VecDeque<char>>,
    moves: Move,
) -> HashMap<i32, VecDeque<char>> {
    let mut popped: Vec<char> = Vec::new();
    container.entry(moves.source).and_modify(|x| {
        let num = moves.moves as usize;
        for i in 0..num {
            let to_add = x.get(i).unwrap().to_owned();
            popped.push(to_add);
        }
    }).and_modify(|x| {
        let num = moves.moves as usize;
        x.drain(0..num);
    });
    container.entry(moves.destination).and_modify(|x| {
        for element in popped {
            x.push_front(element);
        }
    });
    container
}

fn make_moves2(
    mut container: HashMap<i32, VecDeque<char>>,
    moves: Move,
) -> HashMap<i32, VecDeque<char>> {
    let mut popped: Vec<char> = Vec::new();
    container.entry(moves.source).and_modify(|x| {
        let num = moves.moves as usize;
        for i in 0..num {
            let to_add = x.get(i).unwrap().to_owned();
            popped.push(to_add);
        }
    }).and_modify(|x| {
        let num = moves.moves as usize;
        x.drain(0..num);
    });
    container.entry(moves.destination).and_modify(|x| {
        popped.reverse();
        for element in popped {
            x.push_front(element);
        }
    });
    container
}

pub fn day_5_part_1() -> Result<()> {
    let input = parse_input("./data/day_5")?;
    let contents: Vec<&str> = input.lines().collect();
    let header = contents[0..8].to_owned();
    let mut container = parse_header(header)?;
    let length = contents.len();
    for row in &contents[10..length] {
        let parsed_move = parse_move(row)?;
        container = make_moves(container, parsed_move);
    }
    dbg!(&container);
    for (k, v) in container.iter() {
        println!("{}, {}", k, v[0]);
    }
    Ok(())
}
pub fn day_5_part_2() -> Result<()> {
    let input = parse_input("./data/day_5")?;
    let contents: Vec<&str> = input.lines().collect();
    let header = contents[0..8].to_owned();
    let mut container = parse_header(header)?;
    let length = contents.len();
    for row in &contents[10..length] {
        let parsed_move = parse_move(row)?;
        container = make_moves2(container, parsed_move);
    }
    dbg!(&container);
    for (k, v) in container.iter() {
        println!("{}, {}", k, v[0]);
    }
    Ok(())
}