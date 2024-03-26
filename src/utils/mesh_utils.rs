use bevy_ecs::{component::Component, system::Query};
use macroquad::prelude::*;
use std::io::{BufReader, Cursor};

use crate::transform::transform::Transform;
use crate::utils::file_utils::load_string;

#[derive(Component)]
pub struct Model {
    pub meshes: Vec<Mesh>,
}

impl Model {
    pub async fn new(file_name: &str) -> Self {
        // TODO: Proper error handling
        load_model(file_name).await.unwrap()
    }

    pub fn draw(&self, transform: &Transform) {
        for mesh in &self.meshes {
            draw_mesh(&get_transformed_mesh(mesh, transform));
        }
    }
}

pub fn get_transformed_mesh(mesh: &Mesh, transform: &Transform) -> Mesh {
    let mut new_m = Mesh {
        vertices: mesh.vertices.clone(),
        indices: mesh.indices.clone(),
        texture: mesh.texture.clone(),
    };

    for v in 0..new_m.vertices.len() {
        new_m.vertices[v].position.x *= transform.scale.x;
        new_m.vertices[v].position.y *= transform.scale.y;
        new_m.vertices[v].position.z *= transform.scale.z;

        new_m.vertices[v].position = transform
            .rotation
            .mul_vec3(new_m.vertices[v].position - transform.position)
            + transform.position;

        new_m.vertices[v].position.x += transform.position.x;
        new_m.vertices[v].position.y += transform.position.y;
        new_m.vertices[v].position.z += transform.position.z;
    }

    return new_m;
}

pub fn draw_models(query: Query<(&Model, &Transform)>) {
    for (model, transform) in query.iter() {
        model.draw(transform);
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

    Ok(Model { meshes })
}
