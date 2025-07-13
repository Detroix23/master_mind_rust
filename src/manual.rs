// MASTER MIND
// Human gameplay and assisted-by-bot gameplay.

use std::time::Duration;

use crate::{HashMap, Instant, UiLevel, DEBUG_ACTIVATED, DEBUG_LOG_TIME, MAX_TRIES, SET_LENGTH, UI_HINTS, UI_SHOW};
use crate::checks::{convert_input_set, similarities, Hint};
use crate::search_v1::{all_set_entropy, combinations_hints, combinations_sets, combinations_sets_matching, max_entropy, set_entropy};

/// Execute the game where the human player tries to guess.
pub fn game_manual(set_hidden: Vec<u32>) -> bool {
    // User game.
    

    let mut guesses: u32 = 1;
    let mut found: bool = false;

    while guesses <= MAX_TRIES && !found {
        // User input.
        let mut user_guess_correct: bool = false;
        let mut user_guess_set_string: String = String::new();
        let mut user_guess_set: Vec<u32> = Vec::new();
        let mut cheat_jojo: bool = false;

        println!("Guess {}.", guesses);
        while !user_guess_correct {
            // Read
            std::io::stdin()
                .read_line(&mut user_guess_set_string)
                .expect("(!) Warning - Couldn't read line.");

            user_guess_set_string = user_guess_set_string.trim().to_string();
            // Convert to vector
            user_guess_set = convert_input_set(user_guess_set_string.clone());
            
            
            // Validate input
            if user_guess_set.len() == SET_LENGTH {
                user_guess_correct = true;
            } else if DEBUG_ACTIVATED && (user_guess_set_string == String::from("jojo")) {
                // Cheat - Instant win.
                println!("Cheat - Instant win.");
                cheat_jojo = true;
                user_guess_correct = true;
            } else if DEBUG_ACTIVATED && (user_guess_set_string == String::from("omniscia")) {
                // Cheat - Autocorrect input.
                println!("Cheat - Auto-correct input.");
                user_guess_set = set_hidden.clone();
                user_guess_correct = true;
            } else {
                println!("(!) Warning - Invalid input; Input difference: {}", SET_LENGTH as i32 - user_guess_set.len() as i32);
                user_guess_set_string = String::new();
            }
        }

        // Comparing.
        let comparison_results: Hint = similarities(&set_hidden, &user_guess_set);
        
        // UI
        println!("- Set: {:?}", user_guess_set);
        println!("- Found hint: {}{} {}{} {}{}", comparison_results.exact, UI_HINTS.exact, comparison_results.exist, UI_HINTS.exist, comparison_results.null, UI_HINTS.null);

        // End turn.
        found = (comparison_results.exact as usize == SET_LENGTH) | cheat_jojo;

        guesses += 1;
    } 
    println!("\n## Game ended.");
    if found {
        println!("- You win. GG bro.");
    } else {
        println!("- You lost. You ran out of tries. Take the L, bozo.");
    }
    println!("Hidden set was {:?}", set_hidden);
    println!();

    found
}

/// Execute the game where the human player tries to guess but helped by the engine.
pub fn game_assist(set_hidden: Vec<u32>) -> bool {
    // User game.
    
    // Settings
    let mut guesses: u32 = 1;
    let mut found: bool = false;
    let mut cheat_jojo: bool = false;
    let mut _bug: bool = false;

    // Settings - Bot
    let mut hint_history: HashMap<Vec<u32>, Hint> = HashMap::new();
    let mut set_combinations: Vec<Vec<u32>> = combinations_sets();
    let hint_combinations: Vec<Hint> = combinations_hints();

    while guesses <= MAX_TRIES && !found {
        let mut user_guess_correct: bool = false;
        let mut user_guess_set_string: String = String::new();
        let mut user_guess_set: Vec<u32> = Vec::new();
        println!("Guess {}.", guesses);
        // Engine recommendations.
            let t: Instant = Instant::now();
            // Calculate entropy of each possible given combination.
            if UI_SHOW == UiLevel::All {println!("- Assist - Entropy calculation on {} sets.", {set_combinations.len()});}
            let set_combinations_entropy: HashMap<Vec<u32>, f64> = all_set_entropy(&set_combinations, &hint_combinations);
            // Choose the best one, first if ex-aqueo.
            let set_combinations_entropy_max: (Vec<Vec<u32>>, f64) = max_entropy(set_combinations_entropy.clone());
            let d: Duration = t.elapsed();
        
            println!("- Assist - Found max entropy E = {:.3}. Recomended sets: {:?}", set_combinations_entropy_max.1, set_combinations_entropy_max.0);
            if DEBUG_LOG_TIME {println!("- Time elapsed calculating combinations: t = {:.2?}", d);}

        // User guess
        let t: Instant = Instant::now();

        while !user_guess_correct {
            std::io::stdin()
                .read_line(&mut user_guess_set_string)
                .expect("(!) - Couldn't read line.");

            user_guess_set_string = user_guess_set_string.trim().to_string();
            // Convert to vector
            user_guess_set = convert_input_set(user_guess_set_string.clone());
            
            
            // Validate input
            if user_guess_set.len() == SET_LENGTH {
                user_guess_correct = true;
            } else if DEBUG_ACTIVATED && (user_guess_set_string == String::from("jojo")) {
                // Cheat - Instant win.
                println!("Cheat - Instant win.");
                cheat_jojo = true;
                user_guess_correct = true;
            } else if DEBUG_ACTIVATED && (user_guess_set_string == String::from("omniscia")) {
                // Cheat - Autocorrect input.
                println!("Cheat - Auto-correct input.");
                user_guess_set = set_hidden.clone();
                user_guess_correct = true;
            } else {
                println!("(!) Warning - Invalid input; Input difference: {}", SET_LENGTH as i32 - user_guess_set.len() as i32);
                user_guess_set_string = String::new();
            }
        }

        // Comparing.
        let comparison_results: Hint = similarities(&set_hidden, &user_guess_set);
        
        // UI
        println!("- Set: {:?}, E = {}", user_guess_set, set_entropy(&user_guess_set, &set_combinations, &hint_combinations));
        println!("- Found hint: {}{} {}{} {}{}", comparison_results.exact, UI_HINTS.exact, comparison_results.exist, UI_HINTS.exist, comparison_results.null, UI_HINTS.null);

        // Guess and save the hint and set in history.
		hint_history.insert(user_guess_set, comparison_results.clone());
		

		// Create and filter the vector of possible combinations with history.
		if UI_SHOW == UiLevel::All {println!("- Refining combinations.");}
		set_combinations = combinations_sets_matching(&hint_history, &set_combinations);

        // Time to guess
        let d: Duration = t.elapsed();
        if DEBUG_LOG_TIME {println!("- Time elapsed waiting human: t = {:.2?}", d);}

        // End turn.
        found = (comparison_results.exact as usize == SET_LENGTH) | cheat_jojo;

        guesses += 1;
    } 
    println!("\n## Game ended.");
    if found {
        println!("- You win. GG bro.");
    } else {
        println!("- You lost. You ran out of tries. Take the L, bozo.");
    }
    println!("Hidden set was {:?}", set_hidden);
    println!();

    found
}
     