#![no_std]
#![allow(static_mut_refs)]
use gstd::{exec, msg};
use pebbles_game_io::*;

static mut GAME_STATE: Option<GameState> = None;

#[cfg(test)]
pub fn get_random_u32() -> u32 {
    2
}

#[cfg(not(test))]
pub fn get_random_u32() -> u32 {
    let salt = msg::id();
    let (hash, _num) = exec::random(salt.into()).expect("get_random_u32(): random call failed");
    u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
}

#[no_mangle]
unsafe extern "C" fn init() {
    let pebbles_init: PebblesInit = msg::load().expect("Failed to load initial");

    let first_player = if get_random_u32() % 2 == 0 {
        Player::User
    } else {
        Player::Program
    };

    let pebbles_state = GameState {
        difficulty: pebbles_init.difficulty,
        pebbles_count: pebbles_init.pebbles_count,
        max_pebbles_per_turn: pebbles_init.max_pebbles_per_turn,
        pebbles_remaining: pebbles_init.pebbles_count,
        first_player,
        ..Default::default()
    };
    GAME_STATE = Some(pebbles_state);
    msg::reply_bytes("Successfully initialized", 0).expect("Failed to initialize successfully.");
}

#[no_mangle]
unsafe extern "C" fn handle() {
    let pebble_action: PebblesAction = msg::load().expect("Could not load PebblesAction");
    let pebble_game = GAME_STATE.as_mut().expect("Failed to get GAME_STATE");

    let result: PebblesEvent = match pebble_action {
        PebblesAction::Turn(turn_count) => pebble_game.turn(turn_count),
        PebblesAction::Restart {
            difficulty,
            pebbles_count,
            max_pebbles_per_turn,
        } => pebble_game.restart(difficulty, pebbles_count, max_pebbles_per_turn),
        PebblesAction::GiveUp => pebble_game.give_up(),
    };

    msg::reply(result, 0).expect("Failed to encode or reply with PebblesEvent.");
}

#[no_mangle]
extern "C" fn state() {
    let pebble_state = unsafe {
        GAME_STATE
            .as_ref()
            .expect("Unexpected error in getting state")
    };
    msg::reply(pebble_state, 0).expect("Unable to share the state");
}
