extern crate sdl2;

use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use std::collections::HashSet;
use std::time::Duration;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let _window = video_subsystem.window("Mouse", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut events = sdl_context.event_pump().unwrap();

    let mut prev_buttons = HashSet::new();

    'running: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} => break 'running,
                _ => ()
            }
        }

        // get a mouse state
        let state = events.mouse_state();

        // Create a set of pressed Keys.
        let buttons = state.pressed_mouse_buttons().collect();

        // Get the difference between the new and old sets.
        let new_buttons = &buttons - &prev_buttons;
        let old_buttons = &prev_buttons - &buttons;

        if !new_buttons.is_empty() || !old_buttons.is_empty() {
            println!("X = {:?}, Y = {:?} : {:?} -> {:?}", state.x(), state.y(),  new_buttons, old_buttons);
        }

        prev_buttons = buttons;

        std::thread::sleep(Duration::from_millis(100));
    }
}
