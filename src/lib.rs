extern crate anymap;

use anymap::AnyMap;

/// Entity identifier used to acces an Entity in the world.
pub struct Entity {
    idx: usize,
    uuid: usize,
}


pub struct World {
    next_idx: usize,
    next_uuid: usize,
    /// Lists every entity with it's uuid.
    /// A uuid of `0` means an inactive entity.
    active: Vec<usize>,
    reusable_idxs: Vec<usize>,
    components: Vec<AnyMap>,
}

impl World {
    pub fn new() -> World {
        World {
            next_idx: 0,
            next_uuid: 1,
            active: Vec::new(),
            reusable_idxs: Vec::new(),
            components: Vec::new(),
        }
    }

    /// Adds a new entity.
    /// Will reuse an idx if available, if not it will increment the idx counter and allocate
    /// space for the components.
    pub fn add_entity(&mut self) -> Entity {
        // Get the idx.
        let idx = match self.reusable_idxs.pop() {
            None => {
                // No reusable idxs, so make a new one.
                let idx = self.next_idx;
                self.next_idx += 1;
                // And add a new entry to the components vector.
                self.components.push(AnyMap::new());
                idx
            },
            Some(idx) => idx,
        };

        // Get the uuid.
        let uuid = self.next_uuid;
        self.next_uuid += 1;

        Entity { idx: idx, uuid: uuid }
    }

    /// Removes an entity, but only if the uuid matches.
    pub fn remove_entity(&mut self, entity: &Entity) {
        if self.is_valid_entity(entity) {
            self.active[entity.idx] = 0;
            self.reusable_idxs.push(entity.idx);
        }
    }

    /// Checks if an `Entity` reference is valid.
    pub fn is_valid_entity(&self, entity: &Entity) -> bool {
        match self.active.get(entity.idx) {
            Some(uuid) => {
                *uuid != 0 && entity.uuid == *uuid
            },
            None => false,
        }
    }
}
