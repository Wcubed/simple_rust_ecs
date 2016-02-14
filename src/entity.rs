/// Entity identifier used to acces an Entity in the world.
#[derive(Copy, Clone, Eq, Hash)]
pub struct Entity {
    pub idx: usize,
    pub uuid: usize,
}

impl PartialEq for Entity {
    fn eq(&self, other: &Entity) -> bool {
        return self.idx == other.idx && self.uuid == other.uuid
    }
}
