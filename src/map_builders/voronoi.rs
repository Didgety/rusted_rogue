use super::{MapBuilder, Map, TileType, Position, paint, spawner, SHOW_MAPGEN,
    remove_unreachable_areas_returning_most_distant, generate_voronoi_spawn_regions};
use rltk::RandomNumberGenerator;
use specs::prelude::*;
use std::collections::HashMap;

/// Pythagoras - Direct line, more natural.
/// Manhattan - Sum of all distances between source and dest. Like a taxi driver, straighter walls.
/// Chebyshev - Similar to Manhattan but accounts for diagonals. Tends to have thicker walls.
#[derive(PartialEq, Copy, Clone)]
pub enum DistanceAlgorithm { Pythagoras, Manhattan, Chebyshev }

pub struct VoronoiCellBuilder {
    map : Map,
    starting_position : Position,
    depth: i32,
    history: Vec<Map>,
    noise_areas : HashMap<i32, Vec<usize>>,
    n_seeds: usize,
    distance_algorithm: DistanceAlgorithm,
    spawn_list: Vec<(usize, String)>
}

impl MapBuilder for VoronoiCellBuilder {
    fn get_map(&self) -> Map {
        self.map.clone()
    }

    fn get_starting_position(&self) -> Position {
        self.starting_position.clone()
    }

    fn get_snapshot_history(&self) -> Vec<Map> {
        self.history.clone()
    }

    fn build_map(&mut self)  {
        self.build();
    }

    // fn spawn_entities(&mut self, ecs : &mut World) {
    //     for area in self.noise_areas.iter() {
    //         spawner::spawn_region(ecs, area.1, self.depth);
    //     }
    // }

    fn get_spawn_list(&self) -> &Vec<(usize, String)> {
        &self.spawn_list
    }

    fn take_snapshot(&mut self) {
        if SHOW_MAPGEN {
            let mut snapshot = self.map.clone();
            for v in snapshot.revealed_tiles.iter_mut() {
                *v = true;
            }
            self.history.push(snapshot);
        }
    }
}

impl VoronoiCellBuilder {
    #[allow(dead_code)]
    pub fn new(new_depth : i32) -> VoronoiCellBuilder {
        VoronoiCellBuilder{
            map : Map::new(new_depth),
            starting_position : Position{ x: 0, y : 0 },
            depth : new_depth,
            history: Vec::new(),
            noise_areas : HashMap::new(),
            n_seeds: 64,
            distance_algorithm: DistanceAlgorithm::Pythagoras,
            spawn_list : Vec::new()
        }
    }
    
    pub fn pythagoras(new_depth : i32) -> VoronoiCellBuilder {
        VoronoiCellBuilder{
            map : Map::new(new_depth),
            starting_position : Position{ x: 0, y : 0 },
            depth : new_depth,
            history: Vec::new(),
            noise_areas : HashMap::new(),
            n_seeds: 64,
            distance_algorithm: DistanceAlgorithm::Pythagoras,
            spawn_list : Vec::new()
        }
    }

    pub fn manhattan(new_depth : i32) -> VoronoiCellBuilder {
        VoronoiCellBuilder{
            map : Map::new(new_depth),
            starting_position : Position{ x: 0, y : 0 },
            depth : new_depth,
            history: Vec::new(),
            noise_areas : HashMap::new(),
            n_seeds: 64,
            distance_algorithm: DistanceAlgorithm::Manhattan,
            spawn_list : Vec::new()
        }
    }

    pub fn chebyshev(new_depth : i32) -> VoronoiCellBuilder {
        VoronoiCellBuilder{
            map : Map::new(new_depth),
            starting_position : Position{ x: 0, y : 0 },
            depth : new_depth,
            history: Vec::new(),
            noise_areas : HashMap::new(),
            n_seeds: 64,
            distance_algorithm: DistanceAlgorithm::Chebyshev,
            spawn_list : Vec::new()
        }
    }

    #[allow(clippy::map_entry)]
    fn build(&mut self) {
        let mut rng = RandomNumberGenerator::new();

        let n_seeds = 64;
        let mut voronoi_seeds : Vec<(usize, rltk::Point)> = Vec::new();

        // generate seeds for voronoi cells
        while voronoi_seeds.len() < n_seeds {
            let vx = rng.roll_dice(1, self.map.width-1);
            let vy = rng.roll_dice(1, self.map.height-1);
            let vidx = self.map.xy_idx(vx, vy);
            // saving this point could be skipped and the calculation done directly but this is clearer
            let candidate = (vidx, rltk::Point::new(vx, vy));
            if !voronoi_seeds.contains(&candidate) {
                voronoi_seeds.push(candidate);
            }
        }

        let mut voronoi_distance = vec![(0, 0.0f32) ; n_seeds];
        let mut voronoi_membership : Vec<i32> = vec![0 ; self.map.width as usize * self.map.height as usize];
        // each tile is assigned to the voronoi cell of the seed it is closest to
        for (i, vid) in voronoi_membership.iter_mut().enumerate() {
            let x = i as i32 % self.map.width;
            let y = i as i32 / self.map.width;

            for (seed, pos) in voronoi_seeds.iter().enumerate() {
                let distance;
                match self.distance_algorithm {           
                    DistanceAlgorithm::Pythagoras => {
                        distance = rltk::DistanceAlg::PythagorasSquared.distance2d(
                            rltk::Point::new(x, y), 
                            pos.1
                        );
                    }
                    DistanceAlgorithm::Manhattan => {
                        distance = rltk::DistanceAlg::Manhattan.distance2d(
                            rltk::Point::new(x, y), 
                            pos.1
                        );
                    }
                    DistanceAlgorithm::Chebyshev => {
                        distance = rltk::DistanceAlg::Chebyshev.distance2d(
                            rltk::Point::new(x, y), 
                            pos.1
                        );
                    }
                }
                voronoi_distance[seed] = (seed, distance);
            }

            voronoi_distance.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());

            *vid = voronoi_distance[0].0 as i32;
        }

        // build the map
        for y in 1..self.map.height-1 {
            for x in 1..self.map.width-1 {
                let mut neighbors = 0;
                let my_idx = self.map.xy_idx(x, y);
                let my_seed = voronoi_membership[my_idx];
                if voronoi_membership[self.map.xy_idx(x-1, y)] != my_seed { neighbors += 1; }
                if voronoi_membership[self.map.xy_idx(x+1, y)] != my_seed { neighbors += 1; }
                if voronoi_membership[self.map.xy_idx(x, y-1)] != my_seed { neighbors += 1; }
                if voronoi_membership[self.map.xy_idx(x, y+1)] != my_seed { neighbors += 1; }
        
                if neighbors < 2 {
                    self.map.tiles[my_idx] = TileType::Floor;
                }
            }
            self.take_snapshot();
        }

        // Find a starting point; start at the middle and walk left until we find an open tile
        self.starting_position = Position{ x: self.map.width / 2, y : self.map.height / 2 };
        let mut start_idx = self.map.xy_idx(self.starting_position.x, self.starting_position.y);
        while self.map.tiles[start_idx] != TileType::Floor {
            self.starting_position.x -= 1;
            start_idx = self.map.xy_idx(self.starting_position.x, self.starting_position.y);
        }
        self.take_snapshot();

        // Find all tiles we can reach from the starting point
        let exit_tile = remove_unreachable_areas_returning_most_distant(&mut self.map, start_idx);
        self.take_snapshot();

        // Place the stairs
        self.map.tiles[exit_tile] = TileType::DownStairs;
        self.take_snapshot();

        // Now we build a noise map for use in spawning entities later
        self.noise_areas = generate_voronoi_spawn_regions(&self.map, &mut rng);

        // Spawn the entities
        for area in self.noise_areas.iter() {
            spawner::spawn_region(&self.map, &mut rng, area.1, self.depth, &mut self.spawn_list);
        }
    }
}