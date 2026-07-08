use winit::window::Window;
use wgpu;
use crate::renderer::Renderer;
use crate::game::Game;
use crate::input::InputHandler;
use crate::ui::UI,

pub struct State {
    pub window: Window,
    pub renderer: Renderer,
    pub game: Game,
    pub input: InputHandler,
    pub ui: UI,
}

impl State {
    pub async fn new(window: &Window) -> Self {
        let renderer = Renderer::new(window).await;
        let game = Game::new();
        let input = InputHandler::new();
        let ui = UI::new();

        Self {
            window: window.clone(),
            renderer,
            game,
            input,
            ui,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>){
        self.renderer.resize(new_size);
    }

    pub fn input(&mut self, event: winit::event::KeyEvent){
        self.input.handle_key_event(event);
    }

    pub fn update(&mut self){
        self.game.update(&self.input);
        self.ui.update(&self.game);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.renderer.render(&self.game, &self.ui)
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
}