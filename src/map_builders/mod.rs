use super::{ Map, Rect, TileType, Position, SHOW_MAPGEN, spawner };
mod bsp_dungeon;
use bsp_dungeon::BspDungeonBuilder;
mod simple_map;
use simple_map::SimpleMapBuilder;
mod common;
use common::*;
use specs::prelude::*;

// Traits are like interfaces!
pub trait MapBuilder {
    fn build_map(&mut self);
    fn spawn_entities(&mut self, ecs : &mut World);
    fn get_map(&self) -> Map;
    fn get_starting_position(&self) -> Position;
    fn get_snapshot_history(&self) -> Vec<Map>;
    fn take_snapshot(&mut self);
}

pub fn random_builder(new_depth : i32) -> Box<dyn MapBuilder> {
    // Debug Generators
    // Basic
    //Box::new(SimpleMapBuilder::new(new_depth))
    // BSP
    Box::new(BspDungeonBuilder::new(new_depth))

    // Fully random generator
    // let mut rng = rltk::RandomNumberGenerator::new();
    // let builder = rng.roll_dice(1, 2);
    // match builder {
    //     1 => Box::new(BspDungeonBuilder::new(new_depth)),
    //     _ => Box::new(SimpleMapBuilder::new(new_depth))
    // }
}