use wgpu;

pub struct Sprite {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub uv: [f32; 4],
    pub color: [f32; 4],
    pub texture_view: wgpu::TextureView,
}

impl Sprite {
    pub fn new(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        uv: [f32; 4],
        color: [f32; 4],
        texture_view: wgpu::TextureView,
    ) -> Self {
        Self {
            x,
            y,
            width,
            height,
            uv,
            color,
            texture_view,
        }
    }

    pub fn rect_to_vertices(&self, screen_w: f32, screen_h: f32) -> (Vec<[f32; 8]>, Vec<u16>){
        let (uv_min_x, uv_min_y, uv_max_x, uv_max_y) = (self.uv[0], self.uv[1], self.uv[2], self.uv[3]);
        let color = self.color;

        let tl = [
            (self.x / screen_w) * 2.0 - 1.0,
            1.0 - (self.y / screen_h) * 2.0,
        ];
        let tr = [
            ((self.x + self.width) / screen_w) * 2.0 - 1.0,
            1.0 - (self.y / screen_h) * 2.0,
        ];
        let br = [
            ((self.x + self.width) / screen_w) * 2.0 - 1.0,
            1.0 - ((self.y + self.height) / screen_h) * 2.0,
        ];
        let bl = [
            (self.x / screen_w) * 2.0 - 1.0,
            1.0 - ((self.y + self.height) / screen_h) * 2.0,
        ];

        let vertices = vec![
            [tl[0], tl[1], uv_min_x, uv_min_y, color[0], color[1], color[2], color[3]],
            [tr[0], tr[1], uv_max_x, uv_min_y, color[0], color[1], color[2], color[3]],
            [br[0], br[1], uv_max_x, uv_max_y, color[0], color[1], color[2], color[3]],
            [bl[0], bl[1], uv_min_x, uv_max_y, color[0], color[1], color[2], color[3]],
        ];
        let indices = vec![0u16, 1, 2, 0, 2, 3];
        (vertices, indices)
    }
}