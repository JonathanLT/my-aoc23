use std::fs::read_to_string;
use itertools::Itertools;

#[derive(Debug)]
struct Card {
    c: char,
    v: usize
}

impl Card {
    fn new(c: &char, v: &usize) -> Self {
        Self {
            c: *c,
            v: *v
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: usize,
    strength: (usize, usize),
}

impl Hand {
    fn new(line: &String) -> Self {
        let (cards_str, bid_str): (&str, &str) = line.split_once(" ").unwrap();
        let cards: Vec<Card> = cards_str.chars().into_iter().map(|c| {
            Card {
                c: c,
                v: card_index(c, false)
            }
        }).collect::<Vec<Card>>();
        let bid: usize = bid_str.parse::<usize>().unwrap();
        Self {
            cards: cards,
            bid: bid,
            strength: hand_strength(&cards_str.to_string(), false),
        }
    }
    fn new_with_joker(line: &String) -> Self {
        let (cards_str, bid_str): (&str, &str) = line.split_once(" ").unwrap();
        let cards: Vec<Card> = cards_str.chars().into_iter().map(|c| {
            Card {
                c: c,
                v: card_index(c, true)
            }
        }).collect::<Vec<Card>>();
        let bid: usize = bid_str.parse::<usize>().unwrap();
        Self {
            cards: cards,
            bid: bid,
            strength: hand_strength(&cards_str.to_string(), true),
        }
    }
}

fn card_index(c: char, p2: bool) -> usize {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => if p2 {0} else {11},
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => unreachable!()
    }
}

fn get_hand_type(counts: &[usize], jokers: usize) -> usize {
    match (*counts.iter().max().unwrap_or(&0), jokers) {
        (a,b) if a + b == 5 => 6,
        (a,b) if a + b == 4 => 5,
        (3,0) => if counts.contains(&2) {4} else {3},
        (2,_) => {
        let pairs = counts.iter().filter(|&&v| v == 2).count();
        match (pairs, jokers) {
            (2,1) => 4,
            (1,1) => 3,
            (2,0) => 2,
            _ => 1,
        }
        },
        (1,2) => 3,
        (1,1) => 1,
        _ => 0,
    }
}
fn hand_strength(cards: &String, p2: bool) -> (usize, usize) {
    let counts_by_card = cards.chars().counts();
    let counts = counts_by_card.iter()
        .filter(|&(&k,_)| k != 'J' || !p2)
        .map(|(_,&v)| v)
        .collect::<Vec<_>>();
    let jokers = if p2 {*counts_by_card.get(&'J').unwrap_or(&0)} else {0};
    let idx = cards.chars().fold(0, |acc, c| (acc << 4) + card_index(c, p2));
    (get_hand_type(&counts, jokers), idx)
}

fn reader(input: String) -> Vec<String> {
    read_to_string(input)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}


pub fn compute_1(input: String) -> usize {
    let result: usize;
    let contents: Vec<String> = reader(input);
    let mut hands: Vec<Hand> = contents.iter().map(Hand::new).collect();

    hands.sort_unstable_by_key(|h| h.strength);
    result = hands.iter().enumerate().map(|(i, h)| (i + 1) * h.bid).sum();

    result
}

pub fn compute_2(input: String) -> usize {
    let result: usize;
    let contents: Vec<String> = reader(input);
    let mut hands: Vec<Hand> = contents.iter().map(Hand::new_with_joker).collect();

    hands.sort_unstable_by_key(|h| h.strength);
    result = hands.iter().enumerate().map(|(i, h)| (i + 1) * h.bid).sum();

    result
}