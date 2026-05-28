use serde::Serialize;

use crate::Unit;

pub trait Game: Default {
    type Observation: Serialize;
    type Action: Serialize + Clone;
    fn step(&mut self, action: Self::Action);
    fn actions(&self) -> impl Iterator<Item = Self::Action>;
    fn observation(&self) -> Self::Observation;
    fn outcome(&self) -> Unit;
}
