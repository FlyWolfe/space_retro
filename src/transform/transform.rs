use macroquad::{
    math::{EulerRot, Quat, Vec3},
    models::Mesh,
};

use crate::utils::mesh_utils::Model;

pub trait Transform {
    fn rotate(&mut self, angle: f32, axis: Vec3);
    fn scale(&mut self, x: f32, y: f32, z: f32);
    fn translate(&mut self, translation: Vec3);
}

// TODO: Fix having to recreate meshes repeatedly. Shouldn't need to be done
impl Transform for Model {
    fn rotate(&mut self, angle: f32, axis: Vec3) {
        let rot = Quat::from_euler(
            EulerRot::XYZ,
            axis.x * angle,
            axis.y * angle,
            axis.z * angle,
        );
        self.rotation = rot.mul_vec3(self.rotation);

        let mut new_meshes: Vec<Mesh> = vec![];
        for i in 0..self.meshes.len() {
            let mut new_m = Mesh {
                vertices: self.meshes[i].vertices.clone(),
                indices: self.meshes[i].indices.clone(),
                texture: self.meshes[i].texture.clone(),
            };
            for v in 0..new_m.vertices.len() {
                let new_pos =
                    rot.mul_vec3(new_m.vertices[v].position - self.position) + self.position;
                new_m.vertices[v].position = new_pos;
            }
            new_meshes.push(new_m);
        }
        self.meshes = new_meshes;
    }
    fn scale(&mut self, x: f32, y: f32, z: f32) {
        self.scale.x *= x;
        self.scale.y *= y;
        self.scale.z *= z;

        let mut new_meshes: Vec<Mesh> = vec![];
        for i in 0..self.meshes.len() {
            let mut new_m = Mesh {
                vertices: self.meshes[i].vertices.clone(),
                indices: self.meshes[i].indices.clone(),
                texture: self.meshes[i].texture.clone(),
            };
            for v in 0..new_m.vertices.len() {
                new_m.vertices[v].position.x *= x;
                new_m.vertices[v].position.y *= y;
                new_m.vertices[v].position.z *= z;
            }
            new_meshes.push(new_m);
        }
        self.meshes = new_meshes;
    }

    fn translate(&mut self, translation: Vec3) {
        self.position.x += translation.x;
        self.position.y += translation.y;
        self.position.z += translation.z;

        let mut new_meshes: Vec<Mesh> = vec![];
        for i in 0..self.meshes.len() {
            let mut new_m = Mesh {
                vertices: self.meshes[i].vertices.clone(),
                indices: self.meshes[i].indices.clone(),
                texture: self.meshes[i].texture.clone(),
            };
            for v in 0..new_m.vertices.len() {
                new_m.vertices[v].position.x += translation.x;
                new_m.vertices[v].position.y += translation.y;
                new_m.vertices[v].position.z += translation.z;
            }
            new_meshes.push(new_m);
        }
        self.meshes = new_meshes;
    }
}
