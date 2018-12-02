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

    // Track the current control state for opposing direction keys,
    // i.e. +1/0/-1 for up/no input/down or left/no input/right.
    pub up_down_input: i8,
    pub left_right_input: i8,

    // Track whether a quit has been requested.
    pub quit_input: bool,
}

pub fn process_input(event_pump: &mut EventPump, last_control: Control) -> Result<Control, Error> {
    let mut control = last_control;
    for event in event_pump.poll_iter() {
        control = match event {
            Event::Quit { .. } => Control {
                quit_input: true,
                ..control
            },
            Event::KeyDown {
                keycode: Some(keycode),
                ..
            } => match keycode {
                Keycode::Escape => Control {
                    quit_input: true,
                    ..control
                },
                Keycode::Up => Control {
                    up_pressed: true,
                    up_down_input: if !control.up_pressed {
                        -1
                    } else {
                        control.up_down_input
                    },
                    ..control
                },
                Keycode::Down => Control {
                    down_pressed: true,
                    up_down_input: if !control.down_pressed {
                        1
                    } else {
                        control.up_down_input
                    },
                    ..control
                },
                Keycode::Left => Control {
                    left_pressed: true,
                    left_right_input: if !control.left_pressed {
                        -1
                    } else {
                        control.left_right_input
                    },
                    ..control
                },
                Keycode::Right => Control {
                    right_pressed: true,
                    left_right_input: if !control.right_pressed {
                        1
                    } else {
                        control.left_right_input
                    },
                    ..control
                },
                _ => control,
            },
            Event::KeyUp {
                keycode: Some(keycode),
                ..
            } => match keycode {
                Keycode::Up => Control {
                    up_pressed: false,
                    up_down_input: if control.up_down_input == -1 {
                        0
                    } else {
                        control.up_down_input
                    },
                    ..control
                },
                Keycode::Down => Control {
                    down_pressed: false,
                    up_down_input: if control.up_down_input == 1 {
                        0
                    } else {
                        control.up_down_input
                    },
                    ..control
                },
                Keycode::Left => Control {
                    left_pressed: false,
                    left_right_input: if control.left_right_input == -1 {
                        0
                    } else {
                        control.left_right_input
                    },
                    ..control
                },
                Keycode::Right => Control {
                    right_pressed: false,
                    left_right_input: if control.left_right_input == 1 {
                        0
                    } else {
                        control.left_right_input
                    },
                    ..control
                },
                _ => control,
            },
            _ => control,
        };
    }
    Ok(control)
}
