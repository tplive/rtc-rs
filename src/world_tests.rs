#[cfg(test)]
use crate::world::World;

#[test]
fn creating_a_world() {
    let w = World::default();

    assert!(w.objects.is_empty());
    assert!(w.light.is_empty());
}
