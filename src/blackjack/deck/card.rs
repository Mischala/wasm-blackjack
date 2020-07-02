use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Card {
    pub suite: Suite,
    pub rank: Rank,
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum Suite {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum Rank {
    One,
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
            Rank::One => 1,
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
        let suit: String;
        let rank: String;

        match self.suite {
            Suite::Clubs => suit = "♣️".to_string(),
            Suite::Spades => suit = "♠️".to_string(),
            Suite::Diamonds => suit = "♦️".to_string(),
            Suite::Hearts => suit = "♥️".to_string(),
        }

        match self.rank {
            Rank::One => rank = "1".to_string(),
            Rank::Two => rank = "2".to_string(),
            Rank::Three => rank = "3".to_string(),
            Rank::Four => rank = "4".to_string(),
            Rank::Five => rank = "5".to_string(),
            Rank::Six => rank = "6".to_string(),
            Rank::Seven => rank = "7".to_string(),
            Rank::Eight => rank = "8".to_string(),
            Rank::Nine => rank = "9".to_string(),
            Rank::Ten => rank = "10".to_string(),
            Rank::Ace => rank = "A".to_string(),
            Rank::King => rank = "K".to_string(),
            Rank::Queen => rank = "Q".to_string(),
            Rank::Jack => rank = "J".to_string(),
        }
        format!("{}{}", suit, rank)
    }
}

impl core::fmt::Display for Card {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.to_string() )
    }
}