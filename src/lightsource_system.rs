use super::{gamelog::GameLog, LightSourceState, RunState, SufferDamage};
use specs::prelude::*;

pub struct LightSourceSystem {}

impl<'a> System<'a> for LightSourceSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, LightSourceState>,
        ReadExpect<'a, Entity>, // The player
        ReadExpect<'a, RunState>,
        WriteExpect<'a, GameLog>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut lightsource_state, player_entity, runstate,  mut log) =
            data;

        for (entity, mut state) in (&entities, &mut lightsource_state).join() {
            let mut proceed = false;

            match *runstate {
                RunState::PlayerTurn => {
                    if entity == *player_entity {
                        proceed = true;
                    }
                }
                RunState::MonsterTurn => {
                    if entity != *player_entity {
                        proceed = false;
                    }
                }
                _ => proceed = false,
            }

            if proceed {
                state.fuel -= 1;
                if state.fuel < 1 {
                    if entity == *player_entity {
                                     log.entries.push("Your ligth has gone out!".to_string());
                                 }
                }
                 
            }
        }
    }
}
