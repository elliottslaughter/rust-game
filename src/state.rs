pub type EntityId = slotmap::DefaultKey;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum EntityKind {
    Player,
    Monster,
}

#[derive(Copy, Clone)]
pub struct Entity {
    pub hitbox: sdl2::rect::Rect,
    pub kind: EntityKind,
    pub facing_direction: i8, // 0/1/2/3 for up/left/down/right
}

#[derive(Default)]
pub struct State {
    pub entities: slotmap::SlotMap<EntityId, Entity>,
}
