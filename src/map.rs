use util::{Bound, Point};
use actor::{Actors, SafeActor};

#[derive(Clone)]
pub enum MapType {
    Terrain,
    Enemies,
    Friends,
    Pcs
}

pub struct Maps {
    terrain: Map,
    enemies: Map,
    friends: Map,
    pcs:     Map
}

impl Maps {
    pub fn new(size: &Bound) -> Maps {
        Maps {
            terrain: Map::new(size.clone()),
            enemies: Map::new(size.clone()),
            friends: Map::new(size.clone()),
            pcs:     Map::new(size.clone())
        }
    }

    pub fn push_actors(&mut self, actors: Actors, map_type: MapType) {
        let mut map = self.get_map_for_type_mut(map_type);

        for actor in actors {
            map.push_actor(actor.clone());
        }
    }

    pub fn push_actor(&mut self, actor: SafeActor, map_type: MapType) {
        let mut map = self.get_map_for_type_mut(map_type);

        map.push_actor(actor.clone());
    }

    pub fn pull_actors(&self, map_type: MapType) -> Actors {
        let map = self.get_map_for_type(map_type);
        map.pull_actors()
    }

    pub fn pop_actors(&mut self, map_type: MapType) -> Actors {
        let mut map = self.get_map_for_type_mut(map_type);
        map.pop_actors()
    }

    fn get_map_for_type(&self, map_type: MapType) -> &Map {
        match map_type {
            MapType::Terrain => &self.terrain,
            MapType::Enemies => &self.enemies,
            MapType::Friends => &self.friends,
            MapType::Pcs     => &self.pcs
        }
    }

    fn get_map_for_type_mut(&mut self, map_type: MapType) -> &mut Map {
        match map_type {
            MapType::Terrain => &mut self.terrain,
            MapType::Enemies => &mut self.enemies,
            MapType::Friends => &mut self.friends,
            MapType::Pcs     => &mut self.pcs
        }
    }
}

pub struct Map {
    contents: Vec<Actors>,
    size:     Bound
}

impl Map {
    fn new(size: Bound) -> Map {
        let contents = Map::init_contents(&size);
        Map {
            contents: contents,
            size:     size
        }
    }

    fn init_contents(size: &Bound) -> Vec<Actors> {
        let mut contents = vec![];

        for _ in 0 .. (size.max.x * size.max.y) {
            let x = vec![];
            contents.push(x);
        }

        return contents;
    }

    fn push_actor(&mut self, actor: SafeActor) {
        let pushed_actor = actor.clone();
        let actor        = actor.lock().unwrap();
        let position     = self.index_for_point(&actor.position);

        self.contents.get_mut(position).unwrap().push(pushed_actor);
    }

    fn index_for_point(&self, point: &Point) -> usize {
        ((point.y * self.size.max.y) + point.x) as usize
    }

    fn pull_actors(&self) -> Actors {
        let mut ret_val = vec![];

        for vec in &self.contents {
            for actor in vec {
                ret_val.push(actor.clone());
            }
        }
        
        ret_val
    }

    fn pop_actors(&mut self) -> Actors {
        let mut ret_val = vec![];

        for vec in &mut self.contents {
            loop {
                match vec.pop() {
                    Some(actor) => ret_val.push(actor.clone()),
                    _           => break
                }
            }
        }
        
        ret_val
    }
}
