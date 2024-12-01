#[macro_use]
extern crate glium;
mod support;

use glium::index::PrimitiveType;
use glium::{Display, Surface};
use glutin::surface::WindowSurface;
use support::{ApplicationContext, State};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}
implement_vertex!(Vertex, position, color);

struct Application {
    pub vertex_buffer: glium::VertexBuffer<Vertex>,
    pub index_buffer: glium::IndexBuffer<u32>,
    pub program: glium::Program,
    pub color_matrix: Vec<Vec<[f32; 3]>>,
    pub time: f32,
}

fn generate_color_matrix(grid_size: usize, time: f32) -> Vec<Vec<[f32; 3]>> {
    let mut color_matrix = vec![vec![[0.0, 0.0, 0.0]; grid_size]; grid_size];

    for row in 0..grid_size {
        for col in 0..grid_size {
            color_matrix[row][col] = [
                (row as f32 / grid_size as f32) + time.sin() * 0.1,  
                (col as f32 / grid_size as f32) + time.cos() * 0.1,  
                ((row + col) as f32 / (2 * grid_size) as f32) + time.sin() * 0.1 
            ];
        }
    }

    color_matrix
}

fn generate_grid_data(grid_size: usize, cell_size: f32, color_matrix: &Vec<Vec<[f32; 3]>>) -> (Vec<Vertex>, Vec<u32>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for row in 0..grid_size {
        for col in 0..grid_size {
            let x = 1.0 - col as f32 * cell_size;
            let y = 1.0 - row as f32 * cell_size;

            let cell_color = color_matrix[row][col];

            let v0 = vertices.len() as u32;
            vertices.push(Vertex {
                position: [x, y],
                color: cell_color,
            });
            vertices.push(Vertex {
                position: [x + cell_size, y],
                color: cell_color,
            });
            vertices.push(Vertex {
                position: [x + cell_size, y - cell_size],
                color: cell_color,
            });
            vertices.push(Vertex {
                position: [x, y - cell_size],
                color: cell_color,
            });

            indices.extend_from_slice(&[
                v0, v0 + 1, v0 + 2, 
                v0, v0 + 2, v0 + 3,
            ]);
        }
    }

    (vertices, indices)
}

impl ApplicationContext for Application {
    const WINDOW_TITLE: &'static str = "Glium grid example";


    fn new(display: &Display<WindowSurface>) -> Self {
            
        let grid_size = 201; 
        let cell_size = 0.01;

        let color_matrix = generate_color_matrix(grid_size, 0.0);
        let (vertices, indices) = generate_grid_data(grid_size, cell_size, &color_matrix);

        let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
        let index_buffer = glium::IndexBuffer::new(display, PrimitiveType::TrianglesList, &indices).unwrap();

        let program = program!(display,
            100 => {
                vertex: "
                    #version 100

                    attribute lowp vec2 position;
                    attribute lowp vec3 color;

                    varying lowp vec3 vColor;

                    void main() {
                        gl_Position = vec4(position, 0.0, 1.0);
                        vColor = color;
                    }
                ",

                fragment: "
                    #version 100
                    varying lowp vec3 vColor;

                    void main() {
                        gl_FragColor = vec4(vColor, 1.0);
                    }
                ",
            },
        )
        .unwrap();

        Self {
            vertex_buffer,
            index_buffer,
            program,
            color_matrix,
            time: 0.0,
        }
    }

    fn draw_frame(&mut self, display: &Display<WindowSurface>) {
        let mut frame = display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 1.0);
        self.time += 0.01;
        let grid_size = 201; 
        let cell_size = 0.01;
        let color_matrix = generate_color_matrix(grid_size, self.time);
        let (vertices, indices) = generate_grid_data(grid_size, cell_size, &color_matrix);
        let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
        let index_buffer = glium::IndexBuffer::new(display, PrimitiveType::TrianglesList, &indices).unwrap();

        frame
            .draw(
                &vertex_buffer,
                &index_buffer,
                &self.program,
                &uniform! {},
                &Default::default(),
            )
            .unwrap();

        frame.finish().unwrap();
    }
}

fn main() {
    State::<Application>::run_loop();
}