use crate::game::Game;

pub struct UIItem {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: [f32; 4],
}

pub struct UI {
    pub items: Vec<UIItem>,
    pub score_digits: Vec<u8>,
}

impl UI {
    pub fn new() -> Self {
        Self{items: Vec::new(), score_digits: Vec::new()}
    }

    pub fn update(&mut self, game: &Game){
        self.items.clear();
        self.score_digits = game.score.to_string().bytes().map(|b| b - b'0').collect();

        if game.game_over {
            self.items.push(UIItem {x: 280.0, y: 250.0, width: 200.0, height: 60.0, color: [1.0, 0.0, 0.0, 0.8]});
            self.items.push(UIItem {x: 300.0, y: 320.0, width: 160.0, height: 30.0, color: [1.0, 1.0, 1.0, 0.8]});
        }
    }

    pub fn sprites(&self) -> &[UIItem]{
        &self.items
    }
}