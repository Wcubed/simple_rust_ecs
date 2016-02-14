//! srecs stands for: "Simple Rust Entity Component System".

extern crate anymap;

pub mod world;
pub mod components;

/// Entity identifier used to acces an Entity in the world.
#[derive(Copy, Clone)]
pub struct Entity {
    pub idx: usize,
    pub uuid: usize,
}

/// Iterates over all valid entities in the world it was generated from.
/// Only allows immutable acces to the world because the world has been borrowed.
pub struct EntityIterator<'a> {
    active: &'a Vec<usize>,
    curr: usize,
}

impl<'a> Iterator for EntityIterator<'a> {
    type Item = Entity;

    fn next(&mut self) -> Option<Entity> {

        for idx in self.curr .. self.active.len() {
            if let Some(uuid) = self.active.get(idx) {
                if *uuid != 0 {
                    self.curr += 1;
                    return Some(Entity{ idx: idx, uuid: *uuid });
                }
            }
        }
        return None;
    }
}
