use anymap::AnyMap;
use std::any::Any;
use std::collections::HashMap;

use Entity;
use EntityIterator;

/// Keeps track of entities and their components.
pub struct World {
    next_idx: usize,
    next_uuid: usize,
    /// Lists every entity with it's uuid.
    /// A uuid of `0` means an inactive entity.
    active: Vec<usize>,
    reusable_idxs: Vec<usize>,
    /// List of all the components.
    components: Vec<AnyMap>,

    ent_remove: HashMap<usize, usize>,
    ent_changed: HashMap<usize, usize>,
}

impl World {
    pub fn new() -> World {
        World {
            next_idx: 0,
            next_uuid: 1,
            active: Vec::new(),
            reusable_idxs: Vec::new(),
            components: Vec::new(),

            ent_remove: HashMap::new(),
            ent_changed: HashMap::new(),
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

        // Register the entity as recently changed.
        self.ent_changed.insert(idx, uuid);

        Entity { idx: idx, uuid: uuid }
    }

    /// Slates an entity for removal.
    /// The removal won't actually be done until `propagate_changes()` is called.
    pub fn remove_entity(&mut self, entity: Entity) {
        self.ent_remove.insert(entity.idx, entity.uuid);
    }

    /// Removes all entities currently slated for removal.
    pub fn propagate_changes(&mut self) {
        // Remove all entities in the `remove` list.
        for (&idx, &uuid) in self.ent_remove.iter() {
            if self.is_valid_entity(&Entity{ idx: idx, uuid: uuid }) {
                self.active[idx] = 0;
                self.reusable_idxs.push(idx);
                // Clear the components associated with the entity.
                self.components[idx].clear();
            }
        }

        self.ent_remove.clear();

        self.ent_changed.clear();
    }

    /// Checks if an `Entity` reference is valid.
    pub fn is_valid_entity(&self, entity: &Entity) -> bool {
        match self.active.get(entity.idx) {
            Some(&uuid) => {
                uuid != 0 && entity.uuid == uuid
            },
            None => false,
        }
    }

    /// Returns the uuid currently associated with an entity index.
    /// Returns 0 for nonexistent entities.
    pub fn get_uuid(&self, idx: usize) -> usize {
        match self.active.get(idx) {
            Some(&uuid) => uuid,
            None => 0,
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

    /// Returns whether an entity has a specific component or not.
    pub fn has_component<T: Any>(&self, entity: &Entity) -> bool {
        if self.is_valid_entity(entity) {
            return self.components[entity.idx].contains::<T>();
        }
        false
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

    /// Returns a lazy iterator for immutable acces to the entities.
    pub fn iterator(&self) -> EntityIterator {
        EntityIterator {
            active: &self.active,
            curr: 0,
        }
    }

    /// Returns a vector listing all the currently active entities.
    /// Can be used to iterate over all active entities while making changes to the world.
    pub fn list_entities(&self) -> Vec<Entity> {
        self.active.iter().enumerate()
            .filter(|&(_, &uuid)| uuid != 0)
            .map(|(idx, &uuid)| Entity{ idx: idx, uuid: uuid })
            .collect::<Vec<Entity>>()
    }
}
