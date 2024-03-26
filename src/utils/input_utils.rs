use bevy_ecs::system::Resource;
use macroquad::math::Vec2;

#[derive(Resource)]
pub struct MouseInput {
    pub mouse_delta: Vec2,
}
