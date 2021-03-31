use std::cmp::min;

use super::{PhotoPhobia, Position, Map, RunState};
use specs::prelude::*;
use ordered_float::*;

pub struct PhotoPhobiaSystem {}

impl<'a> System<'a> for PhotoPhobiaSystem {
#[allow(clippy::type_complexity)]
  type SystemData = (
        WriteStorage<'a, PhotoPhobia>,
        ReadStorage<'a, Position>,
        ReadExpect<'a, RunState>,
        ReadExpect<'a, Map>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut photophobia, positions,  runstate , map) =
            data;

        for (position, mut state) in (&positions, &mut photophobia).join() {

          if *runstate == RunState::PlayerTurn {
            let idx = map.xy_idx(position.x, position.y);
            let light_intensity = map.light_intensity[idx];
            state.current = min(OrderedFloat::from(0.0), OrderedFloat::from(state.current - state.recovery_rate)).into_inner();
            state.current = state.current + state.sensitivity * light_intensity;
          }
        }
    }
}