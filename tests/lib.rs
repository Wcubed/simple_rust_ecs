extern crate anymap_ecs;

use anymap_ecs::{ World, Entity };

/// Struct to be used as a component.
struct Position {
    x: u64,
    y: u64,
}

#[test]
fn adding_and_deleting_entities() {
    let mut world = World::new();

    let e1 = world.add_entity();
    let e2 = world.add_entity();

    println!("E1: idx = {}, uuid = {}", e1.idx, e1.uuid);
    println!("E2: idx = {}, uuid = {}", e2.idx, e2.uuid);

    // The entity idx should start at 0.
    assert!(e1.idx == 0);
    // The entity uuid should start at 1, (0 means invalid entity).
    assert!(e1.uuid == 1);

    // The idxs of two active entities should never be the same.
    assert!(e1.idx != e2.idx);
    // Two uuids may never be the same.
    assert!(e1.uuid != e2.uuid);

    // Deleting an existing entity should succeed and return true.
    assert!(world.remove_entity(e1));

    let e3 = world.add_entity();

    println!("E3: idx = {}, uuid = {}", e3.idx, e3.uuid);

    // The new entity should have a idx of 0, because of entity 1 being deleted.
    assert!(e3.idx == 0);

    // The new entity should have a uuid of 3, because these go up by 1 for every entity created.
    assert!(e3.uuid == 3);

    // Copy the values of entity 2 so that we can try to acces is after deletion.
    let e2_copy = Entity{ idx: e2.idx, uuid: e2.uuid };

    // Add a component to entity 2 and then remove the entity.
    world.add_component(&e2, Position{ x: 3, y: 10 });
    world.remove_entity(e2);

    // Try and acces the component with the invalid entity, this should return `None`.
    if let Some(_) = world.get_mut_component::<Position>(&e2_copy) {
        panic!("The Position component of entity 2 could be accesed
            after the entity was deleted.");
    }
}

#[test]
fn adding_and_getting_components() {
    let mut world = World::new();

    // Add some entities and components to see if the world will return the correct components.
    let e1 = world.add_entity();
    let e2 = world.add_entity();

    world.add_component(&e2, Position{ x: 5, y: 7 });
    world.add_component(&e1, Position{ x: 10, y: 12 });

    let e3 = world.add_entity();

    let e4 = world.add_entity();
    world.add_component(&e4, Position{ x: 3, y: 14 });

    // Get a reference to the Position component of entity 1.
    let e1_pos = world.get_component::<Position>(&e1);

    if let Some(ref position) = e1_pos {
        assert!(position.x == 10);
        assert!(position.y == 12);
    } else {
        panic!("The Position component of entity 1 should exist, but it doesn't.");
    }

    // Get a reference to the Position component of entity 2.
    let e2_pos = world.get_component::<Position>(&e2);

    if let Some(ref position) = e2_pos {
        assert!(position.x == 5);
        assert!(position.y == 7);
    } else {
        panic!("The Position component of entity 2 should exist, but it doesn't.");
    }

    // The Position component of entity 3 shouldn't exist.
    let e3_pos = world.get_component::<Position>(&e3);

    if let Some(ref position) = e3_pos {
        panic!("Entity 3 shouldn't have a position component, but it does: x = {}, y = {}",
            position.x, position.y);
    }
}

#[test]
fn mutating_components() {
    let mut world = World::new();

    let e1 = world.add_entity();
    let e2 = world.add_entity();

    world.add_component(&e1, Position{ x: 4, y: 13 });
    world.add_component(&e2, Position{ x: 8, y: 0 });

    // Change the values of the Position component of entity 1.
    if let Some(ref mut position) = world.get_mut_component::<Position>(&e1) {
        position.x = 10;
        position.y = 14;
    } else {
        panic!("The Position component of entity 1 should exist, but it doesn't.");
    }

    // Check if the values have changed correctly.
    if let Some(ref position) = world.get_mut_component::<Position>(&e1) {
        assert!(position.x == 10);
        assert!(position.y == 14);
    } else {
        panic!("The Position component of entity 1 should exist, but it doesn't.");
    }
}


#[test]
fn iterator_test() {
    let mut world = World::new();

    // Add 10 entities, give half of them Position components.
    for i in 0..10 {
        let ent = world.add_entity();
        if i % 2 == 0 {
            world.add_component(&ent, Position{ x: i, y: i * 2 });
        }
    }

    let mut iter_count = 0;
    let mut pos_count = 0;

    // Test the iterator, we should have 10 entities.
    // 5 of which should have Position components.
    for ent in world.iterator() {
        iter_count += 1;

        if let Some(_) = world.get_component::<Position>(&ent) {
            pos_count += 1;
        }
    }

    assert!(iter_count == 10);
    assert!(pos_count == 5);
}

#[test]
fn entity_list() {
    let mut world = World::new();

    // Create 10 entities.
    for _ in 0..10 {
        world.add_entity();
    }

    // Delete all entities.
    for ent in world.list_entities() {
        world.remove_entity(ent);
    }

    for _ in world.iterator() {
        panic!("There shouldn't be any entities left, but there is at least one.");
    }
}
