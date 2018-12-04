use crate::rect::Rect;

pub type EntityId = slotmap::DefaultKey;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum EntityKind {
    Player,
    Monster,
    Power,
}

#[derive(Copy, Clone)]
pub struct Entity {
    pub hitbox: Rect,
    pub kind: EntityKind,
    pub facing_direction: i32, // 0/1/2/3 for up/left/down/right
    pub attack_frame: Option<usize>,
    pub attack_box: Rect,
    pub agro: i32,
    pub score: i32,
    pub power: i32,
}

#[derive(Default)]
pub struct State {
    pub entities: slotmap::SlotMap<EntityId, Entity>,
}
