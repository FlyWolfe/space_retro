use macroquad::prelude::*;
use std::io::{BufReader, Cursor};
use bevy_ecs::component::Component;

use crate::utils::file_utils::load_string;

#[derive(Component)]
pub struct Model {
    pub meshes: Vec<Mesh>,
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

impl Model {
    pub async fn new(file_name: &str) -> Self {
        // TODO: Proper error handling
        load_model(file_name).await.unwrap()
    }

    pub fn draw(&self) {
        for mesh in &self.meshes {
            draw_mesh(&mesh);
        }
    }
}

pub async fn load_model(file_name: &str) -> anyhow::Result<Model> {
    let obj_text = load_string(file_name).await?;
    let obj_cursor = Cursor::new(obj_text);
    let mut obj_reader = BufReader::new(obj_cursor);

    let (models, obj_materials) = tobj::load_obj_buf_async(
        &mut obj_reader,
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
        |p| async move {
            let mat_text: String = load_string(&p).await.unwrap();
            tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(mat_text)))
        },
    )
    .await?;

    let mut textures: Vec<Texture2D> = Vec::new();
    for m in obj_materials? {
        let texture_path = &m.diffuse_texture.unwrap_or_default();
        let diffuse_texture = load_texture(&("res/".to_owned() + texture_path)).await?;
        diffuse_texture.set_filter(FilterMode::Nearest);
        //let normal_texture = Some(load_texture(&m.normal_texture, true, device, queue).await?);

        textures.push(diffuse_texture);
    }

    let meshes = models
        .into_iter()
        .map(|m| {
            let vertices = (0..m.mesh.positions.len() / 3)
                .map(|i| macroquad::models::Vertex {
                    position: vec3(
                        m.mesh.positions[i * 3],
                        m.mesh.positions[i * 3 + 1],
                        m.mesh.positions[i * 3 + 2],
                    ),
                    uv: vec2(m.mesh.texcoords[i * 2], m.mesh.texcoords[i * 2 + 1]),
                    color: WHITE,
                })
                .collect::<Vec<_>>();

            let mut indices: Vec<u16> = vec![];
            for i in m.mesh.indices {
                indices.push(i as u16);
            }

            Mesh {
                vertices,
                indices,
                texture: Some(textures[m.mesh.material_id.unwrap_or(0)].clone()),
            }
        })
        .collect::<Vec<_>>();

    Ok(Model {
        meshes,
        scale: vec3(1., 1., 1.),
        position: vec3(0., 0., 0.),
        rotation: vec3(1., 0., 0.),
    })
}
