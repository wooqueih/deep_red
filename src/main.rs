mod chess;

use chess::*;

#[macro_use]
extern crate glium;
extern crate image;
use chess::PieceWithTeam;
use glium::glutin::dpi::PhysicalPosition;
use glium::glutin::{self, dpi::LogicalPosition};
use glium::Surface;
use std::{
    collections::HashMap,
    io::Cursor,
    time::{Duration, Instant},
};

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
    texture_position: [f32; 2],
}
implement_vertex!(Vertex, position, texture_position);

struct Shape {
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
}
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum PieceOrBoard {
    Blue,
    Red,
    Board,
    Piece(PieceWithTeam),
}

const TILE_SIZE: f32 = 0.25;
const LOGICAL_WINDOW_SIZE: glutin::dpi::LogicalSize<f64> =
    glutin::dpi::LogicalSize::new(500.0, 500.0);

fn main() {
    let all_piece_or_board_states_with_respective_file = [
        (
            PieceOrBoard::Board,
            include_bytes!("../png/schach.png").to_vec(),
        ),
        (
            PieceOrBoard::Blue,
            include_bytes!("../png/blue.png").to_vec(),
        ),
        (PieceOrBoard::Red, include_bytes!("../png/red.png").to_vec()),
        (
            PieceOrBoard::Piece(PieceWithTeam {
                piece: Piece::King(RochadeAbility::Unable),
                team: Team::White,
            }),
            include_bytes!("../png/Chess_klt45.png").to_vec(),
        ),
        (
            PieceOrBoard::Piece(PieceWithTeam {
                piece: Piece::King(RochadeAbility::Unable),
                team: Team::Black,
            }),
            include_bytes!("../png/Chess_kdt45.png").to_vec(),
        ),
        (
            PieceOrBoard::Piece(PieceWithTeam {
                piece: Piece::King(RochadeAbility::Able),
                team: Team::White,
            }),
            include_bytes!("../png/Chess_klt45.png").to_vec(),
        ),
        (
            PieceOrBoard::Piece(PieceWithTeam {
                piece: Piece::King(RochadeAbility::Able),
                team: Team::Black,
            }),
            include_bytes!("../png/Chess_kdt45.png").to_vec(),
        ),
        (
            PieceOrBoard::Piece(PieceWithTeam {
                piece: Piece::Queen,
                team: Team::White,
            }),
            include_bytes!("../png/Chess_qlt45.png").to_vec(),
        ),
        (
            PieceOrBoard::Piece(PieceWithTeam {
                piece: Piece::Queen,
                team: Team::Black,
            }),
            include_bytes!("../png/Chess_qdt45.png").to_vec(),
        ),
        (
            PieceOrBoard::Piece(PieceWithTeam {
                piece: Piece::Rook(RochadeAbility::Unable),
                team: Team::White,
            }),
            include_bytes!("../png/Chess_rlt45.png").to_vec(),
        ),
        (
            PieceOrBoard::Piece(PieceWithTeam {
                piece: Piece::Rook(RochadeAbility::Unable),
                team: Team::Black,
            }),
            include_bytes!("../png/Chess_rdt45.png").to_vec(),
        ),
        (
            PieceOrBoard::Piece(PieceWithTeam {
                piece: Piece::Rook(RochadeAbility::Able),
                team: Team::White,
            }),
            include_bytes!("../png/Chess_rlt45.png").to_vec(),
        ),
        (
            PieceOrBoard::Piece(PieceWithTeam {
                piece: Piece::Rook(RochadeAbility::Able),
                team: Team::Black,
            }),
            include_bytes!("../png/Chess_rdt45.png").to_vec(),
        ),
        (
            PieceOrBoard::Piece(PieceWithTeam {
                piece: Piece::Bishop,
                team: Team::White,
            }),
            include_bytes!("../png/Chess_blt45.png").to_vec(),
        ),
        (
            PieceOrBoard::Piece(PieceWithTeam {
                piece: Piece::Bishop,
                team: Team::Black,
            }),
            include_bytes!("../png/Chess_bdt45.png").to_vec(),
        ),
        (
            PieceOrBoard::Piece(PieceWithTeam {
                piece: Piece::Horse,
                team: Team::White,
            }),
            include_bytes!("../png/Chess_nlt45.png").to_vec(),
        ),
        (
            PieceOrBoard::Piece(PieceWithTeam {
                piece: Piece::Horse,
                team: Team::Black,
            }),
            include_bytes!("../png/Chess_ndt45.png").to_vec(),
        ),
        (
            PieceOrBoard::Piece(PieceWithTeam {
                piece: Piece::Pawn(EnPassanteVulnerability::Invulnerable),
                team: Team::White,
            }),
            include_bytes!("../png/Chess_plt45.png").to_vec(),
        ),
        (
            PieceOrBoard::Piece(PieceWithTeam {
                piece: Piece::Pawn(EnPassanteVulnerability::Invulnerable),
                team: Team::Black,
            }),
            include_bytes!("../png/Chess_pdt45.png").to_vec(),
        ),
        (
            PieceOrBoard::Piece(PieceWithTeam {
                piece: Piece::Pawn(EnPassanteVulnerability::Vulnerable),
                team: Team::White,
            }),
            include_bytes!("../png/Chess_plt45.png").to_vec(),
        ),
        (
            PieceOrBoard::Piece(PieceWithTeam {
                piece: Piece::Pawn(EnPassanteVulnerability::Vulnerable),
                team: Team::Black,
            }),
            include_bytes!("../png/Chess_pdt45.png").to_vec(),
        ),
    ];

    let mut game_state = GameState::new();

    let mut hovered_tile: TilePosition = TilePosition {
        letter: 0,
        number: 0,
    };
    let mut cursor_position: glutin::dpi::LogicalPosition<f64> =
        glutin::dpi::LogicalPosition::new(0.0, 0.0);

    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new()
        .with_resizable(false)
        .with_title("deep_red")
        .with_inner_size(LOGICAL_WINDOW_SIZE);
    let context_builder = glutin::ContextBuilder::new();
    let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

    let plane = Shape {
        vertices: vec![
            Vertex {
                position: [-1.0, -1.0],
                texture_position: [0.0, 0.0],
            },
            Vertex {
                position: [1.0, -1.0],
                texture_position: [1.0, 0.0],
            },
            Vertex {
                position: [1.0, 1.0],
                texture_position: [1.0, 1.0],
            },
            Vertex {
                position: [-1.0, 1.0],
                texture_position: [0.0, 1.0],
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

    let program = glium::Program::from_source(
        &display,
        include_str!("vertex_shader.glsl"),
        include_str!("fragment_shader.glsl"),
        None,
    )
    .unwrap();

    let mut all_textures = HashMap::new();
    for (key, value) in all_piece_or_board_states_with_respective_file {
        let image = image::load(Cursor::new(&value), image::ImageFormat::Png)
            .unwrap()
            .to_rgba8();
        let image_dimensions = image.dimensions();
        let image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();
        all_textures.insert(key, texture);
    }

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
                    cursor_position = position.to_logical(1.0);
                    hovered_tile = get_hovered_tile(&cursor_position);
                }
                _ => return,
            },
            _ => (),
        }
        //println!("{},{}", cursor_position.x, cursor_position.y);
        let matrix = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ];
        let Some(texture) = all_textures.get(&PieceOrBoard::Board) else {
            panic!("texture does not exist");
        };
        let uniforms = uniform! {
            matrix: matrix,
            tex: texture,
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
        let draw_parameters = glium::DrawParameters {
            blend: glium::draw_parameters::Blend::alpha_blending(),
            ..glium::draw_parameters::DrawParameters::default()
        };

        let possible_moves = Play::get_possible_plays_for_tile(hovered_tile, &game_state);
        let Some(texture) = all_textures.get(&PieceOrBoard::Blue) else {
                    panic!("texture does not exist");
        };
        for play in possible_moves {
            let matrix = [
                [0.125, 0.0, 0.0, 0.0],
                [0.0, 0.125, 0.0, 0.0],
                [0.0, 0.0, 0.125, 0.0],
                [
                    (play.target.letter as f32) * TILE_SIZE - (1.0 - TILE_SIZE * 0.5),
                    (play.target.number as f32) * TILE_SIZE - (1.0 - TILE_SIZE * 0.5),
                    0.0,
                    1.0f32,
                ],
            ];
            let uniforms = uniform! {
                matrix: matrix,
                tex: texture,
            };
            target
                .draw(
                    &vertex_buffer,
                    &indices,
                    &program,
                    &uniforms,
                    &draw_parameters,
                )
                .unwrap();
        }

        for (number, row) in game_state.board.iter().enumerate() {
            for (letter, piece_option) in row.iter().enumerate() {
                let matrix = [
                    [0.125, 0.0, 0.0, 0.0],
                    [0.0, 0.125, 0.0, 0.0],
                    [0.0, 0.0, 0.125, 0.0],
                    [
                        (letter as f32) * TILE_SIZE - (1.0 - TILE_SIZE * 0.5),
                        (number as f32) * TILE_SIZE - (1.0 - TILE_SIZE * 0.5),
                        0.0,
                        1.0f32,
                    ],
                ];
                let Some(piece_with_team) = *piece_option else {
                    continue;
                };
                let Some(texture) = all_textures.get(&PieceOrBoard::Piece(piece_with_team)) else {
                    panic!("texture does not exist");
                };
                let uniforms = uniform! {
                    matrix: matrix,
                    tex: texture,
                };
                target
                    .draw(
                        &vertex_buffer,
                        &indices,
                        &program,
                        &uniforms,
                        &draw_parameters,
                    )
                    .unwrap();
            }
        }

        target.finish().unwrap();
        frames_delta_time = now.elapsed();
    });
}
/*
struct Position {
    x: f64,
    y: f64,
}
impl Position {
    fn as_glium_physical(&self) -> PhysicalPosition<f64> {}
    fn as_board_indices(&self) -> (usize, usize) {}
    fn as_gl(&self) -> (f32, f32) {}
}*/
fn get_hovered_tile(position: &LogicalPosition<f64>) -> TilePosition {
    return TilePosition {
        letter: (position.x * (8.0 / LOGICAL_WINDOW_SIZE.width)).trunc() as usize,
        number: (position.y * (8.0 / LOGICAL_WINDOW_SIZE.height)).trunc() as usize,
    };
}
