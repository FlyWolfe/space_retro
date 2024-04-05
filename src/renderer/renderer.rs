mod raw_miniquad {
    use macroquad::{color::Color, miniquad};
    use miniquad::*;

    #[repr(C)]
    struct Vec2 {
        x: f32,
        y: f32,
    }

    #[repr(C)]
    struct Vec3 {
        x: f32,
        y: f32,
        z: f32,
    }

    #[repr(C)]
    struct Vertex {
        pos: Vec3,
        normal: Vec3,
        uv: Vec2,
        color: Color,
    }

    pub struct Stage {
        pub pipeline: Pipeline,
        pub bindings: Bindings,
    }

    impl Stage {
        pub fn new(ctx: &mut dyn RenderingBackend) -> Stage {
            #[rustfmt::skip]
            let vertices: [Vertex; 4] = [
                Vertex { pos : Vec3 { x: -0.5, y: -0.5, z: -0.5 }, normal: Vec3 { x: -0.5, y: -0.5, z: -0.5 }, uv: Vec2 { x: 0., y: 0. }, color: Color::from_rgba(100, 200, 30, 255) },
                Vertex { pos : Vec3 { x:  0.5, y: -0.5, z: -0.5 }, normal: Vec3 { x:  0.5, y: -0.5, z: -0.5 }, uv: Vec2 { x: 1., y: 0. }, color: Color::from_rgba(100, 200, 30, 255) },
                Vertex { pos : Vec3 { x:  0.5, y:  0.5, z:  0.5 }, normal: Vec3 { x:  0.5, y:  0.5, z:  0.5 }, uv: Vec2 { x: 1., y: 1. }, color: Color::from_rgba(100, 200, 30, 255) },
                Vertex { pos : Vec3 { x: -0.5, y:  0.5, z:  0.5 }, normal: Vec3 { x: -0.5, y:  0.5, z:  0.5 }, uv: Vec2 { x: 0., y: 1. }, color: Color::from_rgba(100, 200, 30, 255) },
            ];
            let vertex_buffer = ctx.new_buffer(
                BufferType::VertexBuffer,
                BufferUsage::Immutable,
                BufferSource::slice(&vertices),
            );

            let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
            let index_buffer = ctx.new_buffer(
                BufferType::IndexBuffer,
                BufferUsage::Immutable,
                BufferSource::slice(&indices[..]),
            );

            let pixels: [u8; 4 * 4 * 4] = [
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00,
                0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            ];
            let texture = ctx.new_texture_from_rgba8(4, 4, &pixels);

            let bindings = Bindings {
                vertex_buffers: vec![vertex_buffer],
                index_buffer,
                images: vec![texture],
            };

            let shader = ctx
                .new_shader(
                    miniquad::ShaderSource::Glsl {
                        vertex: shader::VERTEX,
                        fragment: shader::FRAGMENT,
                    },
                    shader::meta(),
                )
                .unwrap();

            let pipeline = ctx.new_pipeline(
                &[BufferLayout::default()],
                &[
                    VertexAttribute::new("pos", VertexFormat::Float2),
                    VertexAttribute::new("uv", VertexFormat::Float2),
                ],
                shader,
                Default::default(),
            );

            Stage { pipeline, bindings }
        }
    }

    pub mod shader {
        use macroquad::miniquad;
        use miniquad::*;

        pub const VERTEX: &str = r#"#version 100
attribute vec3 pos;
attribute vec3 normal;
attribute vec2 uv;

uniform vec2 offset;

varying lowp vec2 texcoord;

void main() {
    gl_Position = vec4(pos + offset, 0, 1);
    texcoord = uv;
}"#;

        pub const FRAGMENT: &str = r#"#version 100
varying lowp vec2 texcoord;

uniform sampler2D tex;

void main() {
    gl_FragColor = texture2D(tex, texcoord);
}"#;

        pub fn meta() -> ShaderMeta {
            ShaderMeta {
                images: vec!["tex".to_string()],
                uniforms: UniformBlockLayout {
                    uniforms: vec![UniformDesc::new("offset", UniformType::Float2)],
                },
            }
        }

        #[repr(C)]
        pub struct Uniforms {
            pub offset: (f32, f32),
        }
    }
}