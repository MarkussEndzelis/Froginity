use winit::event::{KeyEvent, ElementState};
use winit::keyboard::{KeyCode, PhysicalKey};

pub struct InputHandler {
    pub jump_pressed: bool,
    pub jump_held: bool,
    pub restart_pressed: bool,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            jump_pressed: false,
            jump_held: false,
            restart_pressed: false,
        }
    }

    pub fn handle_key_event(&mut self, event: KeyEvent){
        let pressed = event.state == ElementState::Pressed;
        match event.physical_key {
            PhysicalKey::Code(KeyCode::Space) | PhysicalKey::Code(KeyCode::ArrowUp) | PhysicalKey::Code(KeyCode::KeyW) => {
                self.jump_pressed = pressed && !self.jump_held;
                self.jump_held = pressed;
            }
            PhysicalKey::Code(KeyCode::KeyR) => {
                self.restart_pressed = pressed;
            }
            _ => {}
        }
    }
}