use crate::{Game, Unit};

pub enum Heuristic<G: Game> {
    Branch {
        predicate: fn(&G::Observation, &G::Action) -> bool,
        y: Box<Heuristic<G>>,
        n: Box<Heuristic<G>>,
    },
    Leaf {
        score: fn(&G::Observation, &G::Action) -> Unit,
    },
}

impl<G: Game> Heuristic<G> {
    pub fn eval(&self, observation: &G::Observation, action: &G::Action) -> Unit {
        match self {
            Heuristic::Branch { predicate, y, n } => {
                if predicate(observation, action) {
                    y.eval(observation, action)
                } else {
                    n.eval(observation, action)
                }
            }
            Heuristic::Leaf { score } => score(observation, action),
        }
    }
}
