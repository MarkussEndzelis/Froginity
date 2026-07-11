use winit::event::{KeyEvent, ElementState};
use winit::keyboard::{KeyCode, PhysicalKey};

pub struct InputHandler {
    pub jump_pressed: bool,
    pub jump_held: bool,
    pub restart_pressed: bool,
    restart_held: bool,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            jump_pressed: false,
            jump_held: false,
            restart_pressed: false,
            restart_held: false,
        }
    }

    pub fn handle_key_event(&mut self, event: KeyEvent){
        let pressed = event.state == ElementState::Pressed;
        match event.physical_key {
            PhysicalKey::Code(KeyCode::Space) | PhysicalKey::Code(KeyCode::ArrowUp) | PhysicalKey::Code(KeyCode::KeyW) => {
                if pressed && !self.jump_held{
                    self.jump_pressed = true;
                }
                self.jump_held = pressed;
            }
            PhysicalKey::Code(KeyCode::KeyR) => {
                if pressed && !self.restart_held {
                    self.restart_pressed = true;
                }
                self.restart_held = pressed;
            }
            
            _ => {}
        }
    }

    pub fn end_frame(&mut self){
        self.jump_pressed = false;
        self.restart_pressed = false;
    }
}