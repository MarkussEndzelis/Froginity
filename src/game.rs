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
            width: 50.0,
            height: 42.0,
            y_vel: 0.0,
            on_ground: false,
        }
    }

    pub fn update(&mut self, dt: f32, input: &InputHandler) {
        let gravity = 650.0;
        self.y_vel += gravity * dt;

        if input.jump_pressed && self.on_ground {
            self.y_vel = -620.0;
            self.on_ground = false;
        }
        if !input.jump_held && self.y_vel < 0.0 {
            self.y_vel *= 0.8;
        }

        self.y += self.y_vel * dt;

        let ground_y = 500.0 - self.height;
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

pub struct Bird {
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
    pub birds: Vec<Bird>,
    pub score: u32,
    score_accum: f32,
    pub game_over: bool,
    pub speed: f32,
    pub obstacle_timer: f32,
    pub bird_timer: f32,
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
            score_accum: 0.0,
            birds: Vec::new(),
            bird_timer: 2.5,
        }
    }

    pub fn update(&mut self, dt: f32, input: &InputHandler){

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
                y: 500.0 - 40.0,
                width: 30.0,
                height: 40.0,
                active: true,
            };
            self.obstacles.push(obs);
        }

        self.bird_timer -= dt;
        if self.bird_timer <= 0.0 {
            let mut rng = rand::thread_rng();
            self.bird_timer = rng.gen_range(2.5..4.5);
            let fly_y = rng.gen_range(220.0..380.0);
            self.birds.push(Bird {x: 800.0, y: fly_y, width: 34.0, height: 24.0, active: true});
        }

        for b in &mut self.birds {
            b.x -= (self.speed * 1.15) * dt;
            if b.x + b.width < 0.0 {
                b.active = false;
            }
        }
        self.birds.retain(|b| b.active);

        for obs in &mut self.obstacles {
            obs.x -= self.speed * dt;
            if obs.x + obs.width < 0.0 {
                obs.active = false;
            }
        }
        self.obstacles.retain(|o| o.active);

        for obs in &self.obstacles {
            let fx = self.frog.x + 6.0;
            let fy = self.frog.y + 7.0;
            let fw = self.frog.width - 12.0;
            let fh = self.frog.height - 14.0;

            let ox = obs.x + 4.0;
            let oy = obs.y + 4.0;
            let ow = obs.width - 8.0;
            let oh = obs.height - 8.0;

            if fx + fw > ox && fx < ox + ow && fy + fh > oy && fy < oy + oh{
                self.game_over = true;
                break;
            }
        }

        for b in &self.birds {
            let fx = self.frog.x + 7.0;
            let fy = self.frog.y + 6.0;
            let fw = self.frog.width - 14.0;
            let fh = self.frog.height - 12.0;
            let bx = b.x + 5.0;
            let by = b.y + 5.0;
            let bw = b.width - 10.0;
            let bh = b.height - 10.0;
            if fx + fw > bx && fx < bx + bw && fy + fh > by && fy < by + bh {
                self.game_over = true;
                break;
            }
        }

        self.score_accum += self.speed * dt * 0.1;
        self.score = self.score_accum as u32;
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
        self.score_accum = 0.0;
        self.birds.clear();
        self.bird_timer = 2.5;
        
        let ground_width = 800.0;
        for i in 0..2 {
            self.grounds[i].x = i as f32 * ground_width;
        }
    }
}