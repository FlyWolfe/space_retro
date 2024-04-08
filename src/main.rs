use bevy_ecs::{prelude::*, world::World};
use camera::camera::CameraState;
use macroquad::prelude::*;
use macroquad::window::miniquad::*;
use transform::transform::Transform;
use utils::{input_utils::MouseInput, mesh_utils::{BaseMeshMaterial, Model}};

mod camera;
mod player;
mod renderer;
mod transform;
mod utils;

use crate::player::player::{Player, PlayerBundle};

fn conf() -> Conf {
    Conf {
        window_title: String::from("Macroquad"),
        window_width: 1260,
        window_height: 768,
        fullscreen: false,
        ..Default::default()
    }
}
#[macroquad::main(conf)]
async fn main() {
    let mut last_mouse_position: Vec2 = mouse_position().into();

    let mut grabbed = true;
    set_cursor_grab(grabbed);
    show_mouse(false);

    let test_model = Model::new("test.obj", "res/").await;

    let mut world = World::new();

    let player = PlayerBundle {
        player: Player::new(BLUE, 200.),
        model: test_model,
        transform: Transform {
            position: vec3(0., 1., 0.),
            scale: Vec3::ONE * 3.,
            rotation: Quat::IDENTITY,
        },
    };

    let world_up = vec3(0.0, 1.0, 0.0);
    let yaw: f32 = 1.18;
    let pitch: f32 = 0.0;
    let roll: f32 = 0.0;

    let front = vec3(
        yaw.cos() * pitch.cos(),
        pitch.sin(),
        yaw.sin() * pitch.cos(),
    )
    .normalize();
    let right = front.cross(world_up).normalize();
    let up = right.cross(front).normalize();
    let camera = CameraState::new(
        vec3(0., 0., -50.),
        player.transform.position + vec3(0., 0., -50.),
        front,
        right,
        up,
        yaw,
        pitch,
        roll,
    );
    world.spawn(player);
    world.insert_resource(camera);

    world.spawn((
        Model::new("test.obj", "res/").await,
        Transform {
            position: vec3(10., 0., 0.),
            scale: Vec3::ONE * 10.,
            rotation: Quat::IDENTITY,
        }
    ));
    world.spawn((
        Model::new("test.obj", "res/").await,
        Transform {
            position: vec3(100., 0., 60.),
            scale: Vec3::ONE * 20.,
            rotation: Quat::IDENTITY,
        }
    ));
    world.spawn((
        Model::new("test.obj", "res/").await,
        Transform {
            position: vec3(500., 300., 500.),
            scale: Vec3::ONE * 200.,
            rotation: Quat::IDENTITY,
        }
    ));
    world.spawn((
        Model::new("test.obj", "res/").await,
        Transform {
            position: vec3(100., 10., 600.),
            scale: Vec3::ONE * 100.,
            rotation: Quat::IDENTITY,
        }
    ));
    world.spawn((
        Model::new("test.obj", "res/").await,
        Transform {
            position: vec3(100., 100., 0.),
            scale: Vec3::ONE * 1000.,
            rotation: Quat::IDENTITY,
        }
    ));

    let mouse_input: MouseInput = MouseInput {
        mouse_delta: Vec2::ZERO,
    };
    world.insert_resource(mouse_input);

    // Create a new Schedule, which defines an execution sptrategy for Systems
    let mut schedule = Schedule::default();
    schedule.add_systems(camera::camera::update_camera.before(player::player::player_input));
    schedule.add_systems(player::player::player_input.before(player::player::update_player));
    schedule.add_systems(player::player::update_player);
    schedule.add_systems(utils::mesh_utils::draw_models.after(player::player::update_player));
    schedule.add_systems(camera::camera::reset_camera.after(utils::mesh_utils::draw_models));

    let dither_material = load_material(
        ShaderSource::Glsl {
            vertex: DITHER_VERTEX_SHADER,
            fragment: DITHER_FRAGMENT_SHADER,
        },
        MaterialParams {
            ..Default::default()
        },
    )
    .unwrap();

    let mesh_material = load_material(
        ShaderSource::Glsl {
            vertex: utils::mesh_utils::MESH_VERTEX_SHADER,
            fragment: utils::mesh_utils::MESH_FRAGMENT_SHADER,
        },
        MaterialParams {
            uniforms: vec![
                ("LightColor".to_owned(), UniformType::Float3),
                ("ObjectColor".to_owned(), UniformType::Float3),
                ("ModelPos".to_owned(), UniformType::Float3),
            ],
            pipeline_params: PipelineParams {
                color_blend: Some(BlendState::new(
                    Equation::Add,
                    BlendFactor::Value(BlendValue::SourceAlpha),
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                )),
                depth_write: true,
                depth_test: Comparison::LessOrEqual,
                ..Default::default()
            },
            ..Default::default()
        },
    )
    .unwrap();

    mesh_material.set_uniform("LightColor", (1.0f32, 0.8f32, 0.4f32));
    mesh_material.set_uniform("ObjectColor", (1f32, 1f32, 1f32));

    let base_mesh_material = BaseMeshMaterial { material: mesh_material };
    
    world.insert_resource(base_mesh_material);

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Tab) {
            grabbed = !grabbed;
            set_cursor_grab(grabbed);
            show_mouse(!grabbed);
        }

        let mouse_position: Vec2 = mouse_position().into();
        world.get_resource_mut::<MouseInput>().unwrap().mouse_delta =
            mouse_position - last_mouse_position;
        last_mouse_position = mouse_position;

        clear_background(BLACK);

        //b.draw_m(&player.get_pos(), &front);
        //b.update(delta);
        // Back to screen space, render some text

        schedule.run(&mut world);

        draw_text(
            format!("Press <TAB> to toggle mouse grab: {}", grabbed).as_str(),
            10.0,
            48.0 + 0.0,
            30.0,
            WHITE,
        );
        draw_text(
            format!("FPS: {}", get_fps()).as_str(),
            10.0,
            48.0 + 24.0,
            30.0,
            WHITE,
        );

        gl_use_material(&dither_material);
        draw_rectangle(
            -screen_width() / 2.,
            -screen_height() / 2.,
            screen_width(),
            screen_height(),
            WHITE,
        );
        gl_use_default_material();

        next_frame().await
    }
}

/// Testing Screen Shaders
const DITHER_FRAGMENT_SHADER: &'static str = r#"#version 150
precision mediump float;

in vec2 vUv;
out vec4 diffuseColor;

uniform sampler2D _ScreenTexture;

float luma(vec3 color) {
    return dot(color, vec3(0.299, 0.587, 0.114));
}

float luma(vec4 color) {
    return dot(color.rgb, vec3(0.299, 0.587, 0.114));
}

float dither4x4(vec2 position, float brightness) {
    int x = int(mod(position.x, 4.0));
    int y = int(mod(position.y, 4.0));
    int index = x + y * 4;
    float limit = 0.0;

    if (x < 8) {
        if (index == 0) limit = 0.0625;
        if (index == 1) limit = 0.5625;
        if (index == 2) limit = 0.1875;
        if (index == 3) limit = 0.6875;
        if (index == 4) limit = 0.8125;
        if (index == 5) limit = 0.3125;
        if (index == 6) limit = 0.9375;
        if (index == 7) limit = 0.4375;
        if (index == 8) limit = 0.25;
        if (index == 9) limit = 0.75;
        if (index == 10) limit = 0.125;
        if (index == 11) limit = 0.625;
        if (index == 12) limit = 1.0;
        if (index == 13) limit = 0.5;
        if (index == 14) limit = 0.875;
        if (index == 15) limit = 0.375;
    }

    return brightness < limit ? 0.92 : 1.0;
}

vec3 dither4x4(vec2 position, vec3 color) {
    return color * dither4x4(position, luma(color));
}

vec4 dither4x4(vec2 position, vec4 color) {
    return vec4(color.rgb * dither4x4(position, luma(color)), color.a);
}

void main() {
    diffuseColor = dither4x4(
        gl_FragCoord.xy
      , texture(_ScreenTexture, vUv)
    );
}
"#;

const DITHER_VERTEX_SHADER: &'static str = "#version 150
precision mediump float;

in vec2 position;
out vec2 vUv;

void main() {
  vUv = (position + vec2(1.0)) / 2.0;
  gl_Position = vec4(position, 1.0, 1.0);
}
";
