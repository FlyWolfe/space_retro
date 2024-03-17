use macroquad::models::Mesh;

pub trait Transform {
    fn rotate(&mut self);
    fn scale(&mut self);
}

impl Transform for Mesh {
    fn rotate(&mut self) {}

    fn scale(&mut self) {}
}
