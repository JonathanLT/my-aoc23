use std::fs::read_to_string;


#[derive(Debug, Default, Eq, Ord, PartialEq, PartialOrd, Clone)]
struct Card {
    id: i32, 
    winning: Vec<i32>, 
    available: Vec<i32>
}


#[derive(Debug, Default, Eq, Ord, PartialEq, PartialOrd, Clone)]
struct CardCollection {
    cards: Vec<Card>,
}

impl CardCollection {
    fn new(lines: Vec<String>) -> Self {
        Self {
            cards: lines.iter().map(Card::new).collect(),
        }
    }

    fn get_card(&self, id: i32) -> Option<&Card> {
        self.cards.iter().find(|card| card.id == id)
    }

    fn get_cards(&self) -> &[Card] {
        &self.cards
    }

    fn total_original_cards(&self) -> usize {
        self.cards.len()
    }
}

impl Card {
    fn new(line: &String) -> Self {
        let (card_signature, lists) = line.split_once(':').unwrap();

        let (winning, available) = lists.split_once('|').unwrap();

        Self {
            id: Self::parse_card_id_from_signature(card_signature),
            winning: Self::parse_numbers_from_list(winning),
            available: Self::parse_numbers_from_list(available),
        }
    }

    fn parse_card_id_from_signature(signature: &str) -> i32 {
        let id = signature.split_whitespace().nth(1).unwrap();
        id.parse().unwrap()
    }

    fn parse_numbers_from_list(list: &str) -> Vec<i32> {
        list.split_whitespace()
            .map(|number| number.parse().unwrap())
            .collect()
    }

    fn get_matching_numbers_amount(&self) -> usize {
        self.available
            .iter()
            .filter(|available_num| {
                self.winning
                    .iter()
                    .any(|winning_num| *winning_num == **available_num)
            })
            .count()
    }

    fn get_total_won_cards(&self, card_collection: &CardCollection) -> i32 {
        self.won_cards_ids()
            .into_iter()
            .map(|id| {
                if let Some(card) = card_collection.get_card(id) {
                    card.get_total_won_cards(card_collection) + 1
                } else {
                    0
                }
            })
            .sum()
    }

    fn won_cards_ids(&self) -> Vec<i32> {
        (self.id + 1..)
            .take(self.get_matching_numbers_amount())
            .collect()
    }
}

fn reader(input: String) -> Vec<String> {
    read_to_string(input) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

pub fn compute_1 (input : String) -> i32 {
    let mut result: Vec<i32> = vec![];
    let contents: Vec<String> = reader(input);

    // for line in lines
        // split ":" [game, card]
        // split game " " [_, id]
        // split card "|" [winning, available]
        // split board " " [numbers parse]
        // split available " " [numbers parse]
        // point = 1
        // for matches board in available
            // point double
        // result push point
    // result iter sum

    let card_collection: CardCollection = CardCollection::new(contents);

    for card in card_collection.get_cards() {
        let mut flag_win = false;
        let mut point:i32 = 0;
        for win in &card.winning {
            if card.available.iter().any(|v| v == win) {
                if point == 0 {
                    point += 1;
                } else {
                    point *= 2;
                }
                flag_win = true;
            }
        }
        if flag_win {
            result.push(point);
        }
    }
    result.iter().sum()
}

pub fn compute_2(input: String) -> i32 {
    let contents = reader(input);
    let card_collection = CardCollection::new(contents);

    let result = card_collection
        .get_cards()
        .iter()
        .map(|card| card.get_total_won_cards(&card_collection))
        .sum::<i32>()
        + card_collection.total_original_cards() as i32;

    result
}