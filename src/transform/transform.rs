use macroquad::{math::Vec3, models::Mesh};

pub trait Transform {
    fn rotate(&mut self, angle: f32, axis: Vec3);
    fn scale(&mut self, x: f32, y: f32, z: f32);
    fn translate(&mut self, pos: Vec3);
}

impl Transform for Mesh {
    fn rotate(&mut self, angle: f32, axis: Vec3) {

    }

    fn scale(&mut self, x: f32, y: f32, z: f32) {
        let mut verts = self.vertices.clone();
        for v in 0..verts.len() {
            verts[v].position.x *= x;
            verts[v].position.y *= y;
            verts[v].position.z *= z;
        }
        self.vertices = verts;
    }
    
    fn translate(&mut self, pos: Vec3) {
        let mut verts = self.vertices.clone();
        for v in 0..verts.len() {
            verts[v].position.x += pos.x;
            verts[v].position.y += pos.y;
            verts[v].position.z += pos.z;
        }
        self.vertices = verts;
    }
}
