# tactarbor

A tiny, dependency-free Rust framework for playing sequential decision games
with a decision-tree heuristic policy. The name reads as *tactic* + *arbor* (a
tree) — a tree of tactics.

You describe a game, build a heuristic tree that scores candidate moves, and
tactarbor plays it greedily: each turn it takes the highest-scoring action until
the game ends.

## Concepts

- **`Game`** — the trait you implement for your game or environment. It has an
  associated `Observation` (what the policy sees) and `Action` type, and
  requires `Default` to build the starting state. `step` applies an action,
  `actions` lists the legal moves (an **empty iterator means the game is over**),
  `observation` returns the current view, and `outcome` reports the final result
  as a `Unit`.
- **`Heuristic`** — a decision tree. A `Branch` routes on a
  `predicate(&observation, &action)` into a yes/no subtree; a `Leaf` scores an
  `(observation, action)` pair. `eval` walks the tree to a leaf and returns that
  score. Predicates and scorers are plain `fn` pointers.
- **`Unit`** — an `f64` newtype constrained to `[0, 1]` (the constructor rejects
  out-of-range values and NaN). It is totally ordered (`Ord` via `total_cmp`),
  so scores work directly as `max`/sort keys.
- **`Arena`** — the driver. `Arena::<MyGame>::default().play(heuristic)` builds a
  fresh game, plays it to completion, and returns the final `outcome`.

## Example

A minimal game: start at 0, add 1 or 2 each turn, and try to land exactly on 10.

```rust
use tactarbor::{Arena, Game, Heuristic, Unit};

#[derive(Default)]
struct Climb {
    position: u32,
}

impl Game for Climb {
    type Observation = u32;
    type Action = u32;

    fn step(&mut self, add: u32) {
        self.position += add;
    }

    fn actions(&self) -> impl Iterator<Item = u32> {
        // No moves once we reach the target — this ends the game.
        let stuck = self.position >= 10;
        [1, 2].into_iter().filter(move |_| !stuck)
    }

    fn observation(&self) -> u32 {
        self.position
    }

    fn outcome(&self) -> Unit {
        // Reward landing as close to 10 as possible.
        let distance = (self.position as i32 - 10).unsigned_abs();
        Unit::from(1.0 - (f64::from(distance) / 10.0).min(1.0))
    }
}

/// Score a move by where it would land, ruling out overshooting 10.
fn toward_ten(position: &u32, add: &u32) -> Unit {
    let next = position + add;
    Unit::from(if next <= 10 { f64::from(next) / 10.0 } else { 0.0 })
}

fn main() {
    let heuristic = Heuristic::Leaf { score: toward_ten };
    let outcome = Arena::<Climb>::default().play(heuristic);
    println!("outcome = {}", f64::from(outcome)); // 1.0 — lands exactly on 10
}
```

### Decision trees

Combine leaves with `Branch` to switch tactics based on the current state — the
namesake "arbor":

```rust
// Illustrative: predicate and scorers are your own `fn`s.
let heuristic = Heuristic::Branch {
    predicate: board_is_crowded,                      // fn(&Obs, &Action) -> bool
    y: Box::new(Heuristic::Leaf { score: merge_tiles }),
    n: Box::new(Heuristic::Leaf { score: keep_open }),
};
```

## How it works

`Arena::play` is a greedy, one-ply loop: for the current observation it
evaluates the heuristic against every legal action, takes the best one, steps the
game, and repeats until `actions` is empty — then returns `outcome`.

There is no lookahead or search (no minimax, no MCTS); all of the tactical
judgment lives in the heuristic. A search layer could be built on top of the same
`Game` trait.

## Status

Early and experimental (`0.1.0`), no dependencies, Rust 2024 edition.
