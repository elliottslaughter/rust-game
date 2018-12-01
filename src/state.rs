pub type EntityId = slotmap::DefaultKey;

#[derive(Copy, Clone)]
pub struct Entity {
    pub hitbox: sdl2::rect::Rect,
}

#[derive(Default)]
pub struct State {
    pub entities: slotmap::SlotMap<EntityId, Entity>,
}
