use rand::{self, seq::SliceRandom};
use std::collections::{HashMap, HashSet};
use super_ttt::{errors, Game, GameState, Player, Square};

struct TreeData {
    visit_count: usize,
    total_score: i8,
    // TODO: Differentiate between explored and unexplored
    // by comparing the length of this set to total possible
    // children
    children: HashSet<Game>,
}

impl Default for TreeData {
    fn default() -> Self {
        Self {
            visit_count: 1,
            total_score: 0,
            children: HashSet::new(),
        }
    }
}

type Move = (usize, usize, usize, usize);

/// Monte Carlo Tree Search
struct MTCS {
    tree_data: HashMap<Game, TreeData>,
}
fn apply_move(
    (board_row, board_col, cell_row, cell_col): (usize, usize, usize, usize),
    mut game: Game,
) -> Result<Game, errors::InvalidMoveError> {
    return game.make_move(board_row, board_col, cell_row, cell_col);
}

pub fn get_valid_moves(node: Game) -> Vec<Move> {
    let mut valid_moves = Vec::new();
    let mut row_range = 0..3;
    let mut col_range = 0..3;

    if let Some(last_move_cord) = node.last_move_cords {
        if node.boards[last_move_cord.0][last_move_cord.1].get_winner() == GameState::InProgress {
            row_range = last_move_cord.0..last_move_cord.0 + 1;
            col_range = last_move_cord.1..last_move_cord.1 + 1;
        }
    }

    for i in row_range {
        for j in col_range.clone() {
            for k in 0..3 {
                for l in 0..3 {
                    if node.boards[i][j].squares[k][l] == Square::Empty {
                        valid_moves.push((i, j, k, l));
                    }
                }
            }
        }
    }

    valid_moves
}
pub fn simulate_game(node: Game) -> (Vec<Game>, i8) {
    let current_node = node;
    let mut visited_nodes = vec![node];
    while current_node.get_winner() == GameState::InProgress {
        visited_nodes.push(
            apply_move(
                *get_valid_moves(current_node)
                    .choose(&mut rand::thread_rng())
                    .expect("No valid moves"),
                current_node,
            )
            .expect("Invalid move generated"),
        );
    }
    visited_nodes.push(current_node);
    return (
        visited_nodes,
        match current_node.get_winner() {
            GameState::Tie => 0,
            GameState::Winner(player) => {
                if player == Player::X {
                    1
                } else {
                    -1
                }
            }
            GameState::InProgress => unreachable!(),
        },
    );
}
impl MTCS {
    pub fn new() -> Self {
        MTCS {
            tree_data: HashMap::new(),
        }
    }
    /// Returns another game state where the best move is made
    pub fn search(&mut self, root_node: Game, budget: usize) -> Game {
        if root_node.get_winner() != GameState::InProgress || self.is_fully_expanded(root_node) {
            panic!("This should never happen");
        }
        // Currently VERY broken
        for search_iteration in 0..budget {
            // TODO: While loop to continuously select children until
            // they are not fully expanded. Will need to handle the case where
            // all child nodes have been explored...
            // Selection phase.

            let selected_node = self.select_child(root_node);
            self.tree_data
                .entry(root_node)
                .or_default()
                .children
                .insert(selected_node);
            // Expansion phase.
            // We choose a random, unexplored move to try

            let new_node = self.tree_data.entry(selected_node).or_default();
            let current_node = get_valid_moves(selected_node)
                .iter()
                .map(|move_| {
                    apply_move(*move_, selected_node.clone()).expect("invalid moves were generated")
                })
                .filter(|new_state| new_node.children.contains(new_state))
                .next()
                // selected_node shouldn't be fully expanded
                // so this should never panic
                .expect(format!("{:?} {selected_node}", search_iteration).as_ref());
            new_node.children.insert(current_node.clone());

            // Simulation phase
            let (visited_nodes, final_result) = simulate_game(current_node);
            // Back propagation phase
            // 1. Update visits
            let mut iterator = visited_nodes.iter().peekable();
            while let Some(&state) = iterator.next() {
                let parent = self.tree_data.entry(state).or_default();
                if let Some(&child) = iterator.peek() {
                    parent.children.insert(*child);
                };
                parent.visit_count += 1;
            }
            // 2. Update statistics
            // We need this step unless you only want to attach
            // the final score to the final, terminal state
            for node in visited_nodes {
                self.tree_data
                    .entry(node)
                    .and_modify(|entry| entry.total_score += final_result);
            }
            // self.tree_data.entry(selected_node).or_default().visit_count += 1;
        }
        // get best child node

        return *self.tree_data[&root_node]
            .children
            .iter()
            // TODO: Handle which is best is for which player
            .max_by_key(|child| {
                self.tree_data[child].total_score / self.tree_data[child].visit_count as i8
            })
            .unwrap();
    }
    fn is_fully_expanded(&self, game: Game) -> bool {
        return game.get_winner() != GameState::InProgress
            || (get_valid_moves(game).len()
                == self
                    .tree_data
                    .get(&game)
                    .map(|data| data.children.len())
                    .unwrap_or(0));
    }
    fn ucb1(&self, node: Game, parent: Game) -> f64 {
        match self.tree_data.get(&node) {
            Some(data) => {
                if self.is_fully_expanded(node) {
                    return -f64::INFINITY;
                }
                let exploitation_term = data.total_score as f64 / data.visit_count as f64;
                let exploration_term = ((2.0
                    * (self.tree_data[&parent].visit_count as f64).log2())
                    / data.visit_count as f64)
                    .sqrt();
                return exploitation_term + exploration_term;
            }
            None => f64::INFINITY,
        }
    }
    fn select_child(&self, node: Game) -> Game {
        // Select the most promising one based on UCB.
        // I have to do this magic instead of .max_by_key
        // because f64 don't implement Ord (stupid NaN)
        get_valid_moves(node)
            .iter()
            .reduce(|a, b| {
                if self.ucb1(apply_move(*a, node.clone()).unwrap(), node)
                    > self.ucb1(apply_move(*b, node.clone()).unwrap(), node)
                {
                    return a;
                } else {
                    return b;
                };
            })
            .and_then(|&(board_row, board_col, cell_row, cell_col)| {
                node.clone()
                    .make_move(board_row, board_col, cell_row, cell_col)
                    .ok()
            })
            .unwrap()
    }
}

const BUDGET: usize = 100;

fn main() {
    println!("Going to implement monte carlo from scratch");
    let mut game = Game::new();
    let mut mtcs = MTCS::new();
    game.make_move(1, 1, 1, 1).unwrap();
    println!("{}", game);
    while game.get_winner() == GameState::InProgress {
        game = mtcs.search(game, BUDGET);
        println!("{}", game);
    }
}