use rltk::{ Point, Rltk, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};
use super::{CombatStats, Position, Player, RunState, State, Map, Viewshed, WantsToMelee};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let combat_stats = ecs.read_storage::<CombatStats>();
    let entities = ecs.entities();
    let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();

    let map = ecs.fetch::<Map>();

    for (entity, _player, pos, viewshed) in (&entities, &players, &mut positions, &mut viewsheds).join() {
        if pos.x + delta_x < 1 || pos.x + delta_x > map.width-1 || pos.y + delta_y < 1 || pos.y + delta_y > map.height-1 { return; }
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
        // if a target is in the tile attack them
        for potential_target in map.tile_content[destination_idx].iter() {
            let target = combat_stats.get(*potential_target);
            match target {
                None => {}
                Some(_target) => {
                    // Attack it
                    wants_to_melee.insert(entity, WantsToMelee{ target: *potential_target }).expect("Add target failed");
                    return; // So we don't move after attacking
                }
            }
        }
        // otherwise move if it isn't blocked
        if !map.blocked[destination_idx] {
            pos.x = min(79 , max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
        
            viewshed.dirty = true; // forces game to render new tiles
            let mut ppos = ecs.write_resource::<Point>();
            ppos.x = pos.x;
            ppos.y = pos.y;
        }
    }
}



pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    // Player movement
    match ctx.key {
        None => { return RunState::Paused } // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left |
            VirtualKeyCode::Numpad4 |
            VirtualKeyCode::H => try_move_player(-1, 0, &mut gs.ecs),

            VirtualKeyCode::Right |
            VirtualKeyCode::Numpad6 |
            VirtualKeyCode::L => try_move_player(1, 0, &mut gs.ecs),

            VirtualKeyCode::Up |
            VirtualKeyCode::Numpad8 |
            VirtualKeyCode::K => try_move_player(0, -1, &mut gs.ecs),

            VirtualKeyCode::Down |
            VirtualKeyCode::Numpad2 |
            VirtualKeyCode::J => try_move_player(0, 1, &mut gs.ecs),

            // Diagonals
            VirtualKeyCode::Numpad9 |
            VirtualKeyCode::Y => try_move_player(1, -1, &mut gs.ecs),

            VirtualKeyCode::Numpad7 |
            VirtualKeyCode::U => try_move_player(-1, -1, &mut gs.ecs),

            VirtualKeyCode::Numpad3 |
            VirtualKeyCode::N => try_move_player(1, 1, &mut gs.ecs),

            VirtualKeyCode::Numpad1 |
            VirtualKeyCode::B => try_move_player(-1, 1, &mut gs.ecs),

            _ => { return RunState::Paused }
        },
    }
    RunState::Running
}