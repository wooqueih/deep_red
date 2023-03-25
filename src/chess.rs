#![allow(unused, unused_comparisons, dead_code)]

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Team {
    Black,
    White,
}

impl std::ops::Not for Team {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Team::White => Team::Black,
            Team::Black => Team::White,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Piece {
    King(RochadeAbility),
    Queen,
    Rook(RochadeAbility),
    Bishop,
    Horse,
    Pawn(EnPassanteVulnerability),
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnPassanteVulnerability {
    Vulnerable,
    Invulnerable,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum RochadeAbility {
    Able,
    Unable,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct PieceWithTeam {
    pub piece: Piece,
    pub team: Team,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TilePosition {
    pub letter: usize,
    pub number: usize,
}
#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Play {
    pub origin: TilePosition,
    pub target: TilePosition,
}

#[derive(Clone, Copy)]
pub struct GameState {
    pub turn: Team,
    pub board: [[Option<PieceWithTeam>; 8]; 8],
}

impl TilePosition {
    pub fn is_valid(&self) -> bool {
        if self.letter < 0 || self.letter > 7 || self.number < 0 || self.number > 7 {
            return false;
        }
        return true;
    }
}

impl Play {
    pub fn get_all_possible_plays(game_state: &GameState) -> Vec<Play> {
        let mut possible_plays: Vec<Play> = vec![];
        for (number, row) in game_state.board.iter().enumerate() {
            for (letter, _) in row.iter().enumerate() {
                let position = TilePosition { number, letter };
                possible_plays.append(&mut Self::get_possible_plays_for_tile(position, game_state));
            }
        }
        return possible_plays;
    }
    pub fn get_possible_plays_for_tile(origin: TilePosition, game_state: &GameState) -> Vec<Play> {
        let mut possible_plays: Vec<Play> = vec![];
        if !origin.is_valid() {
            //println!("invalid tile");
            return possible_plays;
        }
        //println!("GOOD");
        let Some(piece_with_team) = game_state.board[origin.number][origin.letter] else {
            return possible_plays;
        };
        if piece_with_team.team == !game_state.turn {
            return possible_plays;
        }
        match piece_with_team.piece {
            Piece::Pawn(_) => {
                let direction_coefficient: isize = match piece_with_team.team {
                    Team::White => 1,
                    Team::Black => -1,
                };
                if game_state.board[(origin.number as isize + direction_coefficient) as usize]
                    [origin.letter]
                    .is_none()
                {
                    possible_plays.push(Play {
                        origin,
                        target: TilePosition {
                            letter: origin.letter,
                            number: (origin.number as isize + direction_coefficient) as usize,
                        },
                    });
                    if origin.number
                        == ((3.5 - 2.5 * direction_coefficient as f32).round() as usize)
                        && game_state.board
                            [(origin.number as isize + 2 * direction_coefficient) as usize]
                            [origin.letter]
                            .is_none()
                    {
                        possible_plays.push(Play {
                            origin,
                            target: TilePosition {
                                letter: origin.letter,
                                number: (origin.number as isize + 2 * direction_coefficient)
                                    as usize,
                            },
                        });
                    }
                }
                let target = TilePosition {
                    letter: match origin.letter.checked_sub(1) {
                        Some(num) => num,
                        None => 255,
                    },
                    number: (origin.number as isize + direction_coefficient) as usize,
                };
                if target.is_valid() {
                    if let Some(target_piece_with_team) =
                        game_state.board[target.number][target.letter]
                    {
                        if target_piece_with_team.team == !piece_with_team.team {
                            possible_plays.push(Play { origin, target });
                        }
                    }
                }
                let target = TilePosition {
                    letter: origin.letter + 1,
                    number: (origin.number as isize + direction_coefficient) as usize,
                };
                if target.is_valid() {
                    if let Some(target_piece_with_team) =
                        game_state.board[target.number][target.letter]
                    {
                        if target_piece_with_team.team == !piece_with_team.team {
                            possible_plays.push(Play { origin, target });
                        }
                    }
                }
            }
            Piece::King(_) => {
                let possible_targets = vec![
                    TilePosition {
                        letter: origin.letter + 1,
                        number: origin.number + 1,
                    },
                    TilePosition {
                        letter: origin.letter + 1,
                        number: origin.number,
                    },
                    TilePosition {
                        letter: origin.letter + 1,
                        number: match origin.number.checked_sub(1) {
                            Some(num) => num,
                            None => 255,
                        },
                    },
                    TilePosition {
                        letter: origin.letter,
                        number: match origin.number.checked_sub(1) {
                            Some(num) => num,
                            None => 255,
                        },
                    },
                    TilePosition {
                        letter: match origin.letter.checked_sub(1) {
                            Some(num) => num,
                            None => 255,
                        },
                        number: match origin.number.checked_sub(1) {
                            Some(num) => num,
                            None => 255,
                        },
                    },
                    TilePosition {
                        letter: match origin.letter.checked_sub(1) {
                            Some(num) => num,
                            None => 255,
                        },
                        number: origin.number,
                    },
                    TilePosition {
                        letter: match origin.letter.checked_sub(1) {
                            Some(num) => num,
                            None => 255,
                        },
                        number: origin.number + 1,
                    },
                    TilePosition {
                        letter: origin.letter,
                        number: origin.number + 1,
                    },
                ];

                for target in possible_targets {
                    if target.is_valid() {
                        if let Some(target_piece_with_team) =
                            game_state.board[target.number][target.letter]
                        {
                            if target_piece_with_team.team == piece_with_team.team {
                                continue;
                            }
                        }
                        possible_plays.push(Play { origin, target });
                    }
                }
            }
            Piece::Horse => {
                let possible_targets = vec![
                    TilePosition {
                        letter: origin.letter + 2,
                        number: origin.number + 1,
                    },
                    TilePosition {
                        letter: origin.letter + 2,
                        number: match origin.number.checked_sub(1) {
                            Some(num) => num,
                            None => 255,
                        },
                    },
                    TilePosition {
                        letter: origin.letter + 1,
                        number: origin.number + 2,
                    },
                    TilePosition {
                        letter: match origin.letter.checked_sub(1) {
                            Some(num) => num,
                            None => 255,
                        },
                        number: origin.number + 2,
                    },
                    TilePosition {
                        letter: match origin.letter.checked_sub(1) {
                            Some(num) => num,
                            None => 255,
                        },
                        number: match origin.number.checked_sub(2) {
                            Some(num) => num,
                            None => 255,
                        },
                    },
                    TilePosition {
                        letter: origin.letter + 1,
                        number: match origin.number.checked_sub(2) {
                            Some(num) => num,
                            None => 255,
                        },
                    },
                    TilePosition {
                        letter: match origin.letter.checked_sub(2) {
                            Some(num) => num,
                            None => 255,
                        },
                        number: match origin.number.checked_sub(1) {
                            Some(num) => num,
                            None => 255,
                        },
                    },
                    TilePosition {
                        letter: match origin.letter.checked_sub(2) {
                            Some(num) => num,
                            None => 255,
                        },
                        number: origin.number + 1,
                    },
                ];

                for target in possible_targets {
                    if target.is_valid() {
                        if let Some(target_piece_with_team) =
                            game_state.board[target.number][target.letter]
                        {
                            if target_piece_with_team.team == piece_with_team.team {
                                continue;
                            }
                        }
                        possible_plays.push(Play { origin, target });
                    }
                }
            }
            Piece::Rook(_) => {
                let possible_targets = game_state
                    .unblocked_tiles_in_direction(origin, Direction::Up)
                    .unwrap();
                for target in possible_targets {
                    if target.is_valid() {
                        if let Some(target_piece_with_team) =
                            game_state.board[target.number][target.letter]
                        {
                            if target_piece_with_team.team == piece_with_team.team {
                                continue;
                            }
                        }
                        possible_plays.push(Play { origin, target });
                    }
                }

                let possible_targets = game_state
                    .unblocked_tiles_in_direction(origin, Direction::Down)
                    .unwrap();
                for target in possible_targets {
                    if target.is_valid() {
                        if let Some(target_piece_with_team) =
                            game_state.board[target.number][target.letter]
                        {
                            if target_piece_with_team.team == piece_with_team.team {
                                continue;
                            }
                        }
                        possible_plays.push(Play { origin, target });
                    }
                }

                let possible_targets = game_state
                    .unblocked_tiles_in_direction(origin, Direction::Right)
                    .unwrap();
                for target in possible_targets {
                    if target.is_valid() {
                        if let Some(target_piece_with_team) =
                            game_state.board[target.number][target.letter]
                        {
                            if target_piece_with_team.team == piece_with_team.team {
                                continue;
                            }
                        }
                        possible_plays.push(Play { origin, target });
                    }
                }

                let possible_targets = game_state
                    .unblocked_tiles_in_direction(origin, Direction::Left)
                    .unwrap();
                for target in possible_targets {
                    if target.is_valid() {
                        if let Some(target_piece_with_team) =
                            game_state.board[target.number][target.letter]
                        {
                            if target_piece_with_team.team == piece_with_team.team {
                                continue;
                            }
                        }
                        possible_plays.push(Play { origin, target });
                    }
                }
            }
            Piece::Bishop => {
                let possible_targets = game_state
                    .unblocked_tiles_in_direction(origin, Direction::UpRight)
                    .unwrap();
                for target in possible_targets {
                    if target.is_valid() {
                        if let Some(target_piece_with_team) =
                            game_state.board[target.number][target.letter]
                        {
                            if target_piece_with_team.team == piece_with_team.team {
                                continue;
                            }
                        }
                        possible_plays.push(Play { origin, target });
                    }
                }

                let possible_targets = game_state
                    .unblocked_tiles_in_direction(origin, Direction::UpLeft)
                    .unwrap();
                for target in possible_targets {
                    if target.is_valid() {
                        if let Some(target_piece_with_team) =
                            game_state.board[target.number][target.letter]
                        {
                            if target_piece_with_team.team == piece_with_team.team {
                                continue;
                            }
                        }
                        possible_plays.push(Play { origin, target });
                    }
                }

                let possible_targets = game_state
                    .unblocked_tiles_in_direction(origin, Direction::DownRight)
                    .unwrap();
                for target in possible_targets {
                    if target.is_valid() {
                        if let Some(target_piece_with_team) =
                            game_state.board[target.number][target.letter]
                        {
                            if target_piece_with_team.team == piece_with_team.team {
                                continue;
                            }
                        }
                        possible_plays.push(Play { origin, target });
                    }
                }

                let possible_targets = game_state
                    .unblocked_tiles_in_direction(origin, Direction::DownLeft)
                    .unwrap();
                for target in possible_targets {
                    if target.is_valid() {
                        if let Some(target_piece_with_team) =
                            game_state.board[target.number][target.letter]
                        {
                            if target_piece_with_team.team == piece_with_team.team {
                                continue;
                            }
                        }
                        possible_plays.push(Play { origin, target });
                    }
                }
            }
            Piece::Queen => {
                let possible_targets = game_state
                    .unblocked_tiles_in_direction(origin, Direction::UpRight)
                    .unwrap();
                for target in possible_targets {
                    if target.is_valid() {
                        if let Some(target_piece_with_team) =
                            game_state.board[target.number][target.letter]
                        {
                            if target_piece_with_team.team == piece_with_team.team {
                                continue;
                            }
                        }
                        possible_plays.push(Play { origin, target });
                    }
                }

                let possible_targets = game_state
                    .unblocked_tiles_in_direction(origin, Direction::UpLeft)
                    .unwrap();
                for target in possible_targets {
                    if target.is_valid() {
                        if let Some(target_piece_with_team) =
                            game_state.board[target.number][target.letter]
                        {
                            if target_piece_with_team.team == piece_with_team.team {
                                continue;
                            }
                        }
                        possible_plays.push(Play { origin, target });
                    }
                }

                let possible_targets = game_state
                    .unblocked_tiles_in_direction(origin, Direction::DownRight)
                    .unwrap();
                for target in possible_targets {
                    if target.is_valid() {
                        if let Some(target_piece_with_team) =
                            game_state.board[target.number][target.letter]
                        {
                            if target_piece_with_team.team == piece_with_team.team {
                                continue;
                            }
                        }
                        possible_plays.push(Play { origin, target });
                    }
                }

                let possible_targets = game_state
                    .unblocked_tiles_in_direction(origin, Direction::DownLeft)
                    .unwrap();
                for target in possible_targets {
                    if target.is_valid() {
                        if let Some(target_piece_with_team) =
                            game_state.board[target.number][target.letter]
                        {
                            if target_piece_with_team.team == piece_with_team.team {
                                continue;
                            }
                        }
                        possible_plays.push(Play { origin, target });
                    }
                }

                let possible_targets = game_state
                    .unblocked_tiles_in_direction(origin, Direction::Up)
                    .unwrap();
                for target in possible_targets {
                    if target.is_valid() {
                        if let Some(target_piece_with_team) =
                            game_state.board[target.number][target.letter]
                        {
                            if target_piece_with_team.team == piece_with_team.team {
                                continue;
                            }
                        }
                        possible_plays.push(Play { origin, target });
                    }
                }

                let possible_targets = game_state
                    .unblocked_tiles_in_direction(origin, Direction::Down)
                    .unwrap();
                for target in possible_targets {
                    if target.is_valid() {
                        if let Some(target_piece_with_team) =
                            game_state.board[target.number][target.letter]
                        {
                            if target_piece_with_team.team == piece_with_team.team {
                                continue;
                            }
                        }
                        possible_plays.push(Play { origin, target });
                    }
                }

                let possible_targets = game_state
                    .unblocked_tiles_in_direction(origin, Direction::Right)
                    .unwrap();
                for target in possible_targets {
                    if target.is_valid() {
                        if let Some(target_piece_with_team) =
                            game_state.board[target.number][target.letter]
                        {
                            if target_piece_with_team.team == piece_with_team.team {
                                continue;
                            }
                        }
                        possible_plays.push(Play { origin, target });
                    }
                }

                let possible_targets = game_state
                    .unblocked_tiles_in_direction(origin, Direction::Left)
                    .unwrap();
                for target in possible_targets {
                    if target.is_valid() {
                        if let Some(target_piece_with_team) =
                            game_state.board[target.number][target.letter]
                        {
                            if target_piece_with_team.team == piece_with_team.team {
                                continue;
                            }
                        }
                        possible_plays.push(Play { origin, target });
                    }
                }
            }
        }
        return possible_plays;
    }
}

impl GameState {
    fn unblocked_tiles_in_direction(
        &self,
        origin: TilePosition,
        direction: Direction,
    ) -> Result<Vec<TilePosition>, ()> {
        if !origin.is_valid() {
            return Err(());
        }
        let step_letter_offset: isize = match direction {
            Direction::Up => 0,
            Direction::Down => 0,
            Direction::Left => -1,
            Direction::Right => 1,
            Direction::UpLeft => -1,
            Direction::UpRight => 1,
            Direction::DownLeft => -1,
            Direction::DownRight => 1,
        };
        let step_number_offset: isize = match direction {
            Direction::Up => 1,
            Direction::Down => -1,
            Direction::Left => 0,
            Direction::Right => 0,
            Direction::UpLeft => 1,
            Direction::UpRight => 1,
            Direction::DownLeft => -1,
            Direction::DownRight => -1,
        };

        let mut unblocked_tiles: Vec<TilePosition> = vec![];
        for i in 1..8 {
            let target = TilePosition {
                letter: (origin.letter as isize + i * step_letter_offset) as usize,
                number: (origin.number as isize + i * step_number_offset) as usize,
            };
            if !target.is_valid() {
                break;
            }
            unblocked_tiles.push(TilePosition {
                letter: target.letter,
                number: target.number,
            });
            let Some(_) = self.board[target.number][target.letter] else {continue;};
            break;
        }
        return Ok(unblocked_tiles);
    }
    pub fn after(&self, play: Play) -> Self {
        let mut next_game_state: GameState = *self;
        next_game_state.turn = !next_game_state.turn;
        next_game_state.board[play.target.number][play.target.letter] =
            self.board[play.origin.number][play.origin.letter];
        next_game_state.board[play.origin.number][play.origin.letter] = None;

        return next_game_state;
    }
    pub fn empty() -> Self {
        return Self {
            turn: Team::White,
            board: [
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
            ],
        };
    }
    pub fn new() -> Self {
        return Self {
            turn: Team::White,
            board: [
                [
                    Some(PieceWithTeam {
                        piece: Piece::Rook(RochadeAbility::Able),
                        team: Team::White,
                    }),
                    Some(PieceWithTeam {
                        piece: Piece::Horse,
                        team: Team::White,
                    }),
                    Some(PieceWithTeam {
                        piece: Piece::Bishop,
                        team: Team::White,
                    }),
                    Some(PieceWithTeam {
                        piece: Piece::Queen,
                        team: Team::White,
                    }),
                    Some(PieceWithTeam {
                        piece: Piece::King(RochadeAbility::Able),
                        team: Team::White,
                    }),
                    Some(PieceWithTeam {
                        piece: Piece::Bishop,
                        team: Team::White,
                    }),
                    Some(PieceWithTeam {
                        piece: Piece::Horse,
                        team: Team::White,
                    }),
                    Some(PieceWithTeam {
                        piece: Piece::Rook(RochadeAbility::Able),
                        team: Team::White,
                    }),
                ],
                [
                    Some(PieceWithTeam {
                        piece: Piece::Pawn(EnPassanteVulnerability::Invulnerable),
                        team: Team::White,
                    }),
                    Some(PieceWithTeam {
                        piece: Piece::Pawn(EnPassanteVulnerability::Invulnerable),
                        team: Team::White,
                    }),
                    Some(PieceWithTeam {
                        piece: Piece::Pawn(EnPassanteVulnerability::Invulnerable),
                        team: Team::White,
                    }),
                    Some(PieceWithTeam {
                        piece: Piece::Pawn(EnPassanteVulnerability::Invulnerable),
                        team: Team::White,
                    }),
                    Some(PieceWithTeam {
                        piece: Piece::Pawn(EnPassanteVulnerability::Invulnerable),
                        team: Team::White,
                    }),
                    Some(PieceWithTeam {
                        piece: Piece::Pawn(EnPassanteVulnerability::Invulnerable),
                        team: Team::White,
                    }),
                    Some(PieceWithTeam {
                        piece: Piece::Pawn(EnPassanteVulnerability::Invulnerable),
                        team: Team::White,
                    }),
                    Some(PieceWithTeam {
                        piece: Piece::Pawn(EnPassanteVulnerability::Invulnerable),
                        team: Team::White,
                    }),
                ],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [
                    Some(PieceWithTeam {
                        piece: Piece::Pawn(EnPassanteVulnerability::Invulnerable),
                        team: Team::Black,
                    }),
                    Some(PieceWithTeam {
                        piece: Piece::Pawn(EnPassanteVulnerability::Invulnerable),
                        team: Team::Black,
                    }),
                    Some(PieceWithTeam {
                        piece: Piece::Pawn(EnPassanteVulnerability::Invulnerable),
                        team: Team::Black,
                    }),
                    Some(PieceWithTeam {
                        piece: Piece::Pawn(EnPassanteVulnerability::Invulnerable),
                        team: Team::Black,
                    }),
                    Some(PieceWithTeam {
                        piece: Piece::Pawn(EnPassanteVulnerability::Invulnerable),
                        team: Team::Black,
                    }),
                    Some(PieceWithTeam {
                        piece: Piece::Pawn(EnPassanteVulnerability::Invulnerable),
                        team: Team::Black,
                    }),
                    Some(PieceWithTeam {
                        piece: Piece::Pawn(EnPassanteVulnerability::Invulnerable),
                        team: Team::Black,
                    }),
                    Some(PieceWithTeam {
                        piece: Piece::Pawn(EnPassanteVulnerability::Invulnerable),
                        team: Team::Black,
                    }),
                ],
                [
                    Some(PieceWithTeam {
                        piece: Piece::Rook(RochadeAbility::Able),
                        team: Team::Black,
                    }),
                    Some(PieceWithTeam {
                        piece: Piece::Horse,
                        team: Team::Black,
                    }),
                    Some(PieceWithTeam {
                        piece: Piece::Bishop,
                        team: Team::Black,
                    }),
                    Some(PieceWithTeam {
                        piece: Piece::Queen,
                        team: Team::Black,
                    }),
                    Some(PieceWithTeam {
                        piece: Piece::King(RochadeAbility::Able),
                        team: Team::Black,
                    }),
                    Some(PieceWithTeam {
                        piece: Piece::Bishop,
                        team: Team::Black,
                    }),
                    Some(PieceWithTeam {
                        piece: Piece::Horse,
                        team: Team::Black,
                    }),
                    Some(PieceWithTeam {
                        piece: Piece::Rook(RochadeAbility::Able),
                        team: Team::Black,
                    }),
                ],
            ],
        };
    }
}
