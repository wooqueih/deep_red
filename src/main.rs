mod chess;

use chess::*;

#[macro_use]
extern crate glium;
use chess::PieceWithTeam;
use glium::glutin;
use glium::Surface;
use image;
use std::{
    collections::HashMap,
    io::Cursor,
    time::{Duration, Instant},
};

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

struct Shape {
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
}
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum PieceOrBoard {
    Board,
    Piece(PieceWithTeam),
}

const PHYSICAL_WINDOW_SIZE: glutin::dpi::PhysicalSize<f64> =
    glutin::dpi::PhysicalSize::new(1000.0, 1000.0);

fn main() {
    let mut raw_textures = HashMap::new();
    raw_textures.insert(
        PieceOrBoard::Board,
        include_bytes!("../png/schach.png").to_vec(),
    );
    raw_textures.insert(
        PieceOrBoard::Piece(PieceWithTeam {
            piece: Piece::King(RochadeAbility::Unable),
            team: Team::White,
        }),
        include_bytes!("../png/Chess_klt45.png").to_vec(),
    );
    raw_textures.insert(
        PieceOrBoard::Piece(PieceWithTeam {
            piece: Piece::King(RochadeAbility::Unable),
            team: Team::Black,
        }),
        include_bytes!("../png/Chess_kdt45.png").to_vec(),
    );
    raw_textures.insert(
        PieceOrBoard::Piece(PieceWithTeam {
            piece: Piece::King(RochadeAbility::Able),
            team: Team::White,
        }),
        include_bytes!("../png/Chess_klt45.png").to_vec(),
    );
    raw_textures.insert(
        PieceOrBoard::Piece(PieceWithTeam {
            piece: Piece::King(RochadeAbility::Able),
            team: Team::Black,
        }),
        include_bytes!("../png/Chess_kdt45.png").to_vec(),
    );

    raw_textures.insert(
        PieceOrBoard::Piece(PieceWithTeam {
            piece: Piece::Queen,
            team: Team::White,
        }),
        include_bytes!("../png/Chess_qlt45.png").to_vec(),
    );

    raw_textures.insert(
        PieceOrBoard::Piece(PieceWithTeam {
            piece: Piece::Queen,
            team: Team::Black,
        }),
        include_bytes!("../png/Chess_qdt45.png").to_vec(),
    );

    raw_textures.insert(
        PieceOrBoard::Piece(PieceWithTeam {
            piece: Piece::Rook(RochadeAbility::Unable),
            team: Team::White,
        }),
        include_bytes!("../png/Chess_rlt45.png").to_vec(),
    );

    raw_textures.insert(
        PieceOrBoard::Piece(PieceWithTeam {
            piece: Piece::Rook(RochadeAbility::Unable),
            team: Team::Black,
        }),
        include_bytes!("../png/Chess_rdt45.png").to_vec(),
    );
    raw_textures.insert(
        PieceOrBoard::Piece(PieceWithTeam {
            piece: Piece::Rook(RochadeAbility::Able),
            team: Team::White,
        }),
        include_bytes!("../png/Chess_rlt45.png").to_vec(),
    );

    raw_textures.insert(
        PieceOrBoard::Piece(PieceWithTeam {
            piece: Piece::Rook(RochadeAbility::Able),
            team: Team::Black,
        }),
        include_bytes!("../png/Chess_rdt45.png").to_vec(),
    );

    raw_textures.insert(
        PieceOrBoard::Piece(PieceWithTeam {
            piece: Piece::Bishop,
            team: Team::White,
        }),
        include_bytes!("../png/Chess_blt45.png").to_vec(),
    );

    raw_textures.insert(
        PieceOrBoard::Piece(PieceWithTeam {
            piece: Piece::Bishop,
            team: Team::Black,
        }),
        include_bytes!("../png/Chess_bdt45.png").to_vec(),
    );

    raw_textures.insert(
        PieceOrBoard::Piece(PieceWithTeam {
            piece: Piece::Horse,
            team: Team::White,
        }),
        include_bytes!("../png/Chess_nlt45.png").to_vec(),
    );

    raw_textures.insert(
        PieceOrBoard::Piece(PieceWithTeam {
            piece: Piece::Horse,
            team: Team::Black,
        }),
        include_bytes!("../png/Chess_ndt45.png").to_vec(),
    );

    raw_textures.insert(
        PieceOrBoard::Piece(PieceWithTeam {
            piece: Piece::Pawn(EnPassanteVulnerability::Invulnerable),
            team: Team::White,
        }),
        include_bytes!("../png/Chess_plt45.png").to_vec(),
    );

    raw_textures.insert(
        PieceOrBoard::Piece(PieceWithTeam {
            piece: Piece::Pawn(EnPassanteVulnerability::Invulnerable),
            team: Team::Black,
        }),
        include_bytes!("../png/Chess_pdt45.png").to_vec(),
    );

    raw_textures.insert(
        PieceOrBoard::Piece(PieceWithTeam {
            piece: Piece::Pawn(EnPassanteVulnerability::Vulnerable),
            team: Team::White,
        }),
        include_bytes!("../png/Chess_plt45.png").to_vec(),
    );

    raw_textures.insert(
        PieceOrBoard::Piece(PieceWithTeam {
            piece: Piece::Pawn(EnPassanteVulnerability::Vulnerable),
            team: Team::Black,
        }),
        include_bytes!("../png/Chess_pdt45.png").to_vec(),
    );

    let mut cursor_position: glutin::dpi::PhysicalPosition<f64> =
        glutin::dpi::PhysicalPosition::new(0.0, 0.0);
    //let mut window_size: glutin::dpi::PhysicalSize<f64> = glutin::dpi::PhysicalSize::new(0.0, 0.0);

    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new()
        .with_resizable(false)
        .with_title("deep_red")
        .with_inner_size(PHYSICAL_WINDOW_SIZE);
    let context_builder = glutin::ContextBuilder::new();
    let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

    let plane = Shape {
        vertices: vec![
            Vertex {
                position: [-1.0, -1.0],
            },
            Vertex {
                position: [1.0, -1.0],
            },
            Vertex {
                position: [1.0, 1.0],
            },
            Vertex {
                position: [-1.0, 1.0],
            },
        ],
        indices: vec![0, 1, 2, 2, 3, 0],
    };
    let vertex_buffer = glium::VertexBuffer::new(&display, &plane.vertices).unwrap();
    let indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &plane.indices,
    )
    .unwrap();

    let mut t: f32 = 0.0;
    let program = glium::Program::from_source(
        &display,
        include_str!("vertex_shader.glsl"),
        include_str!("fragment_shader.glsl"),
        None,
    )
    .unwrap();

    let mut frames_delta_time = Duration::from_millis(5);

    event_loop.run(move |ev, _, control_flow| {
        let now = Instant::now();
        *control_flow = glutin::event_loop::ControlFlow::Wait;

        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                glutin::event::WindowEvent::CursorMoved { position, .. } => {
                    cursor_position = position;
                }
                _ => return,
            },
            _ => (),
        }
        println!("{},{}", cursor_position.x, cursor_position.y);
        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ],
            u_light:[-1.0, 0.4, 0.9f32],
        };
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
        frames_delta_time = now.elapsed();
        t += 0.000002 * frames_delta_time.as_micros() as f32;
    });
}
