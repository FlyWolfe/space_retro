use bevy_ecs::component::Component;
use macroquad::{
    math::{EulerRot, Quat, Vec3},
    models::Mesh,
};

use crate::utils::mesh_utils::Model;

#[derive(Component)]
pub struct Transform {
    pub position: Vec3,
    pub scale: Vec3,
    pub rotation: Quat,
}

// TODO: Fix having to recreate meshes repeatedly. Shouldn't need to be done
impl Transform {
    pub fn rotate(&mut self, angle: f32, axis: Vec3) {
        let rot = Quat::from_euler(
            EulerRot::XYZ,
            axis.x * angle,
            axis.y * angle,
            axis.z * angle,
        );

        self.rotation = rot.mul_quat(self.rotation);
    }

    pub fn rescale(&mut self, x: f32, y: f32, z: f32) {
        self.scale.x *= x;
        self.scale.y *= y;
        self.scale.z *= z;
    }

    pub fn translate(&mut self, translation: Vec3) {
        self.position.x += translation.x;
        self.position.y += translation.y;
        self.position.z += translation.z;
    }
}
