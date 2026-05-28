use tactarbor::{Heuristic, Unit};

use crate::action::A;
use crate::game::G;
use crate::observation::{O, SIZE, apply, empty_count};

fn board_is_full(grid: &O, _action: &A) -> bool {
    empty_count(grid) <= 4
}

/// Prefer moves that keep the board open.
fn keep_open(grid: &O, action: &A) -> Unit {
    let (next, _gained, _moved) = apply(grid, *action);
    Unit::from(empty_count(&next) as f64 / (SIZE * SIZE) as f64)
}

/// When the board is crowded, prefer moves that merge tiles, breaking ties by
/// how open the board is afterwards.
fn merge_then_open(grid: &O, action: &A) -> Unit {
    let (next, gained, _moved) = apply(grid, *action);
    let merged = if gained > 0 { 1.0 } else { 0.0 };
    let open = empty_count(&next) as f64 / (SIZE * SIZE) as f64;
    Unit::from(0.7 * merged + 0.3 * open)
}

pub fn strategy() -> Heuristic<G> {
    Heuristic::Branch {
        predicate: board_is_full,
        y: Box::new(Heuristic::Leaf {
            score: merge_then_open,
        }),
        n: Box::new(Heuristic::Leaf { score: keep_open }),
    }
}
