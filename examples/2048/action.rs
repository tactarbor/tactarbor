use serde::Serialize;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum A {
    Up,
    Down,
    Left,
    Right,
}
