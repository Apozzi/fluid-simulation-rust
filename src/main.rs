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
}

impl ApplicationContext for Application {
    const WINDOW_TITLE: &'static str = "Glium grid example";

    fn new(display: &Display<WindowSurface>) -> Self {
        // Define o tamanho da grid
        let grid_size = 201; 
        let cell_size = 0.01;

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Criação dos quadrados
        for row in 0..grid_size {
            for col in 0..grid_size {
                // Cálculo das posições para começar do topo (-1.0, 1.0) e avançar para baixo
                let x = 1.0 - col as f32 * cell_size;
                let y = 1.0 - row as f32 * cell_size;

                let v0 = vertices.len() as u32;
                vertices.push(Vertex {
                    position: [x, y],
                    color: [1.0, 0.0, 0.0],
                });
                vertices.push(Vertex {
                    position: [x + cell_size, y],
                    color: [0.0, 1.0, 0.0],
                });
                vertices.push(Vertex {
                    position: [x + cell_size, y - cell_size],
                    color: [0.0, 0.0, 1.0],
                });
                vertices.push(Vertex {
                    position: [x, y - cell_size],
                    color: [1.0, 1.0, 0.0],
                });

                indices.extend_from_slice(&[
                    v0, v0 + 1, v0 + 2, 
                    v0, v0 + 2, v0 + 3,
                ]);
            }
        }

        // Criação do VertexBuffer e IndexBuffer
        let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
        let index_buffer =
            glium::IndexBuffer::new(display, PrimitiveType::TrianglesList, &indices).unwrap();

        // Compilação dos shaders
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
        }
    }

    fn draw_frame(&mut self, display: &Display<WindowSurface>) {
        let mut frame = display.draw();

        // Limpa a tela com um fundo preto
        frame.clear_color(0.0, 0.0, 0.0, 1.0);

        // Desenha a grade
        frame
            .draw(
                &self.vertex_buffer,
                &self.index_buffer,
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