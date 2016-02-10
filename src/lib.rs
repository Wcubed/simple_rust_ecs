extern crate anymap;

use anymap::AnyMap;
use std::any::Any;

/// Entity identifier used to acces an Entity in the world.
pub struct Entity {
    pub idx: usize,
    pub uuid: usize,
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

        // Register the entity as active.
        if self.active.len() <= uuid {
            self.active.resize(uuid + 1, 0);
        }
        self.active[idx] = uuid;

        Entity { idx: idx, uuid: uuid }
    }

    /// Removes an entity, but only if the uuid matches.
    /// Returns true if the entity existed and could be deleted, otherwise false.
    pub fn remove_entity(&mut self, entity: Entity) -> bool {
        if self.is_valid_entity(&entity) {
            self.active[entity.idx] = 0;
            self.reusable_idxs.push(entity.idx);
            // Clear the components associated with the entity.
            self.components[entity.idx].clear();
            return true;
        }
        false
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

    /// Adds a new component to an entity.
    /// If the entity already had that component, that component is returned.
    /// Otherwise, `None` is returned.
    pub fn add_component<T: Any>(&mut self, entity: &Entity, component: T) -> Option<T> {
        if self.is_valid_entity(entity) {
            return self.components[entity.idx].insert(component);
        }
        None
    }

    /// Returns a reference to a component, if it exists.
    pub fn get_component<T: Any>(&self, entity: &Entity) -> Option<&T> {
        if self.is_valid_entity(entity) {
            return self.components[entity.idx].get::<T>()
        }
        None
    }

    /// Returns a mutable reference to a component, if it exists.
    pub fn get_mut_component<T: Any>(&mut self, entity: &Entity) -> Option<&mut T> {
        if self.is_valid_entity(entity) {
            return self.components[entity.idx].get_mut::<T>();
        }
        None
    }
}
