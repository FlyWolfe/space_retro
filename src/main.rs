use macroquad::prelude::*;
use transform::transform::Transform;
use utils::mesh_utils::Model;

mod player;
mod transform;
mod utils;

use crate::player::player::Player;

const ACCELERATION: f32 = 200.0;
const BOOST: f32 = 2.0;
const LOOK_SPEED: f32 = 0.1;
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
    let mut x = 0.0;
    let mut switch = false;
    let bounds = 8.0;

    let world_up = vec3(0.0, 1.0, 0.0);
    let mut yaw: f32 = 1.18;
    let mut pitch: f32 = 0.0;

    let mut front = vec3(
        yaw.cos() * pitch.cos(),
        pitch.sin(),
        yaw.sin() * pitch.cos(),
    )
    .normalize();
    let mut right = front.cross(world_up).normalize();
    let mut up = right.cross(front).normalize();

    let mut last_mouse_position: Vec2 = mouse_position().into();

    let mut grabbed = true;
    set_cursor_grab(grabbed);
    show_mouse(false);

    let mut test_model = Model::new("test.obj").await;
    test_model.scale(10., 10., 10.);

    let mut player = Player::new(vec3(0., 1., 0.), BLUE, 200., test_model);

    loop 
    {
        let camera_offset = vec3(0., 0., -50.);
        let mut camera_position = player.get_pos() + camera_offset;

        let delta = get_frame_time();
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Tab) {
            grabbed = !grabbed;
            set_cursor_grab(grabbed);
            show_mouse(!grabbed);
        }

        camera_position = player.get_pos()
            + (front * camera_offset.z)
            + (-right * camera_offset.x)
            + (up * camera_offset.y);

        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            player.add_force(front, delta * ACCELERATION);
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            player.add_force(-front, delta * ACCELERATION);
        }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            player.add_force(-right, delta * ACCELERATION);
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            player.add_force(right, delta * ACCELERATION);
        }
        if is_key_down(KeyCode::Space) {
            player.add_force(up, delta * ACCELERATION);
        }
        if is_key_down(KeyCode::LeftControl) {
            player.add_force(-up, delta * ACCELERATION);
        }

        let mouse_position: Vec2 = mouse_position().into();
        let mouse_delta = mouse_position - last_mouse_position;
        last_mouse_position = mouse_position;

        yaw += mouse_delta.x * delta * LOOK_SPEED;
        pitch += mouse_delta.y * delta * -LOOK_SPEED;

        pitch = if pitch > 1.5 { 1.5 } else { pitch };
        pitch = if pitch < -1.5 { -1.5 } else { pitch };

        front = vec3(
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos(),
        )
        .normalize();

        right = front.cross(world_up).normalize();
        up = right.cross(front).normalize();

        x += if switch { 0.04 } else { -0.04 };
        if x >= bounds || x <= -bounds {
            switch = !switch;
        }

        clear_background(BLACK);

        // Going 3d!
        //player.update(delta);

        set_camera(&Camera3D {
            position: camera_position,
            up,
            target: camera_position + front * 10.,
            ..Default::default()
        });

        player.update(delta);

        draw_cube(vec3(0., 0., 10.), vec3(2., 2., 2.), None, RED);
        draw_cube(vec3(100., 0., 60.), vec3(20., 20., 20.), None, GRAY);
        draw_sphere(vec3(500., 300., 500.), 500., None, ORANGE);
        draw_cube(vec3(100., 10., 600.), vec3(50., -50., 50.), None, PURPLE);
        draw_cube(vec3(100., 100., 0.), vec3(100., 100., 100.), None, YELLOW);

        player.draw();
        let mut q: Quat = Quat::IDENTITY;
        let a: Vec3 = Vec3::cross(player.get_rotation(), front);
        q.x = a.x;
        q.y = a.y;
        q.z = a.z;
        q.w = f32::sqrt((player.get_rotation().length().powf(2.)) * (player.get_rotation().length().powf(2.))) + Vec3::dot(player.get_rotation(), front);
        let rot = Vec3::from(q.to_euler(EulerRot::XYZ));
        player.rotate(rot.length() * 10. * delta, rot);

        draw_cube(vec3(0., 1., 6.), vec3(2., 2., 2.), None, RED);
        //b.draw_m(&player.get_pos(), &front);
        //b.update(delta);
        // Back to screen space, render some text

        set_default_camera();

        draw_text(
            format!(
                "X: {} Y: {} Z: {}",
                player.get_pos().x.round(),
                player.get_pos().y.round(),
                player.get_pos().z.round(),
            )
            .as_str(),
            10.0,
            48.0 + 18.0,
            30.0,
            WHITE,
        );
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
