// MASTER MIND
// Bot search.





// Import from Main.
use crate::{Duration, HashMap, Instant, UiLevel, DEBUG_LOG_TIME, MAX_TRIES, POOL_SIZE, SET_LENGTH, UI_HINTS, UI_SHOW};
// Import from Checks.
use crate::{checks::{similarities, Hint}};

/// Structure that stores infos on next probabilities and bits
pub struct SetScore {
	pub probability: f64,
	pub bits: f64
}

/// Generate all possible combinations of set, according to main constant's.
/// q(combinations) = pool_size ^ set_length.
pub fn combinations_sets() -> Vec<Vec<u32>> {
	fn combinations_sets_inner(mut iter_global: usize) -> Vec<Vec<u32>> {
		let mut combinations_send: Vec<Vec<u32>> = Vec::new();

		if iter_global < SET_LENGTH {
			iter_global += 1;
			let combinations_received: Vec<Vec<u32>> = combinations_sets_inner(iter_global);
			// println!("\n====================Iter {iter_global} - NORMAL MODE=======================");
			
			for combination in combinations_received {
				for iter_local in 1..=POOL_SIZE {
					let mut combination_temp: Vec<u32> = combination.to_vec();
					combination_temp.push(iter_local);
					combinations_send.push(combination_temp);
				}	
			}
		} else {
			// println!("\n====================Iter {iter_global} - INIT MOD=========================");
			for iter_local in 1..=POOL_SIZE {
				let combination: Vec<u32> = vec![iter_local];
				combinations_send.push(combination);
			}
		}
		
		// print!("{:?} ", combinations_send);

		combinations_send
	}

	let combinations: Vec<Vec<u32>> = combinations_sets_inner(1);

	combinations
}

/// Generate all possible combinations of hints given, *Exact* (2), *Exist* (1), *Non-exist* (0).
/// q(combinations) = 3 ^ set_length.
pub fn combinations_hints() -> Vec<Hint> {
	// Generate all combinations
	fn combinations_sets_inner(mut iter_global: usize) -> Vec<Vec<u32>> {
		let mut combinations_send: Vec<Vec<u32>> = Vec::new();

		if iter_global < SET_LENGTH {
			iter_global += 1;
			let combinations_received: Vec<Vec<u32>> = combinations_sets_inner(iter_global);
			//println!("\n====================Iter {iter_global} - NORMAL MODE=======================");
			
			for combination in combinations_received {
				for iter_local in 0..=2 {
					let mut combination_temp: Vec<u32> = combination.to_vec();
					combination_temp.push(iter_local);
					combinations_send.push(combination_temp);
				}	
			}
		} else {
			//println!("\n====================Iter {iter_global} - INIT MOD=========================");
			for iter_local in 0..=2 {
				let combination: Vec<u32> = vec![iter_local];
				combinations_send.push(combination);
			}
		}
		
		//print!("{:?} ", combinations_send);

		combinations_send
	}
	let combinations: Vec<Vec<u32>> = combinations_sets_inner(1);

	// Convert to Hint struct
	let mut combinations_hints: Vec<Hint> = Vec::new();
	for hint_vec in combinations {
		let mut hint_hint: Hint = Hint {
			exact: 0, 
			exist: 0, 
			null: 0
		};
		for value in hint_vec {
			match value {
				2 => hint_hint.exact += 1,
				1 => hint_hint.exist += 1,
				_ => hint_hint.null += 1
			}
		}

		if !combinations_hints.contains(&hint_hint) {
			combinations_hints.push(hint_hint);
		}
	}

	combinations_hints
}

/// Filter from all the possible sets those who *match previous guesses*.
pub fn combinations_sets_matching(hint_history: &HashMap<Vec<u32>, Hint>, set_combinations: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
	let mut filtered_combinations: Vec<Vec<u32>> = Vec::new();
	
	// Check for all hints ever given...
	for set in set_combinations {
		let mut accepted: bool = true;
		for (set_historic, hint_historic) in hint_history {
			if similarities(set, set_historic) != *hint_historic {
				accepted = false;
			}
		}
		if accepted {
			filtered_combinations.push(set.clone());
		}
	}

	filtered_combinations
}

/// Find, for a given set, the number by set of hint-set that would correspond 
/// Input: set = Given set, set_combinations = vector of all possible set,
/// Returns: HashMap of the quantity of each hint-set.
pub fn set_hint_quantities(set_origin: &Vec<u32>, set_combinations: &Vec<Vec<u32>>, set_hints: &Vec<Hint>) -> HashMap<Hint, u32> {
	// Init all hints
	let mut hint_quantities: HashMap<Hint, u32> = HashMap::new();
	for set_hint in set_hints {
		hint_quantities.insert(set_hint.clone(), 0u32);
	}
	// Go throught all sets
	for set in set_combinations {
		// Compare
		let hint: Hint = similarities(&set_origin, &set);
		// Add to map
		hint_quantities.insert(hint.clone(), hint_quantities[&hint] + 1);
	}

	hint_quantities
}

/// Compute for each hint-set its probability of happening and and bits of a set given its quantity map, by going trought all possible set combinations.
pub fn set_hint_score(hint_quantities: HashMap<Hint, u32>) -> HashMap<Hint, SetScore> {
	let mut hint_probabilities: HashMap<Hint, SetScore> = HashMap::new();
	// Sum.
	let mut total_sum: u32 = 0;
	for (_, quantity) in &hint_quantities {
		total_sum += quantity;
	}
	//println!("sum: {}", total_sum);
	// Proba.
	for (hint, quantity) in hint_quantities {
		let probability: f64 = quantity as f64 / total_sum as f64;
		let mut bits: f64 = 0.0_f64;
		if probability != 0.0_f64 {
			bits = -probability.log2();
		} else {
			bits = 0.0_f64;
		}
		let score: SetScore = SetScore {
			probability,
			bits
		};
		hint_probabilities.insert(hint, score);
	}

	hint_probabilities
}

/// Compute the entropy of a set, given its hint score. Allow a general score.
pub fn set_hint_score_entropy(hints_score: HashMap<Hint, SetScore>) -> f64 {
	let mut entropy: f64 = 0.0f64;
	for (_, scores) in hints_score {
		entropy += scores.probability * scores.bits;
	}

	entropy
}

/// # Entropy of 1 set.
/// Comprehensive function to compute entropy of a given set.
pub fn set_entropy(set: &Vec<u32>, combinations_sets: &Vec<Vec<u32>>, combinations_hint: &Vec<Hint>) -> f64 {
	// Hint quantities.
    let hint_quantities: HashMap<Hint, u32> = set_hint_quantities(&set, &combinations_sets, &combinations_hint);
    /*
	println!("Hint quantities:");
    for (hint, weight) in &hint_quantities {
        println!("- q = {} -> {}{} {}{} {}{}", weight, hint.exact, "AC", hint.exist, "IS", hint.null, "NO");
    }
	*/
    // println!("q: {}", _hint_quantities.len());
    // Hint probability.
    let hint_score: HashMap<Hint, SetScore> = set_hint_score(hint_quantities);
    /*
	println!("Hint probability:");
	for (hint, score) in &hint_score {
        println!("- p = {:.2}%; b = {:.2}bits -> {}{} {}{} {}{}", score.probability * 100f64, score.bits,hint.exact, CORRECT_PLACEMENT, hint.exist, CORRECT_VALUE, hint.null, CORRECT_NON);
    }
	*/
    // Set overhaul score.
    let entropy = set_hint_score_entropy(hint_score);
	/*
	println!("Entropy:");
    println!("- E = {:.5}bits", {entropy});
	 */
	entropy
}

/// # Entropy of combination of set.
/// Compute entropy of all possible combinations given and returns a sorted HashMap.
pub fn all_set_entropy(combinations_sets: &Vec<Vec<u32>>, combinations_hint: &Vec<Hint>) -> HashMap<Vec<u32>, f64> {
	let mut entropies: HashMap<Vec<u32>, f64> = HashMap::new();
	for set in combinations_sets {
		entropies.insert(set.clone(), set_entropy(set, combinations_sets, combinations_hint));
	}

	entropies
}

/// Find and return a tuple of the maximum entropy and the sets reaching this entropy.
pub fn max_entropy(entropy_map: HashMap<Vec<u32>, f64>) -> (Vec<Vec<u32>>, f64) {
	let mut entropy_top_set: Vec<Vec<u32>> = Vec::new();
	let mut entropy_top: f64 = 0.0;
	// Find maximum value.
	for (_, entropy) in entropy_map.clone() {
        if entropy > entropy_top {
            entropy_top = entropy;
        }
    }
	// Link maximum value to actual set.
    for (set, entropy) in entropy_map {
        if entropy == entropy_top {
            entropy_top_set.push(set);
        }
    }

	(entropy_top_set, entropy_top)
}

/// # Bot main body.
/// This function runs the main loop of the "auto-search". Returns 0 if the bot didn't manage to find, else the number of guesses it took.
pub fn game_robot(set_hidden: Vec<u32>) -> u32 {
	println!("\n## Robot automated game. Set UI_SHOW to change what you see.");
	
	// Loop until found or run out of guesses.
	let mut guess_count: u32 = 1;
	let mut found: bool = false;
	let mut bug: bool = false;
	let mut hint_history: HashMap<Vec<u32>, Hint> = HashMap::new();
	let mut set_combinations: Vec<Vec<u32>> = combinations_sets();
	let hint_combinations: Vec<Hint> = combinations_hints();

	while guess_count <= MAX_TRIES && !found && !bug {
		if UI_SHOW == UiLevel::All {println!("Guess {}.", guess_count);}
		let t1: Instant = Instant::now();
		// Calculate entropy of each possible given combination.
		if UI_SHOW == UiLevel::All {println!("- Entropy calculation on {} sets.", {set_combinations.len()});}
		let set_combinations_entropy: HashMap<Vec<u32>, f64> = all_set_entropy(&set_combinations, &hint_combinations);
		// Choose the best one, first if ex-aqueo.
		let set_combinations_entropy_max: (Vec<Vec<u32>>, f64) = max_entropy(set_combinations_entropy.clone());
		let set_choosen: Vec<u32>;
		if set_combinations_entropy_max.0.len() > 0 {
			set_choosen = set_combinations_entropy_max.0[0].clone();
		} else {
			bug = true;
			panic!("(X) - Error, empty combination vector.");
		}
		// Guess and save the hint and set in history.
		if UI_SHOW == UiLevel::All {println!("- Guessing {:?}, E = {}.", set_choosen, set_combinations_entropy[&set_choosen]);}
		let hint_given: Hint = similarities(&set_hidden, &set_choosen);
		if hint_given.exact == 4 {
			found = true;
		}
		if UI_SHOW == UiLevel::All {println!("- Found hint: {}{} {}{} {}{}.", hint_given.exact, UI_HINTS.exact, hint_given.exist, UI_HINTS.exist, hint_given.null, UI_HINTS.null);}
		hint_history.insert(set_choosen, hint_given);
		

		// Create and filter the vector of possible combinations with history.
		if UI_SHOW == UiLevel::All {println!("- Refining combinations.");}
		set_combinations = combinations_sets_matching(&hint_history, &set_combinations);

		// Count elapsed time
		let d1: Duration  = t1.elapsed();
		if DEBUG_LOG_TIME {println!("- Time elapsed: t = {:.2?}", d1);}

		guess_count += 1;
	}
	// Game end
	if found {
		guess_count -= 1;
		if UI_SHOW == UiLevel::All {println!("Victory ! in {} guesses.", guess_count);}
	} else if bug {
		if UI_SHOW == UiLevel::All {println!("(!) - Something wrong happend; no more info.");}
	} else {
		if UI_SHOW == UiLevel::All {println!("Loose ! after {} guesses.", guess_count);}
		guess_count = 0;
	}

	guess_count
}