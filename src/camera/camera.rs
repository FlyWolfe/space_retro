use bevy_ecs::{
    query::With,
    system::{Query, Res, ResMut, Resource},
};
use macroquad::{
    camera::{set_camera, set_default_camera, Camera3D},
    color::{GRAY, ORANGE, PURPLE, RED, WHITE, YELLOW},
    math::{vec3, Vec3},
    models::{draw_cube, draw_sphere},
    text::draw_text,
    time::get_frame_time,
};

use crate::{
    player::player::Player, transform::transform::Transform, utils::input_utils::MouseInput,
};

const LOOK_SPEED: f32 = 0.1;

#[derive(Resource)]
pub struct CameraState {
    camera_offset: Vec3,              // = vec3(0., 0., -50.);
    pub(crate) camera_position: Vec3, // = player.get_pos() + camera_offset;
    pub(crate) front: Vec3,
    pub(crate) right: Vec3,
    pub(crate) up: Vec3,
    yaw: f32,
    pitch: f32,
    roll: f32,
}

impl CameraState {
    pub fn new(
        camera_offset: Vec3,
        camera_position: Vec3,
        front: Vec3,
        right: Vec3,
        up: Vec3,
        yaw: f32,
        pitch: f32,
        roll: f32,
    ) -> Self {
        Self {
            camera_offset,
            camera_position,
            front,
            right,
            up,
            yaw,
            pitch,
            roll,
        }
    }
}

pub fn reset_camera(camera: Res<CameraState>) {
    set_default_camera();
}

pub fn update_camera(
    query: Query<&Transform, With<Player>>,
    mut camera: ResMut<CameraState>,
    mouse_input: Res<MouseInput>,
) {
    camera.yaw += mouse_input.mouse_delta.x * get_frame_time() * LOOK_SPEED;
    camera.pitch += mouse_input.mouse_delta.y * get_frame_time() * -LOOK_SPEED;

    camera.pitch = if camera.pitch > 1.5 {
        1.5
    } else {
        camera.pitch
    };
    camera.pitch = if camera.pitch < -1.5 {
        -1.5
    } else {
        camera.pitch
    };

    camera.front = vec3(
        camera.yaw.cos() * camera.pitch.cos(),
        camera.pitch.sin(),
        camera.yaw.sin() * camera.pitch.cos(),
    )
    .normalize();

    camera.right = camera.front.cross(Vec3::Y).normalize();
    camera.up = camera.right.cross(camera.front).normalize();

    let transform = query.get_single().unwrap();
    camera.camera_position = transform.position
        + (camera.front * camera.camera_offset.z)
        + (-camera.right * camera.camera_offset.x)
        + (camera.up * camera.camera_offset.y);

    set_camera(&Camera3D {
        position: camera.camera_position,
        up: camera.up,
        target: camera.camera_position + camera.front * 10.,
        ..Default::default()
    });
}
