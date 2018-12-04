use sdl2::pixels::Color;
use sdl2::render::{Canvas, RenderTarget};
use std::cmp::{max, min};
use std::time::Duration;

use game::control::{process_input, Control};
use game::error::Error;
use game::point::Point;
use game::rect::Rect;
use game::state::{Entity, EntityId, EntityKind, State};

static ATTACK_FRAMES: &'static [(i32, i32, i32, i32)] = &[
    (12, -5, 4, 4),    // gap 0
    (10, -9, 5, 8),    // gap 0
    (8, -17, 6, 16),   // gap 0
    (4, -21, 8, 18),   // gap 2
    (0, -23, 8, 18),   // gap 4
    (-4, -25, 8, 20),  // gap 4
    (-8, -23, 8, 18),  // gap 4
    (-12, -21, 8, 18), // gap 2
    (-14, -17, 6, 16), // gap 0
    (-16, -9, 5, 8),   // gap 0
    (-18, -5, 4, 4),   // gap 0
];

fn process_scripts(state: &mut State, player_id: EntityId, window: Rect, frame_number: u64) {
    let player_hitbox = state
        .entities
        .get_mut(player_id)
        .map(|player| player.hitbox);
    for entity in state.entities.values_mut() {
        if entity.kind == EntityKind::Monster {
            let dir = entity.facing_direction;
            let delta = if entity.agro < 240 {
                Point::new(
                    if dir % 2 == 1 { dir - 2 } else { 0 },
                    if dir % 2 == 0 { dir - 1 } else { 0 },
                )
            } else if let Some(hitbox) = player_hitbox {
                let factor = entity.agro / 320;
                let target = hitbox.center() - entity.hitbox.center();
                if target.x.abs() > target.y.abs() {
                    Point::new(target.x.signum(), 0) * factor
                } else {
                    Point::new(0, target.y.signum()) * factor
                }
            } else {
                Point::new(0, 0)
            };
            let lo = window
                .grow(-entity.hitbox.size().x)
                .clamp(entity.hitbox.lo + delta);
            let hi = lo + entity.hitbox.size();
            let hitbox = Rect::new(lo, hi);

            // Turn when we hit an object.
            if hitbox == entity.hitbox {
                entity.facing_direction = (dir + 1) % 4;
            }

            entity.hitbox = hitbox;

            entity.agro += 1;
        }
    }

    if frame_number % 100 == 0 {
        let dir = ((frame_number / 100) % 4) as i32;
        state.entities.insert(Entity {
            hitbox: Rect::new_with_size(
                window.width() as i32 * ((dir + 1) % 2) + 400 * (dir - 2),
                window.height() as i32 * (dir % 2) + 300 * (dir - 1),
                32,
                32,
            ),
            kind: EntityKind::Monster,
            facing_direction: ((frame_number % 17) % 4) as i32,
            attack_frame: None,
            attack_box: Rect::default(),
            agro: 0,
            score: 0,
            power: 0,
        });
    }

    if frame_number % 175 == 150 {
        state.entities.insert(Entity {
            hitbox: Rect::new_with_size(
                window.width() as i32 * (frame_number as i32 % 61) / 61,
                window.height() as i32 * (frame_number as i32 % 67) / 67,
                32,
                32,
            ),
            kind: EntityKind::Power,
            facing_direction: 0,
            attack_frame: None,
            attack_box: Rect::default(),
            agro: 0,
            score: 0,
            power: 0,
        });
    }
}

fn process_action(state: &mut State, player_id: EntityId, control: &Control, window: Rect) {
    if let Some(player) = state.entities.get_mut(player_id) {
        let delta = Point::new(control.left_right_input, control.up_down_input) * 2;
        let lo = window
            .grow(-player.hitbox.size().x)
            .clamp(player.hitbox.lo + delta);
        let hi = lo + player.hitbox.size();
        player.hitbox = Rect::new(lo, hi);

        player.facing_direction = control.facing_input;
        player.attack_frame = match player.attack_frame {
            Some(frame) => {
                if frame < ATTACK_FRAMES.len() - 1 {
                    Some(frame + 1)
                } else {
                    None
                }
            }
            None => {
                if control.attack_input {
                    Some(0)
                } else {
                    None
                }
            }
        };

        player.attack_box = match player.attack_frame {
            Some(frame) => {
                let b = player.hitbox;
                let top_center = b.index(0, -1);
                let attack: Rect = ATTACK_FRAMES[frame].into();
                let attack = Rect::new(
                    Point::new(
                        attack.lo.x,
                        attack.hi.y - min(attack.height() as i32, 5 * (player.power + 1)),
                    ),
                    attack.hi,
                );
                (attack + top_center).rotate(b.center(), player.facing_direction * 90)
            }
            None => Rect::default(),
        };
    }
}

fn process_collisions(state: &mut State, player_id: EntityId) {
    let mut score = 0;
    let mut power = 0;
    if let Some(player) = state.entities.get(player_id) {
        let mut dead = Vec::new();

        for (id, entity) in state.entities.iter() {
            if entity.kind == EntityKind::Monster {
                // Monster hit player.
                if entity.hitbox.has_intersection(player.hitbox)
                    || entity.attack_box.has_intersection(player.hitbox)
                {
                    dead.push(player_id);
                    break;
                }

                // Player hit moster.
                if player.attack_box.has_intersection(entity.hitbox) {
                    dead.push(id);
                    score += 1;
                    break;
                }
            } else if entity.kind == EntityKind::Power {
                if player.hitbox.has_intersection(entity.hitbox) {
                    dead.push(id);
                    power += 4;
                    break;
                }
            }
        }

        for id in dead {
            state.entities.remove(id);
        }
    }

    if let Some(player) = state.entities.get_mut(player_id) {
        player.score += score;
        player.power = max(player.power - score + power, 0);
        if score > 0 || power > 0 {
            println!("score: {} power: {}", player.score, player.power);
        }
    }
}

// Wrapper for fill_rect since I can't get the type adapters to work properly.
trait Fill {
    fn fill(&mut self, r: Rect) -> Result<(), String>;
}

impl<T: RenderTarget> Fill for Canvas<T> {
    fn fill(&mut self, r: Rect) -> Result<(), String> {
        let r: sdl2::rect::Rect = r.into();
        self.fill_rect(r)
    }
}

fn render<T: RenderTarget>(canvas: &mut Canvas<T>, state: &State, player_id: EntityId) -> Result<(), Error> {
    if let Some(player) = state.entities.get(player_id) {
        let c = (255*min(player.score, 20)/20) as u8;
        canvas.set_draw_color(Color::RGB(c, c, c));
    } else {
        canvas.set_draw_color(Color::RGB(255, 0, 0));
    }
    canvas.clear();

    for entity in state.entities.values() {
        match entity.kind {
            EntityKind::Player => canvas.set_draw_color(Color::RGB(
                (255 * (8 - min(entity.power, 8)) / 8) as u8,
                (255 * (8 - min(entity.power, 8)) / 8) as u8,
                255,
            )),
            EntityKind::Monster => canvas.set_draw_color(Color::RGB(
                255,
                (255 * (3 - min(entity.agro / 320, 3)) / 3) as u8,
                (255 * (3 - min(entity.agro / 320, 3)) / 3) as u8,
            )),
            EntityKind::Power => canvas.set_draw_color(Color::RGB(0, 0, 255)),
        }
        canvas.fill(entity.hitbox)?;
        if entity.kind == EntityKind::Player {
            canvas.set_draw_color(Color::RGB(0, 255, 0));
            let b = entity.hitbox;
            let w = 4;
            let face: Rect = (b.index(-1, -1), b.index(1, -1) + (0, w)).into();
            let face = face.rotate(b.center(), entity.facing_direction * 90);
            canvas.fill(face)?;

            canvas.set_draw_color(Color::RGB(255, 255, 0));
            canvas.fill(entity.attack_box)?;
        }
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("demo", 800, 600)
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas().build()?;

    let mut state = State::default();
    let player_id = state.entities.insert(Entity {
        hitbox: Rect::new_with_size(400, 300, 32, 32),
        kind: EntityKind::Player,
        facing_direction: 0,
        attack_frame: None,
        attack_box: Rect::default(),
        agro: 0,
        score: 0,
        power: 0,
    });

    // Add monsters.
    state.entities.insert(Entity {
        hitbox: Rect::new_with_size(300, 200, 32, 32),
        kind: EntityKind::Monster,
        facing_direction: 0,
        attack_frame: None,
        attack_box: Rect::default(),
        agro: 0,
        score: 0,
        power: 0,
    });
    state.entities.insert(Entity {
        hitbox: Rect::new_with_size(500, 200, 32, 32),
        kind: EntityKind::Monster,
        facing_direction: 1,
        attack_frame: None,
        attack_box: Rect::default(),
        agro: 0,
        score: 0,
        power: 0,
    });
    state.entities.insert(Entity {
        hitbox: Rect::new_with_size(300, 400, 32, 32),
        kind: EntityKind::Monster,
        facing_direction: 2,
        attack_frame: None,
        attack_box: Rect::default(),
        agro: 0,
        score: 0,
        power: 0,
    });
    state.entities.insert(Entity {
        hitbox: Rect::new_with_size(500, 400, 32, 32),
        kind: EntityKind::Monster,
        facing_direction: 3,
        attack_frame: None,
        attack_box: Rect::default(),
        agro: 0,
        score: 0,
        power: 0,
    });

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;
    let mut control = Control::default();
    let mut frame_number: u64 = 0;
    loop {
        process_input(&mut event_pump, &mut control)?;
        if control.quit_input {
            break;
        }

        let size: Point = canvas.window().size().into();
        let rect = ((0, 0), size).into();

        // Note: start by processing collisions first. This is in effect
        // capturing the collisions from the *last frame*, which is important
        // so that you can actually see something you hit instead of
        // dissappearing off the screen before you even see it.
        process_collisions(&mut state, player_id);
        process_scripts(&mut state, player_id, rect, frame_number);
        process_action(&mut state, player_id, &control, rect);

        render(&mut canvas, &state, player_id)?;

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        frame_number += 1;
    }
    Ok(())
}
