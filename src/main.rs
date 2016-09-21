#[macro_use]
extern crate glium;
extern crate image;

use glium::Surface;
use std::io::Cursor;

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

fn build_vertices() -> std::vec::Vec<Vertex> {
    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [0.0, 0.5] };
    let vertex3 = Vertex { position: [0.0, -0.25] };
    return vec![vertex1, vertex2, vertex3];
}

fn update_time(mut time: f32) -> f32 {
    time += 0.0002;
    if time > 0.5 {
        time = -0.5;
    }
    return time;
}

fn load_image<'a>(filepath: &String, lifetime: &'a Option<String>) -> glium::texture::RawImage2d<'a, u8> { 
    let image = image::load(Cursor::new(&include_bytes!("../test.png")[..]),
                        image::PNG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions)
}

fn main() {
    use glium::DisplayBuild;
    let display = glium::glutin::WindowBuilder::new()
        .build_glium().unwrap();
    let indices = glium::index::NoIndices(
        glium::index::PrimitiveType::TrianglesList
    );
        
    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        out vec2 my_attr;

        uniform mat4 matrix;

        void main() {
            my_attr = position;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;
    let fragment_shader_src = r#"
        #version 140

        in vec2 my_attr;
        out vec4 color;
        uniform mat4 matrix;

        void main() {
            color = vec4(my_attr, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(
        &display, vertex_shader_src, fragment_shader_src, None
    ).unwrap();

    let lifetime = None;
    let image = load_image(&"test.png".to_string(), &lifetime);

    let shape = build_vertices();

    let vertex_buffer = glium::VertexBuffer::new(
        &display, &shape
    ).unwrap();

    let mut time: f32 = -0.5;

    loop {
        time = update_time(time);

        let mut target = display.draw();

        target.clear_color(0.0, 0.0, 0.1, 0.1);
        let uniform = uniform! {
            matrix: [
                [time.cos(), time.sin(), 0.0, 0.0],
                [-time.sin(), time.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ]
        };
        target.draw(
            &vertex_buffer, &indices, &program,
            &uniform, &Default::default()
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
