use macroquad::prelude::*;
use glam::vec3;

pub struct Bullet
{
    size:Vec3,
    color: Color,
    velocity: Vec3,
    pos: Vec3,
    origin: Vec3,
}

impl Bullet {
    pub fn new(size:Vec3, color:Color, velocity:Vec3, pos:Vec3, origin: Vec3)->Self{
        Self {
            size,
            color,
            velocity,
            pos,
            origin,
        }
    }

    pub fn draw_m(&self)
    {
        draw_cube(self.pos, self.size, None, self.color);
    }

    pub fn update(&mut self, dt:f32){
        self.pos += self.velocity * vec3(1.0, 1.0, 1.0) * dt;
    }
}
