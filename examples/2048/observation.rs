use crate::action::A;

pub const SIZE: usize = 4;
pub type O = [[u32; SIZE]; SIZE];

/// Slide and merge one line toward index 0, returning the new line and the
/// points gained. Each tile can take part in at most one merge per move.
fn merge_line(line: [u32; SIZE]) -> ([u32; SIZE], u32) {
    let values: Vec<u32> = line.into_iter().filter(|&v| v != 0).collect();
    let mut out = [0u32; SIZE];
    let mut gained = 0;
    let mut write = 0;
    let mut read = 0;
    while read < values.len() {
        if read + 1 < values.len() && values[read] == values[read + 1] {
            let merged = values[read] * 2;
            out[write] = merged;
            gained += merged;
            read += 2;
        } else {
            out[write] = values[read];
            read += 1;
        }
        write += 1;
    }
    (out, gained)
}

/// The four cell coordinates of a line, ordered from the destination edge
/// inward, so that merging toward index 0 corresponds to moving in `dir`.
fn line_coords(dir: A, index: usize) -> [(usize, usize); SIZE] {
    let mut coords = [(0, 0); SIZE];
    for (step, coord) in coords.iter_mut().enumerate() {
        *coord = match dir {
            A::Left => (index, step),
            A::Right => (index, SIZE - 1 - step),
            A::Up => (step, index),
            A::Down => (SIZE - 1 - step, index),
        };
    }
    coords
}

/// Apply a move to a grid, returning the resulting grid, points gained, and
/// whether anything actually moved.
pub fn apply(grid: &O, dir: A) -> (O, u32, bool) {
    let mut out = *grid;
    let mut gained = 0;
    for index in 0..SIZE {
        let coords = line_coords(dir, index);
        let mut line = [0u32; SIZE];
        for (slot, &(r, c)) in coords.iter().enumerate() {
            line[slot] = grid[r][c];
        }
        let (merged, points) = merge_line(line);
        gained += points;
        for (&(r, c), &value) in coords.iter().zip(merged.iter()) {
            out[r][c] = value;
        }
    }
    (out, gained, out != *grid)
}

pub fn empty_count(grid: &O) -> usize {
    grid.iter().flatten().filter(|&&v| v == 0).count()
}
