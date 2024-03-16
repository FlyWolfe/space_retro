use macroquad::prelude::*;
// use glam::vec3;

const MOVE_SPEED: f32 = 0.1;
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
#[derive(Copy, Clone)]
pub struct Bullet
{
    size:Vec3,
    color: Color, 
    velocity: Vec3,
    pos: Vec3,
    age: f32,
}

impl Bullet {
    pub fn new(size:Vec3, color:Color, velocity:Vec3, pos:Vec3, age:f32)->Self{
        Self{
            size,
            color,
            velocity,
            pos,
            age,
        }
    }
pub fn draw_m(&self,x:&Vec3, y:&Vec3)
{

 draw_cube(self.pos, self.size, None, self.color);
}
pub fn update(&mut self,dt:f32){
   self.age += self.age + 1.0;
   self.pos += self.velocity * vec3(1.0, 1.0, 1.0) * self.age;
   if self.age > 4000.0
   {
       drop(&self);
   }
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
    let mut up;

    let mut position = vec3(0.0, 1.0, 0.0);
    let mut last_mouse_position: Vec2 = mouse_position().into();

    let mut grabbed = true;
    set_cursor_grab(grabbed);
    show_mouse(false);
        let mut b = Bullet{size:vec3(0.1, 0.1, 0.1), color:BLUE, velocity:vec3(0.0,0.0,0.5),pos:position,age:0.0};

    loop 
    {

        let delta = get_frame_time();
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Tab) {
            grabbed = !grabbed;
            set_cursor_grab(grabbed);
            show_mouse(!grabbed);
        }

        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            position += front * MOVE_SPEED;
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            position -= front * MOVE_SPEED;
        }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            position -= right * MOVE_SPEED;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            position += right * MOVE_SPEED;
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

        set_camera(&Camera3D {
            position: position,
            up: up,
            target: position + front,
            ..Default::default()
        });

        draw_plane(vec3(0., 0., 0.), vec2(100., 100.), None, DARKGREEN);

        draw_cube(vec3(0., 1., 6.), vec3(2., 2., 2.), None, RED);
        b.draw_m(&position, &front);
        b.update(delta);
        // Back to screen space, render some text

        set_default_camera();

        draw_text(
            format!("X: {} Y: {}", mouse_position.x, mouse_position.y).as_str(),
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
        next_frame().await
    }
}
