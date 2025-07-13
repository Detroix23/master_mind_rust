// MASTER MIND
// Rules: you a have a set of, usually, 4 values hidden, taken randomly, or by a fellow human, from a pool of, usually, 8; there can be repetition.
// You have to uncover this set, with a max amount of tries, usually 8, and the hint given on each tries. 
// These hints are how much good and correctly placed values your guessed (⚪), and how much good but incorrectly placed values (🔴).

// Import
use rand::Rng;
use std::collections::HashMap;
use std::time::Instant;
use std::time::Duration;

use crate::manual::game_assist;
use crate::search_v1::game_robot;
use crate::{checks::Hint, manual::game_manual, search_v1::{all_set_entropy, combinations_hints, combinations_sets, combinations_sets_matching, max_entropy, set_entropy, SetScore}};

// Basic rules.
mod checks;
// Bot solver.
mod search_v1;
// Human gameplay.
mod manual;
// Benchmark
mod benchmark;

/// Activate feature-test mode
pub const MODE_TEST: bool = false;
/// Game player. Assist means that the human plays, but with the hints of the computer.
#[derive(PartialEq)]
pub enum ModePlayer {
    Robot,
    RobotBenchmark,
    Human,
    Assist
}
pub const MODE_PLAYER: ModePlayer = ModePlayer::Assist;


/// Define how many "colors" values there is.
pub const POOL_SIZE: u32 = 9u32;
/// Define the length of the hidden set.
pub const SET_LENGTH: usize = 4usize;
/// Define the maximum amount of tries availabes
pub const MAX_TRIES: u32 = 1024u32;

/// Graphical - Define hint types for UI.
pub struct UiHints<'a> {
    exact: &'a str,
    exist: &'a str,
    null: &'a str
}
/// Graphical - What to print for hint types.
pub const UI_HINTS: UiHints = UiHints {
    exact: "⚪",
    exist: "🔴",
    null: "⚫",
};

/// Graphical - Define kinds of info to show.
#[derive(PartialEq)]
pub enum UiLevel {
    All,
    Minimal,
    None
}
/// Graphical - How much info to actually show.
pub const UI_SHOW: UiLevel = UiLevel::All;

/// Debug - Activate.
pub const DEBUG_ACTIVATED: bool = true;
/// Debug - Log time
pub const DEBUG_LOG_TIME: bool = true;

/// Generate a set. Random choices.
pub fn generate_random_set() -> Vec<u32>{
    let mut set_hidden: Vec<u32> = Vec::new();
    while set_hidden.len() < SET_LENGTH {
        set_hidden.push(rand::rng().random_range(1u32..=POOL_SIZE));
    }
    set_hidden
}

/// Enter in a state to test features and function. Disable playing.
/// Edit directly in the code
fn mode_test() {
    println!("\n## Feature-test mode"); 
    let _combinations_sets: Vec<Vec<u32>> = combinations_sets();
    let _combinations_hint: Vec<Hint> = combinations_hints();
    /*
    println!("### Recursion test");
    println!("Combination of hints:");
    for hint in &_combinations_hint {
        println!("- {}{} {}{} {}{}", hint.exact, CORRECT_PLACEMENT, hint.exist, CORRECT_VALUE, hint.null, CORRECT_NON);
    }
    println!("q: {}", _combinations_hint.len());

    println!("Combinations of sets: {:?}. # {}", _combinations_sets, _combinations_sets.len());

    println!("### Entropy of a set.");
    let _set: Vec<u32> = vec![1, 2, 3, 4];
    println!("Set = {:?}", _set);

    let _set_entropy = set_entropy(&_set, &_combinations_sets, &_combinations_hint);
    println!("Entropy E = {:.3}", _set_entropy);

    println!("\n### Entropy of all sets on first guess.");
    let _entropies: HashMap<Vec<u32>, f64> = all_set_entropy(&_combinations_sets, &_combinations_hint);
    print!("All entropies: ");
    let mut _entropy_top: (Vec<Vec<u32>>, f64) = max_entropy(_entropies);
    println!("- Top: {:?} {}", _entropy_top.0, _entropy_top.1);
    
    let _hint_history_1: HashMap<Vec<u32>, Hint> = HashMap::from([
        (vec![1u32, 2u32, 3u32, 4u32], Hint { exact: 0, exist: 2, null: 2}), 
        (vec![5u32, 6u32, 7u32, 8u32], Hint { exact: 2, exist: 0, null: 2}),
    ]);
    let _filtered_sets_1 = combinations_sets_matching(&_hint_history_1, &_combinations_sets);
    println!("### Set filtering");
    println!("1. {:?}", _filtered_sets_1);
    */

}

/// Run the game, according to settings, debug and player.
pub fn main() {
    println!("\n# MASTER MIND");

    println!("\n## Initializing");
    // Printing values
    if POOL_SIZE < 16 {
        print!("- Pool of values: 1");
        for value in 2..=POOL_SIZE {
            print!(", {value}");
        }
        println!();
        } else {
        println!("- Pool of values (inclusive): 1 .. {}", POOL_SIZE);
    }
    println!("- Set length: {}", SET_LENGTH);
    println!("- Maximum tries: {}", MAX_TRIES);
    
    // Generating hidden set
    let set_hidden: Vec<u32> = generate_random_set();
    if DEBUG_ACTIVATED {println!("- DEBUG - Hidden set: {:?}", set_hidden);}

    // Running game.
    if MODE_TEST {
        mode_test();
    } else if MODE_PLAYER == ModePlayer::Human {
        game_manual(set_hidden);
    } else if MODE_PLAYER == ModePlayer::Robot {
        game_robot(set_hidden);
    } else if MODE_PLAYER == ModePlayer::Assist {
        game_assist(set_hidden);
    } else {
        println!("(X) - No mode selected. Exitting.");
    }

}