use bevy_ecs::{
    component::Component,
    system::{Query, Res},
};
use macroquad::prelude::*;

use crate::{
    camera::camera::CameraState, transform::transform::Transform, utils::mesh_utils::Model,
};

const ACCELERATION: f32 = 200.0;

#[derive(Component)]
pub struct Player {
    position: Vec3,
    velocity: Vec3,
    max_speed: f32,
    model: Model,
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
    pub fn new(position: Vec3, color: Color, max_speed: f32, model: Model) -> Self {
        Self {
            position,
            velocity: vec3(0., 0., 0.),
            max_speed,
            model,
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
        self.model.draw();
    }

    pub fn update(&mut self, dt: f32) {
        let last_pos = self.position;
        self.position += self.velocity * dt;
        if self.stabilizing {
            self.stabilize(dt);
        } else {
            self.stabilizing = true;
        }

        self.model.translate(self.position - last_pos);
    }

    pub fn rotate(&mut self, angle: f32, axis: Vec3) {
        self.model.rotate(angle, axis);
    }

    pub fn get_rotation(&self) -> Vec3 {
        self.model.rotation
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

pub fn player_input(mut query: Query<&mut Player>, camera: Res<CameraState>) {
    let delta = get_frame_time();
    let mut player = query.single_mut();
    if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
        player.add_force(camera.front, delta * ACCELERATION);
    }
    if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
        player.add_force(-camera.front, delta * ACCELERATION);
    }
    if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
        player.add_force(-camera.right, delta * ACCELERATION);
    }
    if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
        player.add_force(camera.right, delta * ACCELERATION);
    }
    if is_key_down(KeyCode::Space) {
        player.add_force(camera.up, delta * ACCELERATION);
    }
    if is_key_down(KeyCode::LeftControl) {
        player.add_force(-camera.up, delta * ACCELERATION);
    }
}

pub fn update_player(mut query: Query<&mut Player>, camera: Res<CameraState>) {
    let mut player = query.single_mut();
    let mut q: Quat = Quat::IDENTITY;
    let a: Vec3 = Vec3::cross(player.get_rotation(), camera.front);
    q.x = a.x;
    q.y = a.y;
    q.z = a.z;
    q.w = f32::sqrt(
        (player.get_rotation().length().powf(2.)) * (player.get_rotation().length().powf(2.)),
    ) + Vec3::dot(player.get_rotation(), camera.front);
    let rot = Vec3::from(q.to_euler(EulerRot::XYZ));
    player.rotate(rot.length() * 10. * get_frame_time(), rot);

    player.update(get_frame_time());
}

pub fn draw_player(mut query: Query<&Player>) {
    let player = query.single();
    player.draw();
}
