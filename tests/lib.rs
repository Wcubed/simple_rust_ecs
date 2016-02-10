extern crate property_rs;

use property_rs::World;

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
}
