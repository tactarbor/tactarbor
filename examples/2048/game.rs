use std::time::{SystemTime, UNIX_EPOCH};
use tactarbor::{Game, Unit};

use crate::action::A;
use crate::observation::{O, SIZE, apply};

/// A tile of this value wins the game.
const WIN_TILE: u32 = 2048;

pub struct G {
    pub cells: O,
    pub score: u32,
    rng: u64,
}

impl Default for G {
    fn default() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0)
            | 1;
        let mut board = G {
            cells: [[0; SIZE]; SIZE],
            score: 0,
            rng: seed,
        };
        board.spawn();
        board.spawn();
        board
    }
}

impl Game for G {
    type Observation = O;
    type Action = A;

    fn step(&mut self, action: Self::Action) {
        let (next, gained, moved) = apply(&self.cells, action);
        if moved {
            self.cells = next;
            self.score += gained;
            self.spawn();
        }
    }

    fn actions(&self) -> impl Iterator<Item = Self::Action> {
        let grid = self.cells;
        // Reaching a 2048 tile wins: offer no moves so play terminates.
        let won = self.won();
        [A::Up, A::Down, A::Left, A::Right]
            .into_iter()
            .filter(move |&dir| !won && apply(&grid, dir).2)
    }

    fn observation(&self) -> Self::Observation {
        self.cells
    }

    fn outcome(&self) -> Unit {
        Unit::from(if self.won() { 1.0 } else { 0.0 })
    }
}

impl G {
    fn won(&self) -> bool {
        self.cells.iter().flatten().any(|&v| v >= WIN_TILE)
    }

    fn spawn(&mut self) {
        let empties: Vec<(usize, usize)> = (0..SIZE)
            .flat_map(|r| (0..SIZE).map(move |c| (r, c)))
            .filter(|&(r, c)| self.cells[r][c] == 0)
            .collect();
        if empties.is_empty() {
            return;
        }
        let (r, c) = empties[self.next_rng() as usize % empties.len()];
        // 90% spawn a 2, 10% spawn a 4.
        self.cells[r][c] = if self.next_rng() % 10 == 0 { 4 } else { 2 };
    }

    fn next_rng(&mut self) -> u64 {
        let mut x = self.rng;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.rng = x;
        x
    }

    pub fn render(&self) {
        for row in &self.cells {
            let line: String = row
                .iter()
                .map(|&v| if v == 0 { ".".into() } else { v.to_string() })
                .map(|cell| format!("{cell:>5}"))
                .collect();
            println!("{line}");
        }
    }
}
