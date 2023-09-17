use ai::simulate_game;
use rayon::prelude::*;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use super_ttt::Game;
const REPETITION_TIMES: usize = 100_000;
fn main() {
    const REPETITION_TIMES: usize = 10_000;
    fn main() {
        println!("Going to implement monte carlo from scratch");
        let results = Arc::new(Mutex::new(HashMap::from([(0, 0), (-1, 0), (1, 0)])));
        (0..REPETITION_TIMES).into_par_iter().for_each(|_| {
            results
                .clone()
                .lock()
                .unwrap()
                .entry(simulate_game(Game::new()).1)
                .and_modify(|x| *x += 1);
        });
        {
            let results = results.lock().unwrap();
            println!(
                "First player wins: {}",
                results[&1] as f32 / REPETITION_TIMES as f32
            );
            println!(
                "Second player wins: {}",
                results[&-1] as f32 / REPETITION_TIMES as f32
            );
            println!("Ties: {}", results[&0] as f32 / REPETITION_TIMES as f32);
        }
    }
}
