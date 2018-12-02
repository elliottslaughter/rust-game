use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};
use sdl2::video::Window;
use std::cmp::{max, min};
use std::time::Duration;

use game::control::{process_input, Control};
use game::error::Error;
use game::state::{Entity, EntityId, EntityKind, State};

fn process_action(state: &mut State, player_id: EntityId, control: &Control, window: &Window) {
    if let Some(player) = state.entities.get_mut(player_id) {
        player.hitbox.set_x(min(
            max(player.hitbox.x() + control.left_right_input as i32, 0i32),
            (window.size().0 - player.hitbox.size().0) as i32,
        ));
        player.hitbox.set_y(min(
            max(player.hitbox.y() + control.up_down_input as i32, 0i32),
            (window.size().1 - player.hitbox.size().1) as i32,
        ));
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

fn render<T: RenderTarget>(canvas: &mut Canvas<T>, state: &State) -> Result<(), Error> {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    for entity in state.entities.values() {
        match entity.kind {
            EntityKind::Player => canvas.set_draw_color(Color::RGB(255, 255, 255)),
            EntityKind::Monster => canvas.set_draw_color(Color::RGB(255, 0, 0)),
        }
        canvas.fill_rect(entity.hitbox)?;
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
        hitbox: Rect::new(400, 300, 32, 32),
        kind: EntityKind::Player,
    });

    // Add monsters.
    state.entities.insert(Entity {
        hitbox: Rect::new(300, 200, 32, 32),
        kind: EntityKind::Monster,
    });
    state.entities.insert(Entity {
        hitbox: Rect::new(500, 200, 32, 32),
        kind: EntityKind::Monster,
    });
    state.entities.insert(Entity {
        hitbox: Rect::new(300, 400, 32, 32),
        kind: EntityKind::Monster,
    });
    state.entities.insert(Entity {
        hitbox: Rect::new(500, 400, 32, 32),
        kind: EntityKind::Monster,
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

        process_action(&mut state, player_id, &control, canvas.window());

        process_collisions(&mut state, player_id);

        render(&mut canvas, &state)?;

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
