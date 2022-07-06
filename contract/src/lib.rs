use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet};
// use near_sdk::env::log_str;
use near_sdk::serde::{Deserialize, Serialize};
// use near_sdk::{log, near_bindgen, env};
use near_sdk::{env, log, near_bindgen, AccountId, PanicOnDefault, Promise};
// use pretty_assertions::StrComparison;

// Define the default message
// const PUZZLE_NUMBER: u8 = 1;
// 5 N in yoctoNEAR
const PRIZE_AMOUNT: u128 = 5_000_000_000_000_000_000_000_000;

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum AnswerDirection {
    Across,
    Down,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CoordinatePair {
    x: u8,
    y: u8,
}

// #[derive(Serialize)]
// #[serde(crate = "near_sdk::serde")]
// pub struct UnsolvedPuzzles {
//     puzzles: Vec<JsonPuzzle>,
// }

// #[derive(Serialize, Deserialize)]
// #[serde(crate = "near_sdk::serde")]
// pub struct JsonPuzzle {
//     /// The human-readable (not in bytes) hash of the solution
//     solution_hash: String,
//     status: PuzzleStatus,
//     answer: Vec<Answer>,
// }

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Answer {
    num: u8,
    start: CoordinatePair,
    direction: AnswerDirection,
    length: u8,
    clue: String,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum PuzzleStatus {
    Unsolved,
    Solved { memo: String },
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Puzzle {
    status: PuzzleStatus,
    answer: Vec<Answer>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
    puzzles: LookupMap<String, Puzzle>,
    unsolved_puzzles: UnorderedSet<String>,
    // before
    // crossword_solution: String,
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            puzzles: LookupMap::new(b"c"),
            unsolved_puzzles: UnorderedSet::new(b"u"),
        }
    }
    // pub fn new(solution: String) -> Self {
    //     Self {
    //         // crossword_solution: solution,
    //     }
    // }
    pub fn new_puzzle(&mut self, solution_hash: String, answers: Vec<Answer>) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Only the owner may call this method"
        );
        let existing = self.puzzles.insert(
            &solution_hash,
            &Puzzle {
                status: PuzzleStatus::Unsolved,
                answer: answers,
            },
        );
        assert!(existing.is_none(), "Puzzle with that key already exists");
        self.unsolved_puzzles.insert(&solution_hash);
    }

    pub fn get_solution(&self, puzzle_index: u32) -> Option<String> {
        let mut index = 0;
        for puzzle_hash in self.unsolved_puzzles.iter() {
            if puzzle_index == index {
                return Some(puzzle_hash);
            }
            index += 1;
        }
        None
    }

    // pub fn get_unsolved_puzzles(&self) -> UnsolvedPuzzles {
    //     let solution_hashes = self.unsolved_puzzles.to_vec();
    //     let mut all_unsolved_puzzles = vec![];
    //     for hash in solution_hashes {
    //         let puzzle = self
    //             .puzzles
    //             .get(&hash)
    //             .unwrap_or_else(|| env::panic_str("ERR_LOADING_PUZZLE"));
    //         let json_puzzle = JsonPuzzle {
    //             solution_hash: hash,
    //             status: puzzle.status,
    //             answer: puzzle.answer,
    //         };
    //         all_unsolved_puzzles.push(json_puzzle)
    //     }
    //     UnsolvedPuzzles {
    //         puzzles: all_unsolved_puzzles,
    //     }
    // }

    pub fn get_puzzle_status(&self, solution_hash: String) -> Option<PuzzleStatus> {
        let puzzle = self.puzzles.get(&solution_hash);
        if puzzle.is_none() {
            return None;
        }
        Some(puzzle.unwrap().status)
    }

    pub fn submit_solution(&mut self, solution: String, memo: String) {
        let hashed_input = env::sha256(solution.as_bytes());
        let hashed_input_hex = hex::encode(&hashed_input);

        // Check to see if the hashed answer is among the puzzles
        let mut puzzle = self
            .puzzles
            .get(&hashed_input_hex)
            .expect("ERR_NOT_CORRECT_ANSWER");

        // Check if the puzzle is already solved. If it's unsolved, set the status to solved,
        //   then proceed to update the puzzle and pay the winner.
        puzzle.status = match puzzle.status {
            PuzzleStatus::Unsolved => PuzzleStatus::Solved { memo: memo.clone() },
            _ => {
                env::panic_str("ERR_PUZZLE_SOLVED");
            }
        };

        // Reinsert the puzzle back in after we modified the status:
        self.puzzles.insert(&hashed_input_hex, &puzzle);
        // Remove from the list of unsolved ones
        self.unsolved_puzzles.remove(&hashed_input_hex);

        log!(
            "Puzzle with solution hash {} solved, with memo: {}",
            hashed_input_hex,
            memo
        );

        // Transfer the prize money to the winner
        Promise::new(env::predecessor_account_id()).transfer(PRIZE_AMOUNT);
    }

    // pub fn get_solution(&self) -> String {
    //     self.crossword_solution.clone()
    // }

    // pub fn set_solution(&mut self, solution: String) {
    //     self.crossword_solution = solution
    // }

    // pub fn guess_solution(&self, solution: String) -> bool {
    //     let hash_bytes = env::sha256(solution.as_bytes());
    //     let hash_string = hex::encode(hash_bytes);

    //     if hash_string == self.crossword_solution {
    //         env::log_str("You guessed right!");
    //         return true;
    //     } else {
    //         env::log_str("Try again.");
    //         return false;
    //     }
    // }
}

// #[allow(unused_variables, unused_imports, unused_assignments)]
#[cfg(test)]
mod tests {
    // solution hash : d1a5cf9ad1adefe0528f7d31866cf901e665745ff172b96892693769ad284010

    use super::*;
    // use pretty_assertions::assert_eq;
    // use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, AccountId};
    // use near_sdk::{testing_env, AccountId, VMContext};

    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    fn get_answers() -> Vec<Answer> {
        vec![
            Answer {
                num: 1,
                start: CoordinatePair { x: 2, y: 1 },
                direction: AnswerDirection::Across,
                length: 4,
                clue: "Native token".to_string(),
            },
            Answer {
                num: 1,
                start: CoordinatePair { x: 2, y: 1 },
                direction: AnswerDirection::Down,
                length: 7,
                clue: "Name of the specs/standards site is ______.io".to_string(),
            },
            Answer {
                num: 2,
                start: CoordinatePair { x: 5, y: 1 },
                direction: AnswerDirection::Down,
                length: 3,
                clue: "DeFi site on NEAR is ___.finance".to_string(),
            },
            Answer {
                num: 4,
                start: CoordinatePair { x: 0, y: 7 },
                direction: AnswerDirection::Across,
                length: 7,
                clue: "DeFi decentralizes this".to_string(),
            },
        ]
    }

    #[test]
    fn debug_get_hash() {
        testing_env!(VMContextBuilder::new().build());

        let debug_solution = "paras rainbowbridge mintbase yoctonear cli";
        let debug_hash_bytes = env::sha256(debug_solution.as_bytes());
        let debug_hash_string = hex::encode(debug_hash_bytes);

        println!("{:?}", debug_hash_string);
    }

    #[test]
    #[should_panic(expected = "ERR_NOT_CORRECT_ANSWER")]
    fn check_submit_solution_failure() {
        let alice = AccountId::new_unchecked("alice.testnet".to_string());
        let context = get_context(alice.clone());
        testing_env!(context.build());

        let mut contract = Contract::new(alice);
        let answers = get_answers();
        contract.new_puzzle(
            "d1a5cf9ad1adefe0528f7d31866cf901e665745ff172b96892693769ad284010".to_string(),
            answers,
        );
        contract.submit_solution("wrong answer here".to_string(), "my memo".to_string());
    }

    #[test]
    fn check_submit_success() {
        let alice = AccountId::new_unchecked("alice.testnet".to_string());
        let context = get_context(alice.clone());
        testing_env!(context.build());

        let mut contract = Contract::new(alice);

        let answers = get_answers();

        contract.new_puzzle(
            "d1a5cf9ad1adefe0528f7d31866cf901e665745ff172b96892693769ad284010".to_string(),
            answers,
        );

        contract.submit_solution(
            "paras rainbowbridge mintbase yoctonear cli".to_string(),
            "my memo".to_string(),
        );
    }

    // #[test]
    // fn check_guess_solution() {
    //     let alice = AccountId::new_unchecked("alice.testnet".to_string());
    //     let context = get_context(alice);
    //     testing_env!(context.build());

    //     let contract = Contract::new(
    //         "69c2feb084439956193f4c21936025f14a5a5a78979d67ae34762e18a7206a0f".to_string(),
    //     );

    //     let mut guess_result = contract.guess_solution("wrong answer here".to_string());
    //     assert!(!guess_result, "Expected a failure from the wrong guess");
    //     assert_eq!(get_logs(), ["Try again."], "Expected a failure log");

    //     guess_result = contract.guess_solution("near nomicon ref finance".to_string());
    //     assert!(guess_result, "Expected the correct answer to return true.");
    //     assert_eq!(
    //         get_logs(),
    //         ["Try again.", "You guessed right!"],
    //         "Expected a successful log after the previous failed log."
    //     );

    //     assert_eq!(
    //         "69c2feb084439956193f4c21936025f14a5a5a78979d67ae34762e18a7206a0f".to_string(),
    //         contract.get_solution()
    //     )
    // }
}
