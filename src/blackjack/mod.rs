mod hand;
pub mod deck;

use hand::Hand;
use deck::Deck;

pub struct Game {
    deck: Deck,
    pub dealer_hand: Hand,
    pub player_hand: Hand,
}

impl Game {
    pub fn new() -> Game {
        let mut deck = Deck::new();
        let dealer_hand = Hand::new(
            deck.next().unwrap(),
            deck.next().unwrap(),
        );
        let player_hand = Hand::new(
            deck.next().unwrap(),
            deck.next().unwrap(),
        );

        Game {
            deck: deck,
            dealer_hand: dealer_hand,
            player_hand: player_hand,
        }
    }

    pub fn play_dealer(&mut self) -> Option<u8> {
        while self.dealer_hand.get_score() < 18 {
            self.dealer_hand.add_card(self.deck.next().unwrap())
        }
        if self.dealer_hand.get_score() > 21 {
            return None;
        } else {
            return Some(self.dealer_hand.get_score());
        }
    }

    pub fn hit_player(&mut self) -> Option<u8> {
        self.player_hand.add_card(self.deck.next().unwrap());
        let player_is_bust = self.check_player_bust();
        let result = match player_is_bust {
            true => None,
            false => Some(self.player_hand.get_score()),
        };
        result

    }

    pub fn check_player_bust(&self) -> bool {
        self.player_hand.get_score() >= 22
    }
}
