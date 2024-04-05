use bevy_ecs::system::{ResMut, Resource};
use bevy_ecs::{component::Component, system::Query};
use macroquad::{file, prelude::*, text};
use std::io::{BufReader, Cursor};
use std::path::Path;

use crate::transform::transform::Transform;
use crate::utils::file_utils::load_string;

#[derive(Resource)]
pub struct BaseMeshMaterial {
    pub material: Material,
}

#[derive(Component)]
pub struct Model {
    pub meshes: Vec<Mesh>,
}

impl Model {
    pub async fn new(file_name: &str, folder_path: &str) -> Self {
        // TODO: Proper error handling
        load_model(file_name, folder_path).await.unwrap()
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

pub fn draw_models(query: Query<(&Model, &Transform)>, base_mesh_material: ResMut<BaseMeshMaterial>) {
    for (model, transform) in query.iter() {
        base_mesh_material.material.set_uniform("ModelPos", <(f32, f32, f32)>::from(transform.position));
        gl_use_material(&base_mesh_material.material);
        model.draw(transform);
        gl_use_default_material();
    }
}

pub async fn load_model(file_name: &str, folder_path: &str) -> anyhow::Result<Model> {
    let obj_text = load_string(file_name, folder_path).await?;
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
            let mat_text: String = load_string(&p, folder_path).await.unwrap();
            tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(mat_text)))
        },
    )
    .await?;

    let mut textures: Vec<Texture2D> = Vec::new();
    for m in obj_materials? {
        let texture_path = &m.diffuse_texture.unwrap_or_default();
        let final_path = folder_path.to_owned() + texture_path;
        println!("{}", final_path);
        let diffuse_texture = load_texture(&final_path).await?;
        diffuse_texture.set_filter(FilterMode::Linear);
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
                    normal: vec3(
                        m.mesh.normals[i * 3],
                        m.mesh.normals[i * 3 + 1],
                        m.mesh.normals[i * 3 + 2],
                    ),
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

/// Basic Mesh Shaders
pub(crate) const MESH_FRAGMENT_SHADER: &'static str = r#"#version 330
precision mediump float;

in lowp vec2 uv;
in vec3 Normal;
in vec3 FragPos;

out vec4 diffuseColor;

uniform vec3 LightColor;
uniform vec3 ObjectColor;
uniform sampler2D Texture;

void main() {
    float ambientStrength = 0.1;
    vec3 ambient = ambientStrength * LightColor;

    //vec3 result = ambient * ObjectColor;
    
    vec3 norm = normalize(Normal);
    vec3 lightDir = vec3(-0.5, 0.5, 0.0);

    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * LightColor;

    vec3 result = (ambient + diffuse) * ObjectColor;
    //FragColor = vec4(result, 1.0);
    vec2 updatedUV = vec2(uv.x, 1.0 - uv.y);

    diffuseColor = vec4(result, 1.0) * texture(Texture, updatedUV);
}
"#;

pub(crate) const MESH_VERTEX_SHADER: &'static str = "#version 330
precision mediump float;

in vec3 position;
in vec2 texcoord;
in vec4 color0;
in vec3 normal;

out lowp vec2 uv;
out vec3 Normal;
out vec3 FragPos;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    uv = texcoord;
    Normal = normal;
    FragPos = vec3(Model * vec4(position, 1.0));
}
";
