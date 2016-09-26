#[macro_use]
extern crate glium;
extern crate image;

use glium::Surface;
use std::io::Cursor;

mod teapot;

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);

fn build_vertices() -> std::vec::Vec<Vertex> {
    let vertex1 = Vertex { position: [-0.5, -0.5],  tex_coords: [0.0, 0.0] };
    let vertex2 = Vertex { position: [0.0, 0.5],    tex_coords: [0.0, 1.0] };
    let vertex3 = Vertex { position: [0.0, -0.25],  tex_coords: [1.0, 0.0] };
    return vec![vertex1, vertex2, vertex3];
}

fn update_time(mut time: f32) -> f32 {
    time += 0.0002;
    if time > 0.5 {
        time = -0.5;
    }
    return time;
}

fn load_image<'a>(image_data: &'a [u8]) -> glium::texture::RawImage2d<'a, u8> { 
    let image = image::load(Cursor::new(image_data),
                        image::PNG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions)
}

fn main() {
    use glium::DisplayBuild;
    let display = glium::glutin::WindowBuilder::new()
        .build_glium().unwrap();
        
    let vertex_shader_src = r#"
        #version 140

        in vec3 position;
        in vec3 normal;

        uniform mat4 matrix;

        void main() {
            gl_Position = matrix * vec4(position, 1.0);
        }
    "#;
    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(
        &display, vertex_shader_src, fragment_shader_src, None
    ).unwrap();

    let image_data = &include_bytes!("../test.png")[..];
    let image = load_image(image_data);

    let texture = glium::texture::Texture2d::new(&display, image).unwrap();

    //let shape = build_vertices();

    let position = glium::VertexBuffer::new(
        &display, &teapot::VERTICES
    ).unwrap();
    let normals = glium::VertexBuffer::new(
        &display, &teapot::NORMALS
    ).unwrap();
    let indices = glium::IndexBuffer::new(
        &display, glium::index::PrimitiveType::TrianglesList,
        &teapot::INDICES
    ).unwrap();

    let mut time: f32 = -0.5;

    loop {
        time = update_time(time);

        let mut target = display.draw();

        target.clear_color(0.0, 0.0, 0.1, 0.1);
        let uniforms = uniform! {
            matrix: [
                [0.01, 0.0, 0.0, 0.0],
                [0.0, 0.01, 0.0, 0.0],
                [0.0, 0.0, 0.01, 0.0],
                [0.0 , 0.0, 0.0, 1.0f32],
            ],
            tex: &texture,
        };
        target.draw(
            (&position, &normals), &indices, &program,
            &uniforms, &Default::default()
        ).unwrap();
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
