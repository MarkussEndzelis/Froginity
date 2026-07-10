#![windows_subsystem = "windows"]

mod state;
mod renderer;
mod game;
mod sprite;
mod input;
mod ui;
mod sprites;

use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use std::sync::Arc;
use state::State;

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Froginity")
            .build(&event_loop)
            .unwrap(),
    );
    let mut state = pollster::block_on(State::new(window.clone()));

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent { event, window_id } if window_id == state.window().id() => {
                match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::Resized(physical_size) => state.resize(physical_size),
                    WindowEvent::RedrawRequested => {
                        state.update();
                        match state.render() {
                            Ok(_) => {}
                            Err(e) => eprintln!("Render error: {:?}", e),
                        }
                        state.window().request_redraw();
                    }
                    WindowEvent::KeyboardInput { event, .. } => {
                        state.input(event);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }).unwrap();
}