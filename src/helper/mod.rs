use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type MapId = i32;

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct AutoIncrementalMap<T> {
    hashmap: HashMap<MapId, T>,
}

impl<T> Default for AutoIncrementalMap<T> {
    fn default() -> Self {
        Self {
            hashmap: HashMap::default(),
        }
    }
}

impl<T> AutoIncrementalMap<T> {
    pub fn get_values(&self) -> Vec<&T> {
        self.hashmap.values().collect()
    }

    pub fn is_empty(&self) -> bool {
        self.hashmap.is_empty()
    }

    pub fn get(&self, id: MapId) -> Option<&T> {
        self.hashmap.get(&id)
    }

    pub fn get_mut(&mut self, id: MapId) -> Option<&mut T> {
        self.hashmap.get_mut(&id)
    }

    pub fn add(&mut self, value: T) -> MapId {
        let id = self.get_last_id() + 1;
        self.hashmap.insert(id, value);
        id
    }

    pub fn remove(&mut self, id: MapId) -> Option<T> {
        self.hashmap.remove(&id)
    }

    fn get_last_id(&self) -> MapId {
        *self.hashmap.keys().max().unwrap_or(&0)
    }
}
