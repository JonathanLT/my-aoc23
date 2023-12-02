use regex::Regex;
use std::path::Path;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn compute_1(input: String) -> i32 {
    let mut result: i32 = 0;

    let file = File::open(input);
    let reader = BufReader::new(file.unwrap());
    let re_alpha = Regex::new(r"[A-Za-z]").unwrap();

    for line in reader.lines() {
        let contents = line.unwrap();
        let result_str = re_alpha.replace_all(&contents, "");
        result = result
            + format!(
                "{}{}",
                result_str.chars().take(1).last().unwrap(),
                result_str.chars().last().unwrap()
            )
            .parse::<i32>()
            .unwrap();
    }

    result
}

pub fn compute_2(input: String) -> i32 {
    let mut result: i32 = 0;

    let file = File::open(input);
    let reader = BufReader::new(file.unwrap());

    for line in reader.lines() {
        let mut flag:bool = false;
        let first: i32;
        let last: i32;

        let contents = line.unwrap();
        let re = Regex::new(
            r"(?<first>(\d|one|two|three|four|five|six|seven|eight|nine)).*(?<last>(\d|one|two|three|four|five|six|seven|eight|nine))",
        )
        .unwrap();
        let caps = match re.captures(&contents) {
            Some(caps) => caps,
            None => {
                flag = true;
                let rematch = Regex::new(r".*(?<first>(\d|one|two|three|four|five|six|seven|eight|nine)).*",).unwrap();
                rematch.captures(&contents).unwrap()
            }
        };
        first = extract_number(caps["first"].to_string());
        if !flag {
            last = extract_number(caps["last"].to_string())
        } else {
            last = first;
        }
        result = result
            + format!(
                "{}{}",
                first,
                last
            )
            .parse::<i32>()
            .unwrap();
    }

    result
}

fn extract_number(caps: String) -> i32 {
    let number: i32;
    if caps.parse::<i32>().is_ok() {
        number = caps.parse::<i32>().unwrap();
    } else {
        number = match caps.as_str() {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => 0,
        };
    };
    number
}