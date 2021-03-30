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
        WriteStorage<'a, SufferDamage>,
        WriteExpect<'a, GameLog>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut lightsource_state, player_entity, runstate, mut inflict_damage, mut log) =
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
                    //     match clock.state {
                    //         HungerState::WellFed => {
                    //             clock.state = HungerState::Normal;
                    //             clock.duration = 200;
                    //             if entity == *player_entity {
                    //                 log.entries.push("You are no longer well fed.".to_string());
                    //             }
                    //         }
                    //         HungerState::Normal => {
                    //             clock.state = HungerState::Hungry;
                    //             clock.duration = 200;
                    //             if entity == *player_entity {
                    //                 log.entries.push("You are hungry.".to_string());
                    //             }
                    //         }
                    //         HungerState::Hungry => {
                    //             clock.state = HungerState::Starving;
                    //             clock.duration = 200;
                    //             if entity == *player_entity {
                    //                 log.entries.push("You are starving!".to_string());
                    //             }
                    //         }
                    //         HungerState::Starving => {
                    //             // Inflict damage from hunger
                    //             if entity == *player_entity {
                    //                 log.entries.push("Your hunger pangs are getting painful! You suffer 1 hp damage.".to_string());
                    //             }
                    //             SufferDamage::new_damage(&mut inflict_damage, entity, 1);
                    //         }
                    //     }
                    // }
            }
        }
    }
}
