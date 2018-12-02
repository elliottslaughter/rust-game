pub type EntityId = slotmap::DefaultKey;

#[derive(Copy, Clone)]
pub enum EntityKind {
    Player,
    Monster,
}

#[derive(Copy, Clone)]
pub struct Entity {
    pub hitbox: sdl2::rect::Rect,
    pub kind: EntityKind,
}

#[derive(Default)]
pub struct State {
    pub entities: slotmap::SlotMap<EntityId, Entity>,
}
