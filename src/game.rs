use crate::input::InputHandler;
use rand::Rng;

pub struct Frog {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub y_vel: f32,
    pub on_ground: bool,
}

impl Frog {
    pub fn new() -> Self {
        Self {
            x: 200.0,
            y: 0.0,
            width: 40.0,
            height: 50.0,
            y_vel: 0.0,
            on_ground: false,
        }
    }

    pub fn update(&mut self, dt: f32, input: &InputHandler) {
        let gravity = 800.0;
        self.y_vel += gravity * dt;

        if input.jump_pressed && self.on_ground {
            self.y_vel = -450.0;
            self.on_ground = false;
        }
        if !input.jump_held && self.y_vel < 0.0 {
            self.y_vel *= 0.8;
        }

        self.y += self.y_vel * dt;

        let ground_y = 600.0 - self.height;
        if self.y >= ground_y {
            self.y = ground_y;
            self.y_vel = 0.0;
            self.on_ground = true;
        }
    }
}

pub struct Ground {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Ground {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {x, y, width, height}
    }
}

pub struct Obstacle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub active: bool,
}

pub struct Game {
    pub frog: Frog,
    pub grounds: Vec<Ground>,
    pub obstacles: Vec<Obstacle>,
    pub score: u32,
    pub game_over: bool,
    pub speed: f32,
    pub obstacle_timer: f32,
    pub difficulty_timer: f32,
}

impl Game {
    pub fn new() -> Self {
        let mut grounds = Vec::new();
        let ground_height = 100.0;
        let ground_width = 800.0;
        for i in 0..2 {
            grounds.push(Ground::new(i as f32 * ground_width, 600.0 - ground_height, ground_width, ground_height));
        }

        Self {
            frog: Frog::new(),
            grounds,
            obstacles: Vec::new(),
            score: 0,
            game_over: false,
            speed: 300.0,
            obstacle_timer: 1.5,
            difficulty_timer: 0.0,
        }
    }

    pub fn update(&mut self, input: &InputHandler){
        let dt = 1.0 / 60.0;

        if self.game_over {
            if input.restart_pressed{
                self.reset();
            }
            return;
        }

        self.difficulty_timer += dt;
        if self.difficulty_timer > 10.0 {
            self.difficulty_timer = 0.0;
            self.speed += 20.0;
        }

        self.frog.update(dt, input);
        for g in &mut self.grounds {
            g.x -= self.speed * dt;
            if g.x + g.width < 0.0 {
                g.x +=  g.width * 2.0;
            }
        }

        self.obstacle_timer -= dt;
        if self.obstacle_timer <= 0.0 {
            let mut rng = rand::thread_rng();
            let gap = rng.gen_range(0.7..2.0);
            self.obstacle_timer = gap;
            let obs = Obstacle {
                x: 800.0,
                y: 600.0 - 40.0,
                width: 30.0,
                height: 40.0,
                active: true,
            };
            self.obstacles.push(obs);
        }

        for obs in &mut self.obstacles {
            obs.x -= self.speed * dt;
            if obs.x + obs.width < 0.0 {
                obs.active = false;
            }
        }
        self.obstacles.retain(|o| o.active);

        for obs in &self.obstacles {
            if self.frog.x + self.frog.width > obs.x
                && self.frog.x < obs.x + obs.width
                && self.frog.y + self.frog.height > obs.y
                && self.frog.y < obs.y + obs.height
            {
                self.game_over = true;
                break;
            }
        }

        self.score = (self.score as f32 + self.speed * dt * 0.1) as u32;
    }

    pub fn reset(&mut self){
        self.frog.x = 200.0;
        self.frog.y = 0.0;
        self.frog.y_vel = 0.0;
        self.frog.on_ground = false;
        self.obstacles.clear();
        self.score = 0;
        self.game_over = false;
        self.speed = 300.0;
        self.obstacle_timer = 1.5;
        self.difficulty_timer = 0.0;
        
        let ground_width = 800.0;
        for i in 0..2 {
            self.grounds[i].x = i as f32 * ground_width;
        }
    }
}