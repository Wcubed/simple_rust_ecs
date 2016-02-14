//! srecs stands for: "Simple Rust Entity Component System".

extern crate anymap;

use std::collections::HashMap;

pub mod world;
pub mod entity;

use entity::Entity;

/// Iterates over all valid entities in the world it was generated from.
/// Only allows immutable acces to the world because the world has been borrowed.
pub struct EntityIterator<'a> {
    active: &'a Vec<usize>,
    added: &'a HashMap<usize, usize>,
    curr: usize,
}

impl<'a> Iterator for EntityIterator<'a> {
    type Item = Entity;

    fn next(&mut self) -> Option<Entity> {

        for idx in self.curr .. self.active.len() {
            if let Some(uuid) = self.active.get(idx) {
                if *uuid != 0 {
                    // Check if the entity has not recently been added.
                    // If it hasn't return it.
                    match self.added.get(&idx) {
                        Some(_) => continue,
                        None => {
                            self.curr = idx + 1;
                            return Some(Entity{ idx: idx, uuid: *uuid })
                        },
                    }
                }
            }
        }
        return None;
    }
}
