use std::ops::Add;

#[derive(Display,Debug,PartialEq,Clone,Copy)]
pub enum Piece {
    None,
    WhitePawn,
    BlackPawn,
    WhiteKnight,
    BlackKnight,
    WhiteBishop,
    BlackBishop,
    WhiteRook,
    BlackRook,
    WhiteQueen,
    BlackQueen,
    WhiteKing,
    BlackKing,
}

pub fn char_to_piece(c: char) -> Piece {
    match c {
        'P' => Piece::WhitePawn,
        'p' => Piece::BlackPawn,
        'B' => Piece::WhiteBishop,
        'b' => Piece::BlackBishop,
        'N' => Piece::WhiteKnight,
        'n' => Piece::BlackKnight,
        'R' => Piece::WhiteRook,
        'r' => Piece::BlackRook,
        'Q' => Piece::WhiteQueen,
        'q' => Piece::BlackQueen,
        'K' => Piece::WhiteKing,
        'k' => Piece::BlackKing,
        _ => Piece::None
    }
}

pub fn piece_to_char(piece: Piece) -> char {
    match piece {
        Piece::WhitePawn => 'P',
        Piece::BlackPawn => 'p',
        Piece::WhiteBishop => 'B',
        Piece::BlackBishop => 'b',
        Piece::WhiteKnight => 'N',
        Piece::BlackKnight => 'n',
        Piece::WhiteRook => 'R',
        Piece::BlackRook => 'r',
        Piece::WhiteQueen => 'Q',
        Piece::BlackQueen => 'q',
        Piece::WhiteKing => 'K',
        Piece::BlackKing => 'k',
        Piece::None => ' ',
    }
}


pub struct ChessBoard {
    /**
     * first index is the 8 - rank, the second is file - 1
     */  
    pieces: [[Piece; 8]; 8]
}

impl ChessBoard {

    pub fn new() -> ChessBoard {
        ChessBoard{pieces: [[Piece::None; 8]; 8]}
    }

    pub fn initial() -> ChessBoard {
        INITIAL_BOARD
    }

    pub fn set_piece(&mut self, file: u8, rank: u8, piece: Piece) {
        let r = (8 - rank) as usize;
        let f = (file - 1) as usize;
        self.pieces[r][f] = piece;
    }

    pub fn get_piece(&self, file: u8, rank: u8) -> Piece {
        let r = (8 - rank) as usize;
        let f = (file - 1) as usize;
        self.pieces[r][f]
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();

        for (_, row) in self.pieces.iter().enumerate() {
            s = s.add("-----------------\n|");
            for (_, piece) in row.iter().enumerate() {
                s.push(piece_to_char(*piece));
                s.push('|');
            }
            s.push('\n');
        }
        s = s.add("-----------------\n");
        s
    }
}

const INITIAL_BOARD : ChessBoard =
    ChessBoard{pieces: [
            [Piece::BlackRook, Piece::BlackKnight, Piece::BlackBishop, Piece::BlackQueen, Piece::BlackKing, Piece::BlackBishop, Piece::BlackKnight, Piece::BlackRook],
            [Piece::BlackPawn, Piece::BlackPawn, Piece::BlackPawn, Piece::BlackPawn, Piece::BlackPawn, Piece::BlackPawn, Piece::BlackPawn, Piece::BlackPawn],
            [Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None],
            [Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None],
            [Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None],
            [Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None],
            [Piece::WhitePawn, Piece::WhitePawn, Piece::WhitePawn, Piece::WhitePawn, Piece::WhitePawn, Piece::WhitePawn, Piece::WhitePawn, Piece::WhitePawn],
            [Piece::WhiteRook, Piece::WhiteKnight, Piece::WhiteBishop, Piece::WhiteQueen, Piece::WhiteKing, Piece::WhiteBishop, Piece::WhiteKnight, Piece::WhiteRook],
        ]};

const INITIAL_POSITION: ChessPosition = 
    ChessPosition{active_color: ChessColor::White, half_move_clock: 0, full_move_number: 1, 
    white_king_side_castling: true, black_king_side_castling: true, white_queen_side_castling: true, black_queen_side_castling: true,
    board: INITIAL_BOARD};

pub struct ChessPosition {
    pub active_color: ChessColor,
    pub half_move_clock: u16,
    pub full_move_number: u16,
    pub white_king_side_castling: bool,
    pub black_king_side_castling: bool,
    pub white_queen_side_castling: bool,
    pub black_queen_side_castling: bool,
    pub board: ChessBoard,
}

impl ChessPosition {
    pub fn initial_position() -> ChessPosition {
        INITIAL_POSITION
    }
}

#[derive(Display,Debug,PartialEq)]
pub enum ChessColor {
    White,
    Black
}