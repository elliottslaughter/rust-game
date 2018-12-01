use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};
use sdl2::EventPump;
use std::time::Duration;

use game::error::Error;
use game::state::{Entity, EntityId, State};

fn process_input(
    event_pump: &mut EventPump,
    state: &mut State,
    player_id: EntityId,
) -> Result<bool, Error> {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return Ok(true), // quit
            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => {
                if let Some(mut player) = state.entities.get_mut(player_id) {
                    player.pos[1] += -4;
                }
            }
            Event::KeyDown {
                keycode: Some(Keycode::Down),
                ..
            } => {
                if let Some(mut player) = state.entities.get_mut(player_id) {
                    player.pos[1] += 4;
                }
            }
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => {
                if let Some(mut player) = state.entities.get_mut(player_id) {
                    player.pos[0] += -4;
                }
            }
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => {
                if let Some(mut player) = state.entities.get_mut(player_id) {
                    player.pos[0] += 4;
                }
            }
            _ => {}
        }
    }
    Ok(false) // don't quit
}

fn render<T: RenderTarget>(canvas: &mut Canvas<T>, state: &State, frame_number: u64) -> Result<(), Error> {
    let i = (frame_number & 0xFF) as u8;
    canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    for entity in state.entities.values() {
        canvas.fill_rect(Rect::new(
            entity.pos[0],
            entity.pos[1],
            entity.size[0],
            entity.size[1],
        ))?;
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
        pos: [400, 300],
        size: [32, 32],
    });

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;
    let mut frame_number = 0;
    loop {
        if process_input(&mut event_pump, &mut state, player_id)? {
            break;
        }

        render(&mut canvas, &state, frame_number)?;

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        frame_number += 1;
    }
    Ok(())
}
