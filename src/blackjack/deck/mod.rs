pub mod card;

use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;

pub fn make_deck() -> Vec<card::Card> {
    vec![
        card::Card::new(card::Rank::Ace, card::Suite::Clubs),
        card::Card::new(card::Rank::Two, card::Suite::Clubs),
        card::Card::new(card::Rank::Three, card::Suite::Clubs),
        card::Card::new(card::Rank::Four, card::Suite::Clubs),
        card::Card::new(card::Rank::Five, card::Suite::Clubs),
        card::Card::new(card::Rank::Six, card::Suite::Clubs),
        card::Card::new(card::Rank::Seven, card::Suite::Clubs),
        card::Card::new(card::Rank::Eight, card::Suite::Clubs),
        card::Card::new(card::Rank::Nine, card::Suite::Clubs),
        card::Card::new(card::Rank::Ten, card::Suite::Clubs),
        card::Card::new(card::Rank::Jack, card::Suite::Clubs),
        card::Card::new(card::Rank::King, card::Suite::Clubs),
        card::Card::new(card::Rank::Queen, card::Suite::Clubs),
        card::Card::new(card::Rank::Ace, card::Suite::Spades),
        card::Card::new(card::Rank::Two, card::Suite::Spades),
        card::Card::new(card::Rank::Three, card::Suite::Spades),
        card::Card::new(card::Rank::Four, card::Suite::Spades),
        card::Card::new(card::Rank::Five, card::Suite::Spades),
        card::Card::new(card::Rank::Six, card::Suite::Spades),
        card::Card::new(card::Rank::Seven, card::Suite::Spades),
        card::Card::new(card::Rank::Eight, card::Suite::Spades),
        card::Card::new(card::Rank::Nine, card::Suite::Spades),
        card::Card::new(card::Rank::Ten, card::Suite::Spades),
        card::Card::new(card::Rank::Jack, card::Suite::Spades),
        card::Card::new(card::Rank::King, card::Suite::Spades),
        card::Card::new(card::Rank::Queen, card::Suite::Spades),
        card::Card::new(card::Rank::Ace, card::Suite::Diamonds),
        card::Card::new(card::Rank::Two, card::Suite::Diamonds),
        card::Card::new(card::Rank::Three, card::Suite::Diamonds),
        card::Card::new(card::Rank::Four, card::Suite::Diamonds),
        card::Card::new(card::Rank::Five, card::Suite::Diamonds),
        card::Card::new(card::Rank::Six, card::Suite::Diamonds),
        card::Card::new(card::Rank::Seven, card::Suite::Diamonds),
        card::Card::new(card::Rank::Eight, card::Suite::Diamonds),
        card::Card::new(card::Rank::Nine, card::Suite::Diamonds),
        card::Card::new(card::Rank::Ten, card::Suite::Diamonds),
        card::Card::new(card::Rank::Jack, card::Suite::Diamonds),
        card::Card::new(card::Rank::King, card::Suite::Diamonds),
        card::Card::new(card::Rank::Queen, card::Suite::Diamonds),
        card::Card::new(card::Rank::Ace, card::Suite::Hearts),
        card::Card::new(card::Rank::Two, card::Suite::Hearts),
        card::Card::new(card::Rank::Three, card::Suite::Hearts),
        card::Card::new(card::Rank::Four, card::Suite::Hearts),
        card::Card::new(card::Rank::Five, card::Suite::Hearts),
        card::Card::new(card::Rank::Six, card::Suite::Hearts),
        card::Card::new(card::Rank::Seven, card::Suite::Hearts),
        card::Card::new(card::Rank::Eight, card::Suite::Hearts),
        card::Card::new(card::Rank::Nine, card::Suite::Hearts),
        card::Card::new(card::Rank::Ten, card::Suite::Hearts),
        card::Card::new(card::Rank::King, card::Suite::Hearts),
        card::Card::new(card::Rank::Jack, card::Suite::Hearts),
        card::Card::new(card::Rank::Queen, card::Suite::Hearts),
    ]
}

pub struct Deck {
    deck: Vec<card::Card>
}

impl Deck {
    pub fn new() -> Deck {
        let mut rng = StdRng::from_entropy();
        let mut unshuffled_deck = make_deck();
        unshuffled_deck.shuffle(&mut rng);
        let deck = unshuffled_deck;
        Deck {
            deck: deck
        }
    }
}

impl Iterator for Deck {
    type Item = card::Card;

    fn next(&mut self) -> Option<Self::Item> {
        if self.deck.len() > 0 {
            Some(self.deck.pop().unwrap())
        } else {
            None
        }
    }

}