use bevy_ecs::{prelude::*, world::World};
use camera::camera::CameraState;
use macroquad::prelude::*;
use transform::transform::Transform;
use utils::{input_utils::MouseInput, mesh_utils::Model};

mod camera;
mod player;
mod transform;
mod utils;

use crate::player::player::Player;

const BOOST: f32 = 2.0;
const FIRE_RATE1: f32 = 1.0;
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

    let mut test_model = Model::new("test.obj").await;
    test_model.scale(10., 10., 10.);

    let mut world = World::new();

    let player = Player::new(vec3(0., 1., 0.), BLUE, 200., test_model);

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
        player.get_pos() + vec3(0., 0., -50.),
        front,
        right,
        up,
        yaw,
        pitch,
        roll,
    );
    world.spawn(player);
    world.insert_resource(camera);

    let mouse_input: MouseInput = MouseInput {
        mouse_delta: Vec2::ZERO,
    };
    world.insert_resource(mouse_input);

    // Create a new Schedule, which defines an execution sptrategy for Systems
    let mut schedule = Schedule::default();
    schedule.add_systems(camera::camera::update_camera.before(player::player::player_input));
    schedule.add_systems(player::player::player_input.before(player::player::update_player));
    schedule.add_systems(player::player::update_player);
    schedule.add_systems(camera::camera::reset_camera.after(player::player::update_player));
    schedule.add_systems(player::player::draw_player.after(camera::camera::reset_camera));

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

        draw_cube(vec3(0., 0., 10.), vec3(2., 2., 2.), None, RED);
        draw_cube(vec3(100., 0., 60.), vec3(20., 20., 20.), None, GRAY);
        draw_sphere(vec3(500., 300., 500.), 500., None, ORANGE);
        draw_cube(vec3(100., 10., 600.), vec3(50., -50., 50.), None, PURPLE);
        draw_cube(vec3(100., 100., 0.), vec3(100., 100., 100.), None, YELLOW);

        draw_cube(vec3(0., 1., 6.), vec3(2., 2., 2.), None, RED);
        //b.draw_m(&player.get_pos(), &front);
        //b.update(delta);
        // Back to screen space, render some text

        schedule.run(&mut world);

        draw_text(
            format!("Press <TAB> to toggle mouse grab: {}", grabbed).as_str(),
            10.0,
            48.0 + 42.0,
            30.0,
            WHITE,
        );
        draw_text(
            format!("FPS: {}", get_fps()).as_str(),
            10.0,
            48.0 + 66.0,
            30.0,
            WHITE,
        );
        next_frame().await
    }
}
