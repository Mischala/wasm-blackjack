use wasm_bindgen::prelude::*;

pub struct Card {
    pub suite: Suite,
    pub rank: Rank,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum Suite {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Ace,
    Jack,
    King,
    Queen
}

impl Card {
    pub fn new(rank: Rank, suite: Suite) -> Card {
        Card {
            suite: suite,
            rank: rank,
        }
    }

    pub fn get_value(&self) -> u8 {
        match self.rank {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Ace => 11,
            _ => 10
        }
    }

    pub fn to_string(&self) -> String {
        let suit = match self.suite {
            Suite::Clubs => "♣️".to_string(),
            Suite::Spades => "♠️".to_string(),
            Suite::Diamonds => "♦️".to_string(),
            Suite::Hearts => "♥️".to_string(),
        };

        let rank = match self.rank {
            Rank::Two => "2".to_string(),
            Rank::Three => "3".to_string(),
            Rank::Four => "4".to_string(),
            Rank::Five => "5".to_string(),
            Rank::Six => "6".to_string(),
            Rank::Seven => "7".to_string(),
            Rank::Eight => "8".to_string(),
            Rank::Nine => "9".to_string(),
            Rank::Ten => "10".to_string(),
            Rank::Ace => "A".to_string(),
            Rank::King => "K".to_string(),
            Rank::Queen => "Q".to_string(),
            Rank::Jack => "J".to_string(),
        };

        format!("{}{}", suit, rank)
    }
}

impl core::fmt::Display for Card {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.to_string() )
    }
}