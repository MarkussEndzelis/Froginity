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
    pub high_score_digits: Vec<u8>,
}

impl UI {
    pub fn new() -> Self {
        Self{items: Vec::new(), score_digits: Vec::new(), high_score_digits: Vec::new()}
    }

    pub fn update(&mut self, game: &Game){
        self.items.clear();
        self.score_digits = game.score.to_string().bytes().map(|b| b - b'0').collect();
        self.high_score_digits = game.high_score.to_string().bytes().map(|b| b - b'0').collect();

        if game.game_over {
            self.items.push(UIItem {x: 270.0, y: 250.0, width: 260.0, height: 60.0, color: [1.0, 0.0, 0.0, 0.8]});
            self.items.push(UIItem {x: 310.0, y: 320.0, width: 180.0, height: 34.0, color: [1.0, 1.0, 1.0, 0.85]});
        }
        if !game.game_started {
            self.items.push(UIItem{x: 210.0, y: 258.0, width: 380.0, height: 40.0, color: [0.0, 0.0, 0.0, 0.55]});
        }
    }

    pub fn sprites(&self) -> &[UIItem]{
        &self.items
    }
}