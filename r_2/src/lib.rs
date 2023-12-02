use std::fs::File;
use std::io::{prelude::*, BufReader};

fn reader(input: String) -> std::io::Lines<BufReader<File>>{
    let file = File::open(input);
    let reader = BufReader::new(file.unwrap());
    reader.lines()
}

pub fn compute_1 (input : String) -> i32 {
    let max_hand: Vec<i32> = vec![12, 13, 14]; // RGB
    let mut result: Vec<i32> = vec![];

    for line in reader(input) {
        if let Some((game_str, hands_str)) = line.unwrap().split_once(": ") {
            if let Some((_, game_id_str)) = game_str.split_once(' ') {
                let game_id: i32 = game_id_str.parse::<i32>().unwrap();
                let mut valid: bool = true;
                for hand_str in hands_str.split("; ") {
                    let hand_colors_str:Vec<&str> = hand_str.split(", ").collect();
                    for hand_color_str in hand_colors_str {
                        let color_str:Vec<&str> = hand_color_str.split(" ").collect();
                        match color_str[1] {
                            "red" => {
                                if color_str[0].parse::<i32>().unwrap() > max_hand[0] {
                                    valid = false;
                                }
                            },
                            "green" => {
                                if color_str[0].parse::<i32>().unwrap() > max_hand[1] {
                                    valid = false;
                                }
                            },
                            "blue" => {
                                if color_str[0].parse::<i32>().unwrap() > max_hand[2] {
                                    valid = false;
                                }
                            },
                            _ => return 0
                        };
                    }
                }
                if valid {
                    result.push(game_id)
                }
            }
        }
    };
    
    result.iter().sum()
}

pub fn compute_2 (input : String) -> i32 {
    let mut result: Vec<i32> = vec![];

    for line in reader(input) {
        if let Some((game_str, hands_str)) = line.unwrap().split_once(": ") {
            if let Some((_, _)) = game_str.split_once(' ') {
                let mut max_game_color: Vec<i32> = vec![0, 0, 0];
                for hand_str in hands_str.split("; ") {
                    let hand_colors_str:Vec<&str> = hand_str.split(", ").collect();
                    for hand_color_str in hand_colors_str {
                        let color_str:Vec<&str> = hand_color_str.split(" ").collect();
                        match color_str[1] {
                            "red" => {
                                if color_str[0].parse::<i32>().unwrap() > max_game_color[0] {
                                    max_game_color[0] = color_str[0].parse::<i32>().unwrap();
                                }
                            },
                            "green" => {
                                if color_str[0].parse::<i32>().unwrap() > max_game_color[1] {
                                    max_game_color[1] = color_str[0].parse::<i32>().unwrap();
                                }
                            },
                            "blue" => {
                                if color_str[0].parse::<i32>().unwrap() > max_game_color[2] {
                                    max_game_color[2] = color_str[0].parse::<i32>().unwrap();
                                }
                            },
                            _ => return 0
                        };
                    }
                }
                result.push(max_game_color.iter().copied().reduce(|a, b| a * b).unwrap());
            }
        }
    };
    
    result.iter().sum()
}