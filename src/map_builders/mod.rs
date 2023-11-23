use super::{ Map, Rect, TileType, Position, SHOW_MAPGEN, spawner };
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
    // Note that until we have a second map type, this isn't even slighlty random
    Box::new(SimpleMapBuilder::new(new_depth))
}