use bevy_ecs::system::{Query, ResMut, Resource};
use macroquad::math::Vec3;

use crate::player::player::Player;

#[derive(Resource)]
pub struct CameraState {
    camera_offset: Vec3,// = vec3(0., 0., -50.);
    camera_position: Vec3,// = player.get_pos() + camera_offset;
    front: Vec3,
    right: Vec3,
    up: Vec3,
}

impl CameraState {
    pub fn new() {
        
    }
}

pub fn update_camera(mut query: Query<&Player>, mut camera: ResMut<CameraState>) {
    camera.camera_position = query.get_single().unwrap().get_pos()
    + (camera.front * camera.camera_offset.z)
    + (-camera.right * camera.camera_offset.x)
    + (camera.up * camera.camera_offset.y);
}