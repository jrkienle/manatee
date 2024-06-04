use super::Entity;
use hashbrown::HashMap;

pub struct EntityManager {
    entities: HashMap<u32, Entity>,
    // The current number of entities in a given Scene. An entity's unique ID is derived from this
    // number. For instance, if len is currently 2 and a new entity is spawned, that entity's ID
    // would be set to 2 and len would be incremented by 1, making len == 3
    len: u32,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            len: 0,
        }
    }

    pub fn add(&mut self) -> &Entity {
        let entity_id = self.len;
        let entity = Entity::new(entity_id);
        self.entities.insert(entity_id, entity);
        self.len += 1;
        println!("Added entity with ID {entity_id}");
        self.entities.get(&entity_id).unwrap()
    }
}
