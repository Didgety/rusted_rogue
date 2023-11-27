use super::{ Map, Rect, TileType, Position, SHOW_MAPGEN, spawner };
mod bsp_interior;
use bsp_interior::BspInteriorBuilder;
mod bsp_dungeon;
use bsp_dungeon::BspDungeonBuilder;
mod cellular_automata;
use cellular_automata::CellularAutomataBuilder;
mod common;
use common::*;
mod dla;
use dla::*;
mod drunkard;
use drunkard::*;
mod maze;
use maze::MazeBuilder;
mod simple_map;
use simple_map::SimpleMapBuilder;
mod voronoi;
use voronoi::*;
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
    //Box::new(BspDungeonBuilder::new(new_depth))
    // BSP Interior
    //Box::new(BspInteriorBuilder::new(new_depth))
    // Cellular Automata
    //Box::new(CellularAutomataBuilder::new(new_depth))
    // Drunkards Walk
    // Box::new(DrunkardsWalkBuilder::new(new_depth, DrunkardSettings{ 
    //     spawn_mode: DrunkSpawnMode::Random, 
    //     drunken_lifetime: 200,
    //     floor_percent: 0.4
    // }))
    // - open area
    // Box::new(DrunkardsWalkBuilder::open_area(new_depth))
    // - open halls
    // Box::new(DrunkardsWalkBuilder::open_halls(new_depth))
    // - passages
    // Box::new(DrunkardsWalkBuilder::winding_passages(new_depth))
    // - fat passages
    // Box::new(DrunkardsWalkBuilder::fat_passages(new_depth))
    // - fearful symmetry
    // Box::new(DrunkardsWalkBuilder::fearful_symmetry(new_depth))
    // Maze
    // Box::new(MazeBuilder::new(new_depth))
    // DLA
    // - walk in
    // Box::new(DLABuilder::walk_inwards(new_depth))
    // - walk out
    // Box::new(DLABuilder::walk_outwards(new_depth))
    // - central attractor
    // Box::new(DLABuilder::central_attractor(new_depth))
    // - insectoid
    // Box::new(DLABuilder::insectoid(new_depth))
    // Voronoi Cells
    Box::new(VoronoiCellBuilder::new(new_depth))

    // Fully random generator
    // let mut rng = rltk::RandomNumberGenerator::new();
    // let builder = rng.roll_dice(1, 8);
    // match builder {
    //     1 => Box::new(BspDungeonBuilder::new(new_depth)),
    //     2 => Box::new(BspInteriorBuilder::new(new_depth)),
    //     3 => Box::new(CellularAutomataBuilder::new(new_depth)),
    //     4 => Box::new(DrunkardsWalkBuilder::open_area(new_depth)),
    //     5 => Box::new(DrunkardsWalkBuilder::open_halls(new_depth)),
    //     6 => Box::new(DrunkardsWalkBuilder::winding_passages(new_depth)),
    //     7 => Box::new(DrunkardsWalkBuilder::fat_passages(new_depth)),
    //     8 => Box::new(DrunkardsWalkBuilder::fearful_symmetry(new_depth)),
    //     9 => Box::new(MazeBuilder::new(new_depth)),
    //     10 => Box::new(DLABuilder::walk_inwards(new_depth)),
    //     11 => Box::new(DLABuilder::walk_outwards(new_depth)),
    //     12 => Box::new(DLABuilder::central_attractor(new_depth)),
    //     13 => Box::new(DLABuilder::insectoid(new_depth)),
    //     _ => Box::new(SimpleMapBuilder::new(new_depth))
    // }
}