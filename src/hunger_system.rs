use specs::prelude::*;
use super::{ gamelog::GameLog, HungerClock, HungerState, RunState, SufferDamage};

pub struct HungerSystem {}

impl<'a> System<'a> for HungerSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = ( 
        Entities<'a>,
        ReadExpect<'a, Entity>, // The player
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, HungerClock>,
        ReadExpect<'a, RunState>,
        WriteStorage<'a, SufferDamage>
      );

      fn run(&mut self, data : Self::SystemData) {
        let (entities, player_entity, mut log, mut hunger_clock, runstate, mut inflict_damage) = data;

            for(entity, mut clock) in (&entities, &mut hunger_clock).join() {
                let mut proceed = false;

                match *runstate {
                    RunState::PlayerTurn => {
                        if entity == *player_entity {
                            proceed = true;
                        }
                    }
                    // Can add hunger to monsters if desired
                    RunState::MonsterTurn => {
                        if entity != *player_entity {
                            proceed = true;
                        }
                    }
                    _ => {
                        proceed = false;
                    }
                }

                if proceed {
                    clock.duration -= 1;
                    if clock.duration < 1 {
                        match clock.state {
                            HungerState::WellFed => {
                                clock.state = HungerState::Normal;
                                clock.duration = 200;
                                if entity == *player_entity {
                                    log.entries.push("You are no longer well fed.".to_string());
                                }
                            }
                            HungerState::Normal => {
                                clock.state = HungerState::Hungry;
                                clock.duration = 200;
                                if entity == *player_entity {
                                    log.entries.push("You are hungry.".to_string());
                                }
                            }
                            HungerState::Hungry => {
                                clock.state = HungerState::Starving;
                                clock.duration = 200;
                                if entity == *player_entity {
                                    log.entries.push("You are starving! Eat an apple or something!".to_string());
                                }
                            }
                            HungerState::Starving => {
                                // Hunger damage
                                if entity == *player_entity {
                                    log.entries.push("Your stomach begins to digest itself and you take 1 damage.".to_string());
                                }
                                SufferDamage::new_damage(&mut inflict_damage, entity, 1);
                            }
                        }
                    }
                }
            } 
      }



}