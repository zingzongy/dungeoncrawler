use crate::prelude::*;
const NUM_ROOMS: usize = 5;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            player_start: Point::zero(),
        };
        mb.fill(TileType::Wall);
        mb.build_random_rooms(rng);
        mb.build_corridors(rng);
        mb.player_start = mb.rooms[0].center();
        mb
    }
    //iterate through the vector of tiles in map, and assigning the new TileType to it
    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }
    //Building random rooms equal to the number of NUM_ROOMS
    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        //while the amount of rooms is less than NUM_ROOMS
        while self.rooms.len() < NUM_ROOMS {
            //declaring a new room that is a Rect with size
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10), //center of room at x position
                rng.range(1, SCREEN_HEIGHT - 10), // center of room at y position
                rng.range(2, 10), //width of the room 
                rng.range(2, 10) //height of the room
            );
            let mut overlap = false; //initialize overlap to false
            for r in self.rooms.iter() { //going through what we have stored in rooms
                if r.intersect(&room) { //if a room in rooms intersects with our new room
                    overlap = true; //then overlap = true, and we skip the next if
                }
            }
            if !overlap { //if they dont overlap, start assigning the new TileType
                room.for_each(|p| { //going through the tiles in the new room
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
                        let idx = map_idx(p.x, p.y); //getting the index for that tile
                        self.map.tiles[idx] = TileType::Floor; //setting the tile to floor
                    }
                });
                self.rooms.push(room); //lastly, we push the new room into our rooms vector
            }

        }
    }
    //carving out the vertical tunnel here
    //y1 is the vertical starting point, y2 is the ending point
    //x is where the tunnel will be at x location
    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{min, max}; //use min and max crate here
        for y in min(y1, y2)..=max(y1, y2) { //looping through the min of y1, y2 whichever one is
                                             //smaller, to the max of y1, y2 inclusively.
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) { //in try_idx if this particular
                                                                    //point is in bound, which
                                                                    //means its Some idx, then it
                                                                    //will return that idx value.
                self.map.tiles[idx] = TileType::Floor; //now we set the map tile at that idx to be
                                                       //floor
            }
        }
    }
    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{min, max};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }
    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));
        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i-1].center();
            let new = rooms[i].center();
            if rng.range(0, 2) == 1 {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, new.x, new.y);
            } else {
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, new.x);
            }
        }
    }
}

    
    

