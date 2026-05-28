mod action;
mod game;
mod observation;
mod strategy;

use tactarbor::{Arena, Game};

use crate::game::G;
use crate::strategy::strategy;

fn main() {
    // Play a full game move by move so we can watch it. This is the same
    // greedy selection loop that Arena::play runs internally, unrolled here
    // only so we can render the final board.
    let heuristic = strategy();
    let mut game = G::default();
    let mut moves = 0u32;
    while let Some(action) = {
        let observation = game.observation();
        game.actions()
            .max_by_key(|action| heuristic.eval(&observation, action))
    } {
        game.step(action);
        moves += 1;
    }

    game.render();
    let max = game.cells.iter().flatten().copied().max().unwrap_or(0);
    println!();
    println!(
        "moves: {moves}   score: {}   max tile: {max}   outcome: {:.3}",
        game.score,
        f64::from(game.outcome()),
    );

    // The same thing through the library's one-call driver.
    let outcome = Arena::<G>::default().play(strategy());
    println!("arena outcome: {:.3}", f64::from(outcome));
}
