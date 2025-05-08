#![no_std]
use gtest::{Program, System};
use pebbles_game_io::*;

const USER: u64 = 1024;

fn init_game(
    sys: &System,
    difficulty: DifficultyLevel,
    pebbles_count: u32,
    max_pebbles_per_turn: u32,
) -> Program<'_> {
    sys.init_logger();
    let program = Program::current(sys);
    sys.mint_to(USER, 10000000000000000);

    let pebbles_init = PebblesInit {
        difficulty,
        pebbles_count,
        max_pebbles_per_turn,
    };
    program.send(USER, pebbles_init);
    sys.run_next_block();

    program
}

#[test]
fn test_user_turn_winning() {
    let sys = System::new();
    let program = init_game(&sys, DifficultyLevel::Easy, 3, 3);
    program.send(USER, PebblesAction::Turn(3));
    sys.run_next_block();
    let state: GameState = program.read_state(()).expect("Failed to read state");
    assert_eq!(state.pebbles_remaining, 0);
    assert_eq!(state.winner, Some(Player::User));
}

#[test]
fn test_program_turn_winning() {
    let sys = System::new();
    let program = init_game(&sys, DifficultyLevel::Hard, 4, 3);
    program.send(USER, PebblesAction::Turn(1));
    sys.run_next_block();
    let state: GameState = program.read_state(()).expect("Failed to read state");
    assert_eq!(state.pebbles_remaining, 0);
    assert_eq!(state.winner, Some(Player::Program));
}

#[test]
fn test_restart_game() {
    let sys = System::new();
    let program = init_game(&sys, DifficultyLevel::Easy, 10, 3);
    program.send(
        USER,
        PebblesAction::Restart {
            difficulty: DifficultyLevel::Hard,
            pebbles_count: 5,
            max_pebbles_per_turn: 2,
        },
    );
    sys.run_next_block();
    let state: GameState = program.read_state(()).expect("Failed to read state");
    assert_eq!(state.pebbles_remaining, 5);
    assert_eq!(state.max_pebbles_per_turn, 2);
    assert_eq!(state.difficulty, DifficultyLevel::Hard);
}

#[test]
fn test_give_up() {
    let sys = System::new();
    let program = init_game(&sys, DifficultyLevel::Easy, 10, 3);
    program.send(USER, PebblesAction::GiveUp);
    sys.run_next_block();
    let state: GameState = program.read_state(()).expect("Failed to read state");
    assert_eq!(state.winner, Some(Player::Program));
}
