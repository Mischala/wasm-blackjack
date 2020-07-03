import * as wasmBlackjack from "../pkg/wasm_blackjack";

window.wasmBlackjack = wasmBlackjack;

wasmBlackjack.start_new_game();
console.log(`Is player bust: ${wasmBlackjack.check_player_bust()}`);
