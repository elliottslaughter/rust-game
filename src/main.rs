use sdl2::pixels::Color;
use sdl2::render::{Canvas, RenderTarget};
use std::time::Duration;

use game::control::{process_input, Control};
use game::error::Error;
use game::point::Point;
use game::rect::Rect;
use game::state::{Entity, EntityId, EntityKind, State};

static ATTACK_FRAMES: &'static [(i32, i32, i32, i32)] = &[
    (12, -5, 4, 4),    // gap 0
    (10, -6, 5, 5),    // gap 0
    (8, -7, 6, 6),     // gap 0
    (4, -13, 8, 10),   // gap 2
    (0, -19, 8, 14),   // gap 4
    (-4, -21, 8, 16),  // gap 4
    (-8, -19, 8, 14),  // gap 4
    (-12, -13, 8, 10), // gap 2
    (-14, -7, 6, 6),   // gap 0
    (-16, -6, 5, 5),   // gap 0
    (-18, -5, 4, 4),   // gap 0
];

fn process_action(state: &mut State, player_id: EntityId, control: &Control, window: Rect) {
    if let Some(player) = state.entities.get_mut(player_id) {
        let delta = Point::new(control.left_right_input, control.up_down_input);
        let lo = window.clamp(player.hitbox.lo + delta);
        let hi = lo + player.hitbox.size() - 1;
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
                let top_center = b.lo + Point::new((b.width() as i32) / 2, 0);
                let attack: Rect = ATTACK_FRAMES[frame].into();
                (attack + top_center).rotate(b.center(), player.facing_direction * 90)
            }
            None => Rect::default(),
        };
    }
}

fn process_collisions(state: &mut State, player_id: EntityId) {
    if let Some(&player) = state.entities.get(player_id) {
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
                    break;
                }
            }
        }

        for id in dead {
            state.entities.remove(id);
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

fn render<T: RenderTarget>(canvas: &mut Canvas<T>, state: &State) -> Result<(), Error> {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    for entity in state.entities.values() {
        match entity.kind {
            EntityKind::Player => canvas.set_draw_color(Color::RGB(255, 255, 255)),
            EntityKind::Monster => canvas.set_draw_color(Color::RGB(255, 0, 0)),
        }
        canvas.fill(entity.hitbox)?;
        if entity.kind == EntityKind::Player {
            canvas.set_draw_color(Color::RGB(0, 255, 0));
            let b = entity.hitbox;
            let w = 4;
            let face: Rect = (b.lo, (b.hi.x, b.lo.y + w)).into();
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
    });

    // Add monsters.
    state.entities.insert(Entity {
        hitbox: Rect::new_with_size(300, 200, 32, 32),
        kind: EntityKind::Monster,
        facing_direction: 0,
        attack_frame: None,
        attack_box: Rect::default(),
    });
    state.entities.insert(Entity {
        hitbox: Rect::new_with_size(500, 200, 32, 32),
        kind: EntityKind::Monster,
        facing_direction: 0,
        attack_frame: None,
        attack_box: Rect::default(),
    });
    state.entities.insert(Entity {
        hitbox: Rect::new_with_size(300, 400, 32, 32),
        kind: EntityKind::Monster,
        facing_direction: 0,
        attack_frame: None,
        attack_box: Rect::default(),
    });
    state.entities.insert(Entity {
        hitbox: Rect::new_with_size(500, 400, 32, 32),
        kind: EntityKind::Monster,
        facing_direction: 0,
        attack_frame: None,
        attack_box: Rect::default(),
    });

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;
    let mut control = Control::default();
    loop {
        process_input(&mut event_pump, &mut control)?;
        if control.quit_input {
            break;
        }

        let size: Point = canvas.window().size().into();
        let rect = ((0, 0), size - 1).into();
        process_action(&mut state, player_id, &control, rect);

        process_collisions(&mut state, player_id);

        render(&mut canvas, &state)?;

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
