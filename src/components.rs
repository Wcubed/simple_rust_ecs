
use Entity;

/// Reference to the parent of an entity.
pub struct Parent (pub Entity);

/// Lists the children of a parental entity.
pub struct Children (pub Vec<Entity>);
