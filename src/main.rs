// MASTER MIND
// Rules: you a have a set of, usually, 4 values hidden, taken randomly, or by a fellow human, from a pool of, usually, 8; there can be repetition.
// You have to uncover this set, with a max amount of tries, usually 8, and the hint given on each tries. 
// These hints are how much good and correctly placed values your guessed (⚪), and how much good but incorrectly placed values (🔴).

// Import
use rand::Rng;
use std::collections::HashMap;
use std::io::Read;
use std::time::Instant;
use std::time::Duration;

use crate::manual::game_assist;
use crate::search_v1::game_robot;
use crate::{checks::Hint, manual::game_manual, search_v1::{combinations_hints, combinations_sets}};

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
#[derive(PartialEq, Debug)]
pub enum ModePlayer {
    Robot,
    RobotBenchmark,
    Human,
    Assist
}

/// Define the default player.
pub const DEFAULT_MODE_PLAYER: ModePlayer = ModePlayer::Robot;
/// Define the default choice for advanced config.
pub const DEFULAT_ADVANCED_CONFIG: bool = false;
/// Define how many default "colors" values there is.
pub const DEFAULT_POOL_SIZE: u32 = 8u32;
/// Define the default length of the hidden set.
pub const DEFAULT_SET_LENGTH: usize = 4usize;
/// Define the default maximum amount of tries availabes
pub const DEFAULT_MAX_TRIES: u32 = 8u32;

/// Graphical - Define hint types for UI.
pub struct UiHints<'a> {
    exact: &'a str,
    exist: &'a str,
    null: &'a str
}
/// Graphical - What to print for hint types.
pub const UI_HINTS: UiHints = UiHints {
    exact: "██",
    exist: "▒▒",
    null:  "┈┈",
};

/// Graphical - Define kinds of info to show.
#[derive(PartialEq, Debug)]
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
pub fn generate_random_set(set_length: usize, pool_size: u32) -> Vec<u32>{
    let mut set_hidden: Vec<u32> = Vec::new();
    while set_hidden.len() < set_length {
        set_hidden.push(rand::rng().random_range(1u32..=pool_size));
    }
    set_hidden
}

/// Enter in a state to test features and function. Disable playing.
/// Edit directly in the code
fn mode_test(_set_length: usize, _pool_size: u32, _max_tries: u32) {
    println!("\n## Feature-test mode"); 
    let _combinations_sets: Vec<Vec<u32>> = combinations_sets(_set_length, _pool_size);
    let _combinations_hint: Vec<Hint> = combinations_hints(_set_length);
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
    println!("\n# MASTER MIND.");

    println!("\n## Initialization.");
    if DEBUG_ACTIVATED {
        println!("### Default values.");
        // Printing values
        if DEFAULT_POOL_SIZE < 16 {
            print!("- Pool of values: 1");
            for value in 2..=DEFAULT_POOL_SIZE {
                print!(", {value}");
            }
            println!();
            } else {
            println!("- Pool of values (inclusive): 1 .. {}", DEFAULT_POOL_SIZE);
        }
        println!("- Set length: {}", DEFAULT_SET_LENGTH);
        println!("- Maximum tries: {}", DEFAULT_MAX_TRIES);
        println!("- Game player: {:#?}", DEFAULT_MODE_PLAYER);
        println!("- Ui level: {:#?}", UI_SHOW);
    }
    
    println!("### User configuration.");
    let mut user_game_player: String = String::new();
    let mut user_advanced_config: String = String::new();
    let mut user_pool_size: String = String::new();
    let mut user_set_length: String = String::new();
    let mut user_max_tries: String = String::new();
    
    println!("- Enter game player [Human, Robot, Assisted, RobotBenchmark(W.I.P.)] ({:#?}): ", DEFAULT_MODE_PLAYER);
    std::io::stdin()
        .read_line(&mut user_game_player)
        .expect("(X) - Error reading line.");
    let user_game_player: ModePlayer = match &*user_game_player.trim().to_lowercase() {
        "human" | "h" => ModePlayer::Human,
        "robot" | "r" => ModePlayer::Robot,
        "assisted" | "assist" | "a" | "ass" => ModePlayer::Assist,
        "robot_benchmark" | "robotbenchmark" | "robot benchmark" | "benchmark" | "bench" | "rb" => ModePlayer::RobotBenchmark,
        _ => DEFAULT_MODE_PLAYER
    };
    println!("Chose {:#?}", user_game_player);

    println!("- Do you want to enter advanced configuration [yes/ no] (no): ");
    std::io::stdin()
        .read_line(&mut user_advanced_config)
        .expect("(X) - Error reading line.");
    let user_advanced_config: bool = match &*user_advanced_config.trim().to_lowercase() {
        "yes" | "ye" | "y" => true,
        "non" | "no" | "n" => false,
        _ => DEFULAT_ADVANCED_CONFIG,
    };

    if user_advanced_config {
        println!("### Advanced configuration.");
        println!("- Enter pool size [N*] ({}): ", DEFAULT_POOL_SIZE);
        std::io::stdin()
            .read_line(&mut user_pool_size)
            .expect("(X) - Error reading line.");
        println!("- Enter set length [N*] ({}): ", DEFAULT_SET_LENGTH);
        std::io::stdin()
            .read_line(&mut user_set_length)
            .expect("(X) - Error reading line.");
        println!("- Enter max tries [N*] ({}): ", DEFAULT_MAX_TRIES);
        std::io::stdin()
            .read_line(&mut user_max_tries)
            .expect("(X) - Error reading line.");

    } else {
        println!("Skiping advanced configuration.");
    }

    let user_pool_size: u32 = match user_pool_size.trim().parse::<u32>() {
        Ok(number) => number,
        Err(_) => DEFAULT_POOL_SIZE
    };
    let user_set_length: usize = match user_set_length.trim().parse::<usize>() {
        Ok(number) => number,
        Err(_) => DEFAULT_SET_LENGTH
    };
    let user_max_tries: u32 = match user_max_tries.trim().parse::<u32>() {
        Ok(number) => number,
        Err(_) => DEFAULT_MAX_TRIES
    };

    println!("### Final values");
    if user_pool_size < 16 {
        print!("- Pool of values: 1");
        for value in 2..=user_pool_size {
            print!(", {value}");
        }
        println!();
    } else {
        println!("- Pool of values (inclusive): 1 .. {}.", user_pool_size);
    }
    println!("- Set length: {}.", user_set_length);
    println!("- Maximum tries: {}.", user_max_tries);
    println!("- Game player: {:#?}.", user_game_player);
    println!("- Ui level: {:#?}.", UI_SHOW);
    println!("- Ui for hint: {} = `exact`, {} = `exists`, {} = `null`.", UI_HINTS.exact, UI_HINTS.exist, UI_HINTS.null);

    // Generating hidden set
    let set_hidden: Vec<u32> = generate_random_set(user_set_length, user_pool_size);
    if DEBUG_ACTIVATED {
        println!("\n## Debug ON.");
        println!("- DEBUG - Hidden set: {:?}", set_hidden);
    }

    // Running game.
    if MODE_TEST {
        mode_test(user_set_length, user_pool_size, user_max_tries);
    } else if user_game_player == ModePlayer::Human {
        println!("\n## HUMAN PLAY (Engine assist: OFF).");
        println!("Enter your guesses. Seperated by ','. Ex: `1, 2, 3, 4`.");
        game_manual(set_hidden, user_set_length,  user_pool_size, user_max_tries);
    } else if user_game_player == ModePlayer::Robot {
        println!("\n## ENGINE PLAY (Autoplay: ON, Benchmark: OFF).");
        game_robot(set_hidden, user_set_length,  user_pool_size, user_max_tries);
    } else if user_game_player == ModePlayer::Assist {
        println!("\n## HUMAN PLAY (Engine assist: ON).");
        game_assist(set_hidden, user_set_length,  user_pool_size, user_max_tries);
    } else if user_game_player == ModePlayer::RobotBenchmark {
        println!("(!) - Robot Benchmark is Work-In-Progress. ")
    } else {
        println!("(X) - No valid mode selected. ");
    }

    println!("Press enter to exit...");
    std::io::stdin()
        .read_line(&mut String::new())
        .expect("(X) - Can't read line. Exiting anyways.");
}