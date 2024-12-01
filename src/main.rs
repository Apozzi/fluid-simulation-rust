#[macro_use]
extern crate glium;
mod support;

use glium::index::PrimitiveType;
use glium::{Display, Surface};
use glutin::surface::WindowSurface;
use support::{ApplicationContext, State};
use support::field::{ColorField2D, VectorField2D};

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
    pub color_field: ColorField2D,
    pub velocity_field: VectorField2D
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

fn generate_arrows(grid_size: usize, cell_size: f32, velocity_field: &VectorField2D) -> (Vec<Vertex>, Vec<u32>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for row in 0..velocity_field.height {
        for col in 0..velocity_field.width {
            let x = 1.0 - col as f32 * cell_size;
            let y = 1.0 - row as f32 * cell_size;

            let direction = velocity_field.field[row][col];
            let dx = direction[0] * 0.05;
            let dy = direction[1] * 0.05;

            let start = vertices.len() as u32;
            vertices.push(Vertex {
                position: [x, y],
                color: [1.0, 1.0, 1.0],
            });
            vertices.push(Vertex {
                position: [x + dx, y + dy],
                color: [1.0, 1.0, 1.0],
            });

            indices.push(start);
            indices.push(start + 1);

            // Ponta da flecha (duas linhas formando um "V")
            let arrow_size = 0.05;
            let left = [
                x + dx - dy * arrow_size,
                y + dy + dx * arrow_size,
            ];
            let right = [
                x + dx + dy * arrow_size,
                y + dy - dx * arrow_size,
            ];

            vertices.push(Vertex {
                position: left,
                color: [1.0, 1.0, 1.0],
            });
            vertices.push(Vertex {
                position: right,
                color: [1.0, 1.0, 1.0],
            });

            indices.push(start + 1);
            indices.push(start + 2);

            indices.push(start + 1);
            indices.push(start + 3);
        }
    }

    (vertices, indices)
}

impl ApplicationContext for Application {
    const WINDOW_TITLE: &'static str = "Glium grid example";


    fn new(display: &Display<WindowSurface>) -> Self {
        //-- Field
        let fieldWidth = 40;
        let fieldHeight = 40;

        let initial_color = 1.0;
        let mut color_field = ColorField2D::new(fieldHeight, fieldHeight, initial_color);
        let mut velocity_field = VectorField2D {
            width: fieldWidth,
            height: fieldHeight,
            field: vec![vec![[0.5, 0.5]; fieldWidth]; fieldHeight],
        };

        // -- Grid
            
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
            color_field,
            velocity_field
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

        let cell_size_field = 0.05;

        let (arrow_vertices, arrow_indices) = generate_arrows(self.velocity_field.width, cell_size_field, &self.velocity_field);
        let arrow_vertex_buffer = glium::VertexBuffer::new(display, &arrow_vertices).unwrap();
        let arrow_index_buffer = glium::IndexBuffer::new(display, PrimitiveType::LinesList, &arrow_indices).unwrap();

        frame
            .draw(
                &arrow_vertex_buffer,
                &arrow_index_buffer,
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