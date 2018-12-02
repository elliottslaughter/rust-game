use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};
use std::time::Duration;

use game::control::{process_input, Control};
use game::error::Error;
use game::state::{Entity, EntityId, State};

fn process_action(
    state: &mut State,
    player_id: EntityId,
    control: &Control,
) {
    if let Some(player) = state.entities.get_mut(player_id) {
        player.hitbox.set_x(player.hitbox.x() + control.left_right_input as i32);
        player.hitbox.set_y(player.hitbox.y() + control.up_down_input as i32);
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

fn render<T: RenderTarget>(
    canvas: &mut Canvas<T>,
    state: &State,
    frame_number: u64,
) -> Result<(), Error> {
    let i = (frame_number & 0xFF) as u8;
    canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    for entity in state.entities.values() {
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
    });

    // Add monsters.
    state.entities.insert(Entity {
        hitbox: Rect::new(300, 200, 32, 32),
    });
    state.entities.insert(Entity {
        hitbox: Rect::new(500, 200, 32, 32),
    });
    state.entities.insert(Entity {
        hitbox: Rect::new(300, 400, 32, 32),
    });
    state.entities.insert(Entity {
        hitbox: Rect::new(500, 400, 32, 32),
    });

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;
    let mut frame_number = 0;
    let mut control = Control::default();
    loop {
        control = process_input(&mut event_pump, control)?;
        if control.quit_input {
            break;
        }

        process_action(&mut state, player_id, &control);

        process_collisions(&mut state, player_id);

        render(&mut canvas, &state, frame_number)?;

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        frame_number += 1;
    }
    Ok(())
}
