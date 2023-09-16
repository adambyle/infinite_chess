#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

pub use Shape::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn other(self) -> Color {
        match self {
            White => Black,
            Black => White,
        }
    }
}

pub use Color::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Location(i128, i128);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    color: Color,
    shape: Shape,
    location: Location,
}

#[derive(Clone, Copy)]
struct CastleDataEntry {
    king_moved: bool,
    kingdside_rook_moved: bool,
    queenside_rook_moved: bool,
}

impl CastleDataEntry {
    fn new() -> CastleDataEntry {
        CastleDataEntry {
            king_moved: false,
            kingdside_rook_moved: false,
            queenside_rook_moved: false,
        }
    }
}

#[derive(Clone, Copy)]
struct CastleData {
    white: CastleDataEntry,
    black: CastleDataEntry,
}

impl CastleData {
    fn new() -> CastleData {
        CastleData {
            white: CastleDataEntry::new(),
            black: CastleDataEntry::new(),
        }
    }

    fn for_color(self, color: Color) -> CastleDataEntry {
        match color {
            White => self.white,
            Black => self.black,
        }
    }
}

pub struct Board {
    pieces: Vec<Piece>,
    castle_data: CastleData,
}

impl Board {
    pub fn new() -> Board {
        let mut pieces = Vec::with_capacity(32);

        // Set up the pieces for each color.
        //  base_rank = Rank where the pieces go.
        //  pawn_rank = Rank where the pawns go.
        for (color, base_rank, pawn_rank) in [(White, -4, -3), (Black, 3, 2)] {
            for file in -4..3 {
                pieces.push(Piece {
                    color,
                    shape: Pawn,
                    location: Location(file, pawn_rank),
                });
            }
            for (shape, files) in [(Rook, [-4, 3]), (Knight, [-3, 2]), (Bishop, [-2, 1])] {
                for file in files {
                    pieces.push(Piece {
                        color,
                        shape,
                        location: Location(file, base_rank),
                    });
                }
            }
            pieces.push(Piece {
                color,
                shape: Queen,
                location: Location(-1, base_rank),
            });
            pieces.push(Piece {
                color,
                shape: King,
                location: Location(0, base_rank),
            });
        }
        Board { pieces, castle_data: CastleData::new() }
    }

    pub fn new_blank() -> Board {
        Board { pieces: Vec::new(), castle_data: CastleData::new() }
    }

    pub fn piece_at(&self, location: Location) -> Option<BoardPiece> {
        for &piece in &self.pieces {
            if piece.location == location {
                return Some(self.board_piece(piece));
            }
        }
        None
    }

    pub fn pieces(&self) -> impl Iterator<Item = BoardPiece> {
        self.pieces.iter().map(|&piece| self.board_piece(piece))
    }

    pub fn pieces_where<'a, P: FnMut(&Piece) -> bool + 'a>(
        &'a self,
        mut predicate: P,
    ) -> impl Iterator<Item = BoardPiece> {
        self.pieces
            .iter()
            .filter(move |piece| predicate(*piece))
            .map(|piece| self.board_piece(*piece))
    }

    pub fn raw_board(&mut self) -> &mut Vec<Piece> {
        &mut self.pieces
    }

    pub fn find_attackers_of(&self, location: Location, check_legal: bool, color: Option<Color>) -> Box<dyn Iterator<Item = BoardPiece>> {
        match color {
            Some(color) => Box::new(self.pieces().filter(move |&piece| color == piece.color() && piece.attack_sight(location, check_legal).is_legal())),
            None => Box::new(self.pieces().filter(move |&piece| piece.attack_sight(location, check_legal).is_legal())),
        }
    }

    fn board_piece(&self, piece: Piece) -> BoardPiece {
        BoardPiece { piece, board: self }
    }

    fn makes_discovered_attack(&self, location: Location, blocking_at: Location) -> bool {
        todo!()
    }
}

#[derive(Clone, Copy)]
pub struct BoardPiece<'a> {
    piece: Piece,
    board: &'a Board,
}

#[derive(Clone, Copy)]
pub enum Sight<'a> {
    CannotSee,
    SeesEmpty,
    Sees(BoardPiece<'a>),
    IllegalSeesEmpty,
    IllegalSees(BoardPiece<'a>),
}

impl Sight<'_> {
    pub fn is_legal(self) -> bool {
        matches!(self, Sight::Sees(_) | Sight::SeesEmpty)
    }

    pub fn sees(self) -> bool {
        !matches!(self, Sight::CannotSee)
    }
}

impl<'a> Sight<'a> {
    pub fn piece_at(self) -> Option<BoardPiece<'a>> {
        if let Sight::Sees(piece) | Sight::IllegalSees(piece) = self {
            Some(piece)
        } else {
            None
        }
    }
}

impl BoardPiece<'_> {
    pub fn color(self) -> Color {
        self.piece.color
    }

    pub fn shape(self) -> Shape {
        self.piece.shape
    }

    pub fn location(self) -> Location {
        self.piece.location
    }
}

impl<'a> BoardPiece<'a> {
    pub fn parent_board(self) -> &'a Board {
        self.board
    }

    pub fn move_sight(self, destination: Location, check_legal: bool) -> Sight<'a> {
        let illegal;
        let makes_discovered_attack = || check_legal && self.board.makes_discovered_attack(self.location(), destination);
        match self.shape() {
            Pawn => {
                todo!();
                illegal = makes_discovered_attack();
            }
            Rook => {
                todo!();
                illegal = makes_discovered_attack();
            }
            Knight => {
                todo!();
                illegal = makes_discovered_attack();
            }
            Bishop => {
                todo!();
                illegal = makes_discovered_attack();
            }
            Queen => {
                todo!();
                illegal = makes_discovered_attack();
            }
            King => {
                let file_delta = destination.0 - self.location().0;
                if file_delta != 1 && file_delta != -1 { return Sight::CannotSee; }
                let rank_delta = destination.1 - self.location().1;
                if rank_delta != 1 && rank_delta != -1 { return Sight::CannotSee; }

                illegal = check_legal && self.board.find_attackers_of(destination, false, Some(self.color().other())).next().is_some();
            }
        }
        let piece_at_dest = self.board.piece_at(destination);
        match (piece_at_dest, illegal) {
            (Some(piece), false) => Sight::Sees(piece),
            (None, false) => Sight::SeesEmpty,
            (Some(piece), true) => Sight::IllegalSees(piece),
            (None, true) => Sight::IllegalSeesEmpty,
        }
    }

    pub fn attack_sight(self, destination: Location, check_legal: bool) -> Sight {
        todo!()
    }
}