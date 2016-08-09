#[macro_use]
extern crate glium;
use glium::Surface;

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
        uniform float time;

        void main() {
            vec2 pos = position;
            pos.x += time;
            gl_Position = vec4(pos, 0.0, 1.0);
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

    let shape = build_vertices();

    let vertex_buffer = glium::VertexBuffer::new(
        &display, &shape
    ).unwrap();

    let mut time: f32 = -0.5;

    loop {
        time = update_time(time);

        let mut target = display.draw();

        target.clear_color(0.0, 0.0, 0.1, 0.1);
        let uniform = uniform! {time: time};
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
