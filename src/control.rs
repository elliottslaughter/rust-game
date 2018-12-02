use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

use crate::error::Error;

#[derive(Debug, Default)]
pub struct Control {
    // Track whether the corresponding keys are currently pressed,
    // since SDL2 apparently do this for us anymore. Set to true on
    // KeyDown and false on KeyUp.
    up_pressed: bool,
    down_pressed: bool,
    left_pressed: bool,
    right_pressed: bool,

    // Track the current control state for opposing direction keys.
    pub up_down_input: i32,    // -1/0/+1 for up/no input/down
    pub left_right_input: i32, // -1/0/+1 for left/no input/right

    // Track the current facing direction.
    pub facing_input: i32, // 0/1/2/3 for up/left/down/right

    // Track the attack status.
    pub attack_input: bool,

    // Track whether a quit has been requested.
    pub quit_input: bool,
}

pub fn process_input(event_pump: &mut EventPump, control: &mut Control) -> Result<(), Error> {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => {
                control.quit_input = true;
            }
            Event::KeyDown {
                keycode: Some(keycode),
                ..
            } => match keycode {
                Keycode::Escape => {
                    control.quit_input = true;
                }
                Keycode::Up => {
                    if !control.up_pressed {
                        control.up_down_input = -1;
                        if control.left_right_input == 0 {
                            control.facing_input = 0;
                        }
                    }
                    control.up_pressed = true;
                }
                Keycode::Down => {
                    if !control.down_pressed {
                        control.up_down_input = 1;
                        if control.left_right_input == 0 {
                            control.facing_input = 2;
                        }
                    }
                    control.down_pressed = true;
                }
                Keycode::Left => {
                    if !control.left_pressed {
                        control.left_right_input = -1;
                        if control.up_down_input == 0 {
                            control.facing_input = 1;
                        }
                    }
                    control.left_pressed = true;
                }
                Keycode::Right => {
                    if !control.right_pressed {
                        control.left_right_input = 1;
                        if control.up_down_input == 0 {
                            control.facing_input = 3;
                        }
                    }
                    control.right_pressed = true;
                }
                Keycode::Space => {
                    control.attack_input = true;
                }
                _ => {}
            },
            Event::KeyUp {
                keycode: Some(keycode),
                ..
            } => match keycode {
                Keycode::Up => {
                    control.up_pressed = false;
                    if control.up_down_input == -1 {
                        control.up_down_input = 0;
                    }
                }
                Keycode::Down => {
                    control.down_pressed = false;
                    if control.up_down_input == 1 {
                        control.up_down_input = 0;
                    }
                }
                Keycode::Left => {
                    control.left_pressed = false;
                    if control.left_right_input == -1 {
                        control.left_right_input = 0;
                    }
                }
                Keycode::Right => {
                    control.right_pressed = false;
                    if control.left_right_input == 1 {
                        control.left_right_input = 0;
                    }
                }
                Keycode::Space => {
                    control.attack_input = false;
                }
                _ => {}
            },
            _ => {}
        }
    }
    Ok(())
}
