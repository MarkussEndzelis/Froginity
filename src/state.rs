use winit::window::Window;
use wgpu;
use crate::renderer::Renderer;
use crate::game::Game;
use crate::input::InputHandler;
use crate::ui::UI;
use std::sync::Arc;

pub const VIRTUAL_WIDTH: f32 = 800.0;
pub const VIRTUAL_HEIGHT: f32 = 600.0;

pub struct State {
    pub window: Arc<Window>,
    pub renderer: Renderer,
    pub game: Game,
    pub input: InputHandler,
    pub ui: UI,
    pub last_time: std::time::Instant,
}

impl State {
    pub async fn new(window: Arc<Window>) -> Self {
        let renderer = Renderer::new(&window).await;
        let game = Game::new();
        let input = InputHandler::new();
        let ui = UI::new();

        Self {
            window,
            renderer,
            game,
            input,
            ui,
            last_time: std::time::Instant::now(),
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>){
        self.renderer.resize(new_size);
    }

    pub fn input(&mut self, event: winit::event::KeyEvent){
        self.input.handle_key_event(event);
    }

    pub fn update(&mut self){
        let now = std::time::Instant::now();
        let dt = now.duration_since(self.last_time).as_secs_f32();
        self.last_time = now;

        self.game.update(dt, &self.input);
        self.ui.update(&self.game);
        self.input.end_frame();
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.renderer.render(&self.game, &self.ui)
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
}