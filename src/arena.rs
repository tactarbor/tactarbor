use crate::Game;
use std::marker::PhantomData;
#[derive(Debug, Default)]
pub struct Arena<G: Game>(PhantomData<G>);
impl<G: Game> Arena<G> {
    pub fn play(&mut self, h: crate::Heuristic<G>) -> crate::Unit {
        let mut game = G::default();
        while let Some(action) = {
            let observation = game.observation();
            game.actions()
                .max_by_key(|action| h.eval(&observation, action))
        } {
            game.step(action);
        }
        game.outcome()
    }
}
