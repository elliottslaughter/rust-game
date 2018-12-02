use sdl2::pixels::Color;
use sdl2::render::{Canvas, RenderTarget};
use std::time::Duration;

use game::control::{process_input, Control};
use game::error::Error;
use game::point::Point;
use game::rect::Rect;
use game::state::{Entity, EntityId, EntityKind, State};

fn process_action(state: &mut State, player_id: EntityId, control: &Control, window: Rect) {
    if let Some(player) = state.entities.get_mut(player_id) {
        let delta = Point::new(
            control.left_right_input as i32,
            control.up_down_input as i32,
        );
        let lo = window.clamp(player.hitbox.lo + delta);
        let hi = lo + player.hitbox.size() - 1;
        player.hitbox = Rect::new(lo, hi);

        player.facing_direction = control.facing_input;
        player.attack = control.attack_input;
    }
}

fn process_collisions(state: &mut State, player_id: EntityId) {
    if let Some(&player) = state.entities.get(player_id) {
        let mut dead = false;
        for (id, entity) in state.entities.iter() {
            dead = id != player_id && entity.hitbox.has_intersection(player.hitbox);
            if dead {
                break;
            }
        }
        if dead {
            state.entities.remove(player_id);
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
            let facing_edge = match entity.facing_direction {
                0 => (b.lo, (b.hi.x, b.lo.y + w - 1)).into(),
                1 => (b.lo, (b.lo.x + w - 1, b.hi.y)).into(),
                2 => ((b.lo.x, b.hi.y - w), b.hi).into(),
                3 => ((b.hi.x - w, b.lo.y), b.hi).into(),
                _ => Rect::default(),
            };
            canvas.fill(facing_edge)?;

            if entity.attack {
                canvas.set_draw_color(Color::RGB(0, 0, 255));
                let u = 8;
                let facing_edge = match entity.facing_direction {
                    0 => ((b.lo.x, b.lo.y - u), (b.hi.x, b.lo.y - 1)).into(),
                    1 => ((b.lo.x - u, b.lo.y), (b.lo.x - 1, b.hi.y)).into(),
                    2 => ((b.lo.x, b.hi.y), (b.hi.x, b.hi.y + u)).into(),
                    3 => ((b.hi.x, b.lo.y), (b.hi.x + u, b.hi.y)).into(),
                    _ => Rect::default(),
                };
                canvas.fill(facing_edge)?;
            }
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
        attack: false,
    });

    // Add monsters.
    state.entities.insert(Entity {
        hitbox: Rect::new_with_size(300, 200, 32, 32),
        kind: EntityKind::Monster,
        facing_direction: 0,
        attack: false,
    });
    state.entities.insert(Entity {
        hitbox: Rect::new_with_size(500, 200, 32, 32),
        kind: EntityKind::Monster,
        facing_direction: 0,
        attack: false,
    });
    state.entities.insert(Entity {
        hitbox: Rect::new_with_size(300, 400, 32, 32),
        kind: EntityKind::Monster,
        facing_direction: 0,
        attack: false,
    });
    state.entities.insert(Entity {
        hitbox: Rect::new_with_size(500, 400, 32, 32),
        kind: EntityKind::Monster,
        facing_direction: 0,
        attack: false,
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
