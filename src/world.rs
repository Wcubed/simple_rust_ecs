use anymap::AnyMap;
use std::any::Any;
use std::collections::HashMap;

use entity::Entity;
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

    parents: HashMap<Entity, Entity>,

    ent_added: HashMap<usize, usize>,
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

            parents: HashMap::new(),

            ent_added: HashMap::new(),
            ent_remove: HashMap::new(),
            ent_changed: HashMap::new(),
        }
    }

    /// Adds a new entity.
    /// Will reuse an idx if available, if not it will increment the idx counter and allocate
    /// space for the components.
    /// The new entity will only show up in the iterator and list after
    /// `confirm_changes()` is called.
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

        // Register the entity as newly added.
        self.ent_added.insert(idx, uuid);

        Entity { idx: idx, uuid: uuid }
    }

    /// Slates an entity for removal.
    /// The removal won't actually be done until `confirm_changes()` is called.
    pub fn remove_entity(&mut self, entity: Entity) {
        self.ent_remove.insert(entity.idx, entity.uuid);
    }

    /// Removes all entities currently slated for removal.
    /// Also clears the lists of added, removed and changed entities and
    /// checks all entries in the `parents` hashmap for validity.
    pub fn confirm_changes(&mut self) {
        // Remove all entities in the `remove` list.
        for (&idx, &uuid) in self.ent_remove.iter() {
            if self.is_valid_entity(&Entity{ idx: idx, uuid: uuid }) {
                self.active[idx] = 0;
                self.reusable_idxs.push(idx);
                // Clear the components associated with the entity.
                self.components[idx].clear();
            }
        }

        // Check if there are any invalid parent links.
        // This could probably be done quicker (that `clone()` doesn't look efficient).
        for (child, parent) in self.parents.clone().iter() {
            if !(self.is_valid_entity(child) && self.is_valid_entity(parent)) {
                self.parents.remove(child);
            }
        }

        self.ent_added.clear();
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
            self.ent_changed.insert(entity.idx, entity.uuid);
            return self.components[entity.idx].insert(component);
        }
        None
    }

    /// Returns whether an entity has a specific component or not.
    /// Does also search the parents for the component.
    pub fn has_component<T: Any>(&self, entity: &Entity) -> bool {
        if self.is_valid_entity(entity) {
            if self.components[entity.idx].contains::<T>() {
                return true;
            } else {
                // This entity doesn't have the component.
                // See if has inherited it from a parent.
                let mut cur_ent = *entity;
                println!("Start {}, {}", cur_ent.idx, cur_ent.uuid);
                loop  {
                    if self.is_valid_entity(&cur_ent) {
                        if self.components[cur_ent.idx].contains::<T>() {
                            return true;
                        }
                        if let Some(parent) = self.get_parent(&cur_ent) {
                            cur_ent = parent;
                            println!("Parent {}, {}", cur_ent.idx, cur_ent.uuid);
                        } else {
                            // No parents left.
                            break;
                        }
                    } else {
                        // No valid parents left.
                        break;
                    }
                }
            }
        }
        false
    }

    /// Returns a reference to a component.
    /// If a component does not exist, but it does in the parent,
    /// the parent's component will be returned.
    pub fn get_component<T: Any>(&self, entity: &Entity) -> Option<&T> {
        if self.is_valid_entity(entity) {
            // See if the component is there, if so: return it.
            match self.components[entity.idx].get::<T>() {
                Some(comp) => return Some(comp),
                None => {
                    // This entity doesn't have the component.
                    // See if has inherited it from a parent.
                    let mut cur_ent = *entity;
                    loop  {
                        if self.is_valid_entity(&cur_ent) {
                            if let Some(comp) = self.components[cur_ent.idx].get::<T>() {
                                return Some(comp);
                            }
                            if let Some(parent) = self.get_parent(&cur_ent) {
                                cur_ent = parent;
                            } else {
                                // No parents left.
                                break;
                            }
                        } else {
                            // No valid parents left.
                            break;
                        }
                    }
                }
            }
        }
        None
    }

    /// Returns a mutable reference to a component, if it exists.
    /// Mutable references won't be retreived from an entities parent,
    /// as this will easily lead to bugs that are very hard to debug.
    pub fn get_mut_component<T: Any>(&mut self, entity: &Entity) -> Option<&mut T> {
        if self.is_valid_entity(entity) {
            let comp = self.components[entity.idx].get_mut::<T>();

            return match comp {
                Some(val) => {
                    self.ent_changed.insert(entity.idx, entity.uuid);
                    Some(val)
                },
                None => None,
            }
        }
        None
    }

    /// Removes a component from an entity.
    /// Returning the component if it existed, or `None` if it didn't.
    pub fn remove_component<T: Any>(&mut self, entity: &Entity) -> Option<T> {
        if self.is_valid_entity(entity) {
            let res = self.components[entity.idx].remove::<T>();

            return match res {
                Some(res) => {
                    self.ent_changed.insert(entity.idx, entity.uuid);
                    Some(res)
                },
                None => None,
            }
        }
        None
    }

    /// Sets the parent of an entity.
    /// Checks if both the entity and the parent are valid.
    pub fn set_parent(&mut self, entity: &Entity, parent: &Entity) -> bool {
        if self.is_valid_entity(entity) && self.is_valid_entity(parent) {
            self.parents.insert(entity.clone(), parent.clone());
        } else {
            return false
        }
        true
    }

    /// Returns the parent of an entity, if it has one.
    pub fn get_parent(&self, entity: &Entity) -> Option<Entity> {
        if self.is_valid_entity(entity) {
            if let Some(parent) = self.parents.get(entity) {
                return Some(parent.clone());
            }
        }
        None
    }

    /// Temoves the parenting link from an Entity.
    pub fn unlink_parent(&mut self, entity: &Entity) {
        if self.is_valid_entity(entity) {
            self.parents.remove(entity);
        }
    }

    /// Returns a lazy iterator for immutable acces to the entities.
    pub fn iterator(&self) -> EntityIterator {
        EntityIterator {
            active: &self.active,
            added: &self.ent_added,
            curr: 0,
        }
    }

    /// Returns a vector listing all the currently active entities.
    /// Can be used to iterate over all active entities while making changes to the world.
    pub fn list_entities(&self) -> Vec<Entity> {
        self.active.iter().enumerate()
            .filter(|&(_, &uuid)| uuid != 0)
            .filter(|&(idx, &uuid)| {
                if let Some(&other_uuid) = self.ent_added.get(&idx) {
                    if uuid == other_uuid {
                        return false;
                    }
                }
                true
            })
            .map(|(idx, &uuid)| Entity{ idx: idx, uuid: uuid })
            .collect::<Vec<Entity>>()
    }

    /// Returns a vector listing all the recently added entities.
    pub fn list_additions(&self) -> Vec<Entity> {
        self.ent_added.iter()
            .map(|(&idx, &uuid)| Entity{ idx: idx, uuid: uuid })
            .collect::<Vec<Entity>>()
    }
    /// Returns a vector listing all the entities currently slated for removal.
    pub fn list_removals(&self) -> Vec<Entity> {
        self.ent_remove.iter()
            .map(|(&idx, &uuid)| Entity{ idx: idx, uuid: uuid })
            .collect::<Vec<Entity>>()
    }

    /// Returns a vector listing all the entities that have changed since the last call of.
    /// `confirm_changes`.
    pub fn list_changes(&self) -> Vec<Entity> {
        self.ent_changed.iter()
            .map(|(&idx, &uuid)| Entity{ idx: idx, uuid: uuid })
            .collect::<Vec<Entity>>()
    }
}
