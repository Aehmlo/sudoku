//! Constructs relevant to implementating game logic.

use Difficulty;
use Element;
use Generate;
use Grid;
use Point;
use Solve;
use Sudoku;

/// Represents an in-progress game.
#[derive(Debug)]
pub struct Game {
    problem: Sudoku,
    /// The current state of the game.
    pub current: Sudoku,
    solution: Sudoku,
    /// The number of moves performed so far.
    pub moves: usize,
}

impl Game {
    /// Creates a new game with a sudoku of the specified order and difficulty.
    pub fn new(order: u8, difficulty: Difficulty) -> Self {
        let problem = Sudoku::generate(order, difficulty);
        let current = problem.clone();
        let solution = problem.solution().unwrap();
        Self {
            problem,
            current,
            solution,
            moves: 0,
        }
    }
    /// Returns the points relevant to the selection (for e.g. highlighting).
    ///
    /// The order of these points is intentionally left unspecified.
    pub fn relevant_points(&self, point: Point) -> Vec<Point> {
        self.problem.group_indices(point)
    }
    /// Whether the proposed change is correct (according to the stored solution).
    pub fn insertion_is_correct(&self, point: Point, value: Element) -> bool {
        self.solution[point] == Some(value)
    }
    /// Updates the game model to reflect the insertion.
    ///
    /// # Notes
    /// No validation of the insertion is made; use
    /// [`insertion_is_valid`](#method.insertion_is_valid) to double-check the change before
    /// insertion (and check whether invalid insertions should be allowed) before commiting.
    pub fn insert(&mut self, point: Point, value: Element) {
        self.current.substitute(point, Some(value));
        self.moves += 1;
    }
    /// Removes the indexed element from the puzzle, returning the old value (if applicable).
    pub fn remove(&mut self, point: Point) -> Option<Element> {
        self.moves += 1;
        let value = self.current[point];
        self.current.substitute(point, None);
        value
    }
    /// Returns all points associated with this game.
    pub fn points(&self) -> Vec<Point> {
        self.current.points()
    }
}

/// Tools for managing the user's preferences.
pub mod config {
    use Difficulty;

    /// Monolithic struct containing all user-configurable preferences.
    #[derive(Clone, Copy, Debug, Default)]
    pub struct Preferences {
        behavior: Behavior,
        generation: Generation,
    }

    /// Specifies in-game behavior, such as what to do when the user answers incorrectly.
    #[derive(Clone, Copy, Debug)]
    pub struct Behavior {
        /// Whether the user should be allowed to answer incorrectly.
        pub allow_incorrect_answers: bool,
    }

    impl Default for Behavior {
        fn default() -> Self {
            Self {
                allow_incorrect_answers: false,
            }
        }
    }

    /// Specifies puzzle generation behavior, such as the default sudoku difficulty and order.
    #[derive(Clone, Copy, Debug)]
    pub struct Generation {
        /// The default puzzle order.
        pub default_order: u8,
        /// The default puzzle difficulty.
        pub default_difficulty: Difficulty,
    }

    impl Default for Generation {
        fn default() -> Self {
            Self {
                default_order: 3,
                default_difficulty: Difficulty::Intermediate,
            }
        }
    }
}