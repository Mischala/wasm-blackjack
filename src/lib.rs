use wasm_bindgen::prelude::*;
use std::{sync::Mutex};
use once_cell::sync::Lazy;

mod utils;
mod blackjack;
use crate::blackjack::Game;

static GAME_STATE: Lazy<Mutex<Game>> = Lazy::new( || {
    utils::set_panic_hook();
    Mutex::new(Game::new())
});

#[wasm_bindgen]
pub fn start_new_game() {
    let mut game_state = GAME_STATE.lock().unwrap();
    *game_state = Game::new();
}

#[wasm_bindgen]
pub fn play_dealer() -> Option<u8> {
    let mut game_state = GAME_STATE.lock().unwrap();
    let dealer_score = game_state.play_dealer();
    dealer_score
}

#[wasm_bindgen]
pub fn hit_player() -> Option<u8> {
    let mut game_state = GAME_STATE.lock().unwrap();
    let player_score = game_state.hit_player();
    player_score
}

#[wasm_bindgen]
pub fn check_player_bust() -> bool {
    let game_state = GAME_STATE.lock().unwrap();
    game_state.check_player_bust()
}
