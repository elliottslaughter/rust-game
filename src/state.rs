pub type EntityId = slotmap::DefaultKey;

#[derive(Copy, Clone)]
pub struct Entity {
    pub pos: [i32; 2],
    pub size: [u32; 2],
}

#[derive(Default)]
pub struct State {
    pub entities: slotmap::SlotMap<EntityId, Entity>,
}
