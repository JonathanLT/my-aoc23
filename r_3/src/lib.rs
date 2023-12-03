use std::fs::read_to_string;
use regex::Regex;


#[derive(Debug)]
#[derive(PartialEq)]
struct Position(i32, i32);

#[derive(Debug)]
struct Gear(i32, Position, Position);

fn reader(input: String) -> Vec<String> {
    read_to_string(input) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

pub fn compute_1 (input : String) -> i32 {
    let mut result: Vec<i32> = vec![];

    let contents = reader(input);
    let num_re = Regex::new(r"\d+").unwrap();

    let mut numbers: Vec<(i32, Position)> = vec![];
    let mut symbols: Vec<(char, Position)> = vec![];

    for (row, line) in contents.iter().enumerate() {
        for num_match in num_re.find_iter(&line) {
            numbers.push((num_match.as_str().parse::<i32>().unwrap(), Position(row as i32, num_match.start() as i32)))
        }
        for (col, c) in line.chars().enumerate() {
            if !c.is_ascii_digit() && c != '.' {
                symbols.push((c, Position(row as i32, col as i32)));
            }
        }
    }
    for (num, num_start_pos) in numbers.iter() {
        let num_end_pos = Position(num_start_pos.0, num_start_pos.1 + num.to_string().len() as i32 - 1);
        for (_, symbol_pos) in symbols.iter() {
            // [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 0), (0, 1), (1, -1), (1, 0), (1, 1)]

            let neighbors = [
                Position(symbol_pos.0 - 1, symbol_pos.1 - 1),
                Position(symbol_pos.0 - 1, symbol_pos.1),
                Position(symbol_pos.0 - 1, symbol_pos.1 + 1),
                Position(symbol_pos.0, symbol_pos.1 - 1),
                Position(symbol_pos.0, symbol_pos.1 + 1),
                Position(symbol_pos.0 + 1, symbol_pos.1 - 1),
                Position(symbol_pos.0 + 1, symbol_pos.1),
                Position(symbol_pos.0 + 1, symbol_pos.1 + 1),
            ].into_iter().collect::<Vec<Position>>();
            for neighbor in neighbors.iter() {
                if neighbor.0 == num_start_pos.0
                    && neighbor.1 >= num_start_pos.1
                    && neighbor.1 <= num_end_pos.1
                {
                    result.push(*num);
                    break;
                }
            }
        }
    }
    result.iter().sum()
}

pub fn compute_2 (input : String) -> i32 {
    let mut result: Vec<i32> = vec![];

    let contents = reader(input);
    let num_re = Regex::new(r"\d+").unwrap();

    let mut numbers: Vec<(i32, Position)> = vec![];
    let mut gears: Vec<Gear> = vec![];
    let mut symbols: Vec<(char, Position)> = vec![];

    for (row, line) in contents.iter().enumerate() {
        for num_match in num_re.find_iter(&line) {
            numbers.push((num_match.as_str().parse::<i32>().unwrap(), Position(row as i32, num_match.start() as i32)))
        }
        for (col, c) in line.chars().enumerate() {
            if !c.is_ascii_digit() && c != '.' {
                symbols.push((c, Position(row as i32, col as i32)));
            }
        }
    }
    for (num, num_start_pos) in numbers.iter() {
        let num_end_pos = Position(num_start_pos.0, num_start_pos.1 + num.to_string().len() as i32 - 1);
        for (symbol, symbol_pos) in symbols.iter() {
            // [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 0), (0, 1), (1, -1), (1, 0), (1, 1)]
            let neighbors = [
                Position(symbol_pos.0 - 1, symbol_pos.1 - 1),
                Position(symbol_pos.0 - 1, symbol_pos.1),
                Position(symbol_pos.0 - 1, symbol_pos.1 + 1),
                Position(symbol_pos.0, symbol_pos.1 - 1),
                Position(symbol_pos.0, symbol_pos.1 + 1),
                Position(symbol_pos.0 + 1, symbol_pos.1 - 1),
                Position(symbol_pos.0 + 1, symbol_pos.1),
                Position(symbol_pos.0 + 1, symbol_pos.1 + 1),
            ].into_iter().collect::<Vec<Position>>();
            for neighbor in neighbors.iter() {
                if neighbor.0 == num_start_pos.0
                    && neighbor.1 >= num_start_pos.1
                    && neighbor.1 <= num_end_pos.1
                {
                    if symbol == &'*' {
                        let find_gear = gears.iter().find(|r| r.1 == Position(symbol_pos.0, symbol_pos.1));
                        if !find_gear.is_none() {
                            let gear = find_gear.unwrap().0 * *num;
                            result.push(gear);
                        }
                        gears.push(Gear(*num, Position(symbol_pos.0, symbol_pos.1), Position(neighbor.0, neighbor.1)));
                        break;
                    }
                }
            }
        }
    }
    result.iter().sum()
}