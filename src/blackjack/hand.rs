use std::fmt;

use crate::blackjack::deck::card::{ Card, Rank };

pub struct Hand {
    pub cards_in_hand: Vec<Card>,
    num_aces: u8
}

impl Hand {

    pub fn new(card1: Card, card2: Card) -> Hand {
        let mut hand = Hand {
            cards_in_hand: vec![],
            num_aces: 0
        };
        hand.add_card(card1);
        hand.add_card(card2);
        hand
    }

    pub fn add_card(&mut self, card: Card) {
        match card.rank {
            Rank::Ace => self.num_aces = self.num_aces + 1,
            _ => ()
        }
        self.cards_in_hand.push(card);
    }

    pub fn get_score(&self) -> u8 {
        let card_values = self.cards_in_hand.iter()
            .map(|card| card.get_value());

        let mut score = card_values.sum();
        let mut ace_count = self.num_aces.clone();

        while ace_count > 0 && score > 21 {
            ace_count = ace_count - 1;
            score = score - 10;
        }
        score
    }

}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hand_string = self.cards_in_hand.iter().fold(
            String::from(""),
            |acc, card| format!("{} {}", acc, card.to_string()));
        write!(f, "{} Score:{}", hand_string, self.get_score())
    }
}
