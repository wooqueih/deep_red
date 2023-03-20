use extend::*;
use text_io::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Team {
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

#[derive(Clone, Copy, Debug)]
enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Horse,
    Pawn,
}

struct PieceOfTeam {
    piece: Piece,
    team: Team
}

#[derive(Clone, Copy)]
struct TilePosition {
    letter: usize,
    number: usize,
}

#[derive(Clone, Copy)]
struct Play {
    origin: TilePosition,
    target: TilePosition,
}

struct PossiblePlaysTreeNode {
    game_state: GameState,
    possibilities: Vec<PossiblePlaysTreeNode>,
}

#[derive(Clone, Copy)]
struct GameState {
    turn: Team,
    board: [[Option<Piece>; 8]; 8],
}

impl TilePosition {
    fn is_valid(&self) -> bool {
        if self.letter < 0 || self.letter > 7 || self.number < 0 || self.number > 7 {
            return false;
        }
        return true;
    }
}

impl Play {
    fn get_all_possible_plays(game_state: &GameState) -> Vec<Play> {
        let mut possible_plays: Vec<Play> = vec![];
        for (number, row) in game_state.board.iter().enumerate() {
            for (letter, _) in row.iter().enumerate() {
                let position = TilePosition { number, letter };
                possible_plays.append(&mut Self::get_possible_plays_for_tile(position, game_state));
            }
        }
        return possible_plays;
    }
    fn get_possible_plays_for_tile(origin: TilePosition, game_state: &GameState) -> Vec<Play> {
        let mut possible_plays: Vec<Play> = vec![];
        let Some(piece) = game_state.board[origin.number][origin.letter] else {
            return possible_plays;
        };
        match piece {
            Piece::Pawn(team) if team == game_state.turn => match team {
                Team::White => {
                    if let None = game_state.board[origin.number + 1][origin.letter] {
                        possible_plays.push(Play {
                            origin,
                            target: TilePosition {
                                letter: origin.letter,
                                number: origin.number + 1,
                            },
                        });
                    }
                    let mut target = TilePosition {
                        letter: origin.letter + 1,
                        number: origin.number + 1,
                    };
                    if target.is_valid() {
                        if let Some(target_piece) = game_state.board[target.number][target.letter] {
                            if let target_piece(team) = game_state.turn {}
                        }
                    }
                    if origin.number == 1 {
                        possible_plays.push(Play {
                            origin,
                            target: TilePosition {
                                letter: origin.letter,
                                number: origin.number + 2,
                            },
                        });
                    }
                }
                Team::Black => {
                    if let None = game_state.board[origin.number - 1][origin.letter] {
                        possible_plays.push(Play {
                            origin,
                            target: TilePosition {
                                letter: origin.letter,
                                number: origin.number - 1,
                            },
                        });
                    }
                    if origin.number == 6 {
                        possible_plays.push(Play {
                            origin,
                            target: TilePosition {
                                letter: origin.letter,
                                number: origin.number - 2,
                            },
                        });
                    }
                }
            },
            Piece::King(team) if team == game_state.turn => {}
        }

        return possible_plays;
    }
}

impl Piece {
    fn get_value(&self) -> u32 {
        match *self {
            Piece::Pawn(_) => return 1,
            Piece::Horse(_) => return 3,
            Piece::Bishop(_) => return 3,
            Piece::Rook(_) => return 5,
            Piece::Queen(_) => return 9,
            Piece::King(_) => return 10_000,
        }
    }
}

impl GameState {
    fn check_for_piece(&self, target: TilePosition) -> Option<(Piece,Team)> {
        match self.board[target.number][target.letter] {
            None => return None,
            Some(piece) => {
                return Some((piece, )
            }
        }
    }
    fn after(&self, play: Play) -> Self {
        let mut next_game_state: GameState = *self;
        next_game_state.turn = !next_game_state.turn;
        next_game_state.board[play.target.number][play.target.letter] =
            self.board[play.origin.number][play.origin.letter];
        next_game_state.board[play.origin.number][play.origin.letter] = None;

        return next_game_state;
    }
    fn new() -> Self {
        return Self {
            turn: Team::White,
            board: [
                [
                    Some(Piece::Rook(Team::White)),
                    Some(Piece::Horse(Team::White)),
                    Some(Piece::Bishop(Team::White)),
                    Some(Piece::Queen(Team::White)),
                    Some(Piece::King(Team::White)),
                    Some(Piece::Bishop(Team::White)),
                    Some(Piece::Horse(Team::White)),
                    Some(Piece::Rook(Team::White)),
                ],
                [
                    Some(Piece::Pawn(Team::White)),
                    Some(Piece::Pawn(Team::White)),
                    Some(Piece::Pawn(Team::White)),
                    Some(Piece::Pawn(Team::White)),
                    Some(Piece::Pawn(Team::White)),
                    Some(Piece::Pawn(Team::White)),
                    Some(Piece::Pawn(Team::White)),
                    Some(Piece::Pawn(Team::White)),
                ],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [
                    Some(Piece::Pawn(Team::Black)),
                    Some(Piece::Pawn(Team::Black)),
                    Some(Piece::Pawn(Team::Black)),
                    Some(Piece::Pawn(Team::Black)),
                    Some(Piece::Pawn(Team::Black)),
                    Some(Piece::Pawn(Team::Black)),
                    Some(Piece::Pawn(Team::Black)),
                    Some(Piece::Pawn(Team::Black)),
                ],
                [
                    Some(Piece::Rook(Team::Black)),
                    Some(Piece::Horse(Team::Black)),
                    Some(Piece::Bishop(Team::Black)),
                    Some(Piece::Queen(Team::Black)),
                    Some(Piece::King(Team::Black)),
                    Some(Piece::Bishop(Team::Black)),
                    Some(Piece::Horse(Team::Black)),
                    Some(Piece::Rook(Team::Black)),
                ],
            ],
        };
    }
}

fn main() {
    let mut game_state = GameState::new();
}
