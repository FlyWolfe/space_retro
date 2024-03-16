use macroquad::prelude::*;

pub struct Player {
    position: Vec3,
    velocity: Vec3,
    max_speed: f32,
    //mesh: Mesh,
    color: Color,
    yaw: f32,
    pitch: f32,
    roll: f32,
    front: Vec3,
    right: Vec3,
    up: Vec3,
    stabilizing: bool,
    stabilizer_power: f32,
}

impl Player {
    pub fn new(position: Vec3, color: Color, max_speed: f32) -> Self {
        Self {
            position,
            velocity: vec3(0., 0., 0.),
            max_speed,
            color,
            yaw: 0.,
            pitch: 0.,
            roll: 0.,
            front: Vec3::Z,
            right: Vec3::X,
            up: Vec3::Y,
            stabilizing: false,
            stabilizer_power: 1.,
        }
    }

    pub fn draw(&self) {
        // TODO: Draw ship mesh, not just cube
        draw_cube(self.position, vec3(2., 5., 10.), None, self.color);
    }

    pub fn update(&mut self, dt: f32) {
        self.position += self.velocity * dt;
        if self.stabilizing {
            self.stabilize(dt);
        } else {
            self.stabilizing = true;
        }
    }

    pub fn stabilize(&mut self, dt: f32) {
        self.velocity = self
            .velocity
            .lerp(Vec3::ZERO, (self.stabilizer_power * dt).clamp(0., 1.));
    }

    pub fn add_force(&mut self, dir: Vec3, amount: f32) {
        //self.velocity += (dir.x * amount) * self.right;
        //self.velocity += (dir.y * amount) * self.up;
        //self.velocity += (dir.z * amount) * self.front;
        self.velocity += dir * amount;

        self.velocity = self.velocity.clamp_length(0., self.max_speed);
        if dir != Vec3::ZERO {
            self.stabilizing = false;
        }
    }

    pub fn get_pos(&self) -> Vec3 {
        self.position
    }

    pub fn get_yaw_pitch_roll(&self) -> Vec3 {
        vec3(self.yaw, self.pitch, self.roll)
    }

    pub fn set_yaw_pitch_roll(&mut self, yaw: f32, pitch: f32, roll: f32) {
        self.yaw = yaw;
        self.pitch = pitch;
        self.roll = roll;
    }

    pub fn get_up_vector(&self) -> Vec3 {
        self.up
    }
}
