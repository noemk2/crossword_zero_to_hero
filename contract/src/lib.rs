use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::serde::{Deserialize, Serialize};
// use near_sdk::{log, near_bindgen, env};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};
// use pretty_assertions::StrComparison;

// Define the default message
// const PUZZLE_NUMBER: u8 = 1;

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


    // pub fn get_solution(&self) -> String {
    //     self.crossword_solution.clone()
    // }

    // pub fn get_puzzle_number(&self) -> u8 {
    //     PUZZLE_NUMBER
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

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;
    // use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};
    // use near_sdk::{testing_env, AccountId, VMContext};

    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    #[test]
    fn debug_get_hash() {
        testing_env!(VMContextBuilder::new().build());

        let debug_solution = "near nomicon ref finance";
        let debug_hash_bytes = env::sha256(debug_solution.as_bytes());
        let debug_hash_string = hex::encode(debug_hash_bytes);

        println!("{:?}", debug_hash_string);
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
