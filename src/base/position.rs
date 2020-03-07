use std::fmt::Display;
use std::ops::Add;
use std::fmt;

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

pub fn char_to_piece(c: char) -> Result<Piece, String> {
    match c {
        'P' => Ok(Piece::WhitePawn),
        'p' => Ok(Piece::BlackPawn),
        'B' => Ok(Piece::WhiteBishop),
        'b' => Ok(Piece::BlackBishop),
        'N' => Ok(Piece::WhiteKnight),
        'n' => Ok(Piece::BlackKnight),
        'R' => Ok(Piece::WhiteRook),
        'r' => Ok(Piece::BlackRook),
        'Q' => Ok(Piece::WhiteQueen),
        'q' => Ok(Piece::BlackQueen),
        'K' => Ok(Piece::WhiteKing),
        'k' => Ok(Piece::BlackKing),
        ' ' => Ok(Piece::None),
        _ => Err(format!("Unknown piece {}.", c))
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

pub fn piece_color(piece: &Piece) -> Option<ChessColor> {
    match piece {
        Piece::WhitePawn => Some(ChessColor::White),
        Piece::BlackPawn => Some(ChessColor::Black),
        Piece::WhiteBishop => Some(ChessColor::White),
        Piece::BlackBishop => Some(ChessColor::Black),
        Piece::WhiteKnight => Some(ChessColor::White),
        Piece::BlackKnight => Some(ChessColor::Black),
        Piece::WhiteRook => Some(ChessColor::White),
        Piece::BlackRook => Some(ChessColor::Black),
        Piece::WhiteQueen => Some(ChessColor::White),
        Piece::BlackQueen => Some(ChessColor::Black),
        Piece::WhiteKing => Some(ChessColor::White),
        Piece::BlackKing => Some(ChessColor::Black),
        Piece::None => None,
    }
}

fn piece_type_to_piece(piece_type: PieceType, color: ChessColor) -> Piece {
    let mut piece_char = piece_type_to_char(piece_type);

    if color == ChessColor::Black {
        piece_char = piece_char.to_lowercase().next().unwrap();
    }

    char_to_piece(piece_char).unwrap()
}

#[derive(Display,Debug,PartialEq,Clone,Copy)]
pub enum PieceType {
    None,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub fn char_to_piece_type(c: char) -> PieceType {
    match c {
        'P' => PieceType::Pawn,
        'B' => PieceType::Bishop,
        'N' => PieceType::Knight,
        'R' => PieceType::Rook,
        'Q' => PieceType::Queen,
        'K' => PieceType::King,
        _ => PieceType::None
    }
}

pub fn piece_type_to_char(piece_type: PieceType) -> char {
    match piece_type {
        PieceType::Pawn => 'P',
        PieceType::Bishop => 'B',
        PieceType::Knight => 'N',
        PieceType::Rook => 'R',
        PieceType::Queen => 'Q',
        PieceType::King => 'K',
        PieceType::None => ' ',
    }
}

#[derive(PartialEq,Clone,Copy)]
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

    pub fn set_piece(&mut self, file: u8, rank: u8, piece: Piece) -> Option<String> {
        if file < 1 || file > 8 || rank < 1 || rank > 8 {
            return Some(format!("Invalid square ({}, {}).", file, rank));
        }
        let r = (8 - rank) as usize;
        let f = (file - 1) as usize;
        self.pieces[r][f] = piece;
        None
    }

    pub fn get_piece(&self, file: u8, rank: u8) -> Piece {
        let r = (8 - rank) as usize;
        let f = (file - 1) as usize;
        self.pieces[r][f]
    }

    pub fn find_piece(&self, piece: Piece) -> Vec<Square> {
        let mut squares = Vec::new();

        for (rank, row) in self.pieces.iter().enumerate() {
            for (file, p) in row.iter().enumerate() {
                if *p == piece {
                    squares.push(Square::new(file as u8 + 1, 8 - rank as u8 ).unwrap());
                }
            }
        }

        squares
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

    pub fn is_safe(&self, square: &Square, attacking_color: ChessColor) -> bool {

        let pawn = piece_type_to_piece(PieceType::Pawn, attacking_color);

        for s in self.reachable_from_pawn(square, true, attacking_color) {
            if self.get_piece(s.file, s.rank) == pawn {
                // println!("square {} is reachable from pawn in {}", square, s);
                return false;
            }
        }

        let knight = piece_type_to_piece(PieceType::Knight, attacking_color);
        for s in ChessBoard::reachable_from_knight(&square) {

            if self.get_piece(s.file, s.rank) == knight {
                // println!("square {} is reachable from knight in {}", square, s);
                return false;
            }
        }

        let king = piece_type_to_piece(PieceType::King, attacking_color);
        for s in ChessBoard::reachable_from_king(&square) {

            if self.get_piece(s.file, s.rank) == king {
                // println!("square {} is reachable from king in {}", square, s);
                return false;
            }
        }

        let bishop = piece_type_to_piece(PieceType::Bishop, attacking_color);
        for s in self.reachable_from_sliding_piece(&square, true, false) {

            if self.get_piece(s.file, s.rank) == bishop {
                // println!("square {} is reachable from bishop in {}", square, s);
                return false;
            }
        }

        let rook = piece_type_to_piece(PieceType::Rook, attacking_color);
        for s in self.reachable_from_sliding_piece(&square, false, true) {

            if self.get_piece(s.file, s.rank) == rook {
                // println!("square {} is reachable from rook in {}", square, s);
                return false;
            }
        }

        let queen = piece_type_to_piece(PieceType::Queen, attacking_color);
        for s in self.reachable_from_sliding_piece(&square, true, true) {

            if self.get_piece(s.file, s.rank) == queen {
                // println!("square {} is reachable from queen in {}", square, s);
                return false;
            }
        }

        // TODO rook, queen
        true
    }

    fn reachable_from_pawn(&self, square: &Square, capture: bool, pawn_color: ChessColor) -> Vec<Square> {
        let mut squares = Vec::new();

        let rank_dir = if pawn_color == ChessColor::White {
            -1
        } else {
            1
        };

        if capture {
            // TODO en passant
            square.mv(1, rank_dir).iter().for_each(|&it| squares.push(it));
            square.mv(-1, rank_dir).iter().for_each(|&it| squares.push(it));
        } else {
            square.mv(0, rank_dir).iter().for_each(|&it| squares.push(it));
            let can_move_forward = squares.len() == 1 && self.get_piece(square.file, (square.rank as i8 + rank_dir) as u8) == Piece::None;

            if can_move_forward && (pawn_color == ChessColor::White && square.rank == 4 || 
                    pawn_color == ChessColor::Black && square.rank == 5) {
                square.mv(0, 2 * rank_dir).iter().for_each(|&it| squares.push(it));
            }
        }

        squares
    }

    pub fn reachable_from_king(square: &Square) -> Vec<Square> {
        let mut squares = Vec::new();

        ChessBoard::add_to_squares_if_ok(&mut squares, square.mv(-1, -1));
        ChessBoard::add_to_squares_if_ok(&mut squares, square.mv(0, -1));
        ChessBoard::add_to_squares_if_ok(&mut squares, square.mv(1, -1));
        ChessBoard::add_to_squares_if_ok(&mut squares, square.mv(-1, 0));
        ChessBoard::add_to_squares_if_ok(&mut squares, square.mv(1, 0));
        ChessBoard::add_to_squares_if_ok(&mut squares, square.mv(-1, 1));
        ChessBoard::add_to_squares_if_ok(&mut squares, square.mv(0, 1));
        ChessBoard::add_to_squares_if_ok(&mut squares, square.mv(1, 1));

        squares
    }

    pub fn reachable_from_knight(square: &Square) -> Vec<Square> {
        let mut squares = Vec::new();

        ChessBoard::add_to_squares_if_ok(&mut squares, square.mv(-2, -1));
        ChessBoard::add_to_squares_if_ok(&mut squares, square.mv(-1, -2));
        ChessBoard::add_to_squares_if_ok(&mut squares, square.mv(1, -2));
        ChessBoard::add_to_squares_if_ok(&mut squares, square.mv(2, -1));
        ChessBoard::add_to_squares_if_ok(&mut squares, square.mv(-2, 1));
        ChessBoard::add_to_squares_if_ok(&mut squares, square.mv(-1, 2));
        ChessBoard::add_to_squares_if_ok(&mut squares, square.mv(1, 2));
        ChessBoard::add_to_squares_if_ok(&mut squares, square.mv(2, 1));

        squares
    }

    fn add_to_squares_if_ok(squares: &mut Vec<Square>, square_result: Result<Square, String>) {
        if square_result.is_err() {
            return;
        }

        squares.push(square_result.unwrap());
    }

    pub fn reachable_from_sliding_piece(&self, square: &Square, diagonal: bool, straight: bool) -> Vec<Square> {
        let mut squares = Vec::new();

        if diagonal {
            squares.extend(self.reachable_from_direction(square, 1, 1));
            squares.extend(self.reachable_from_direction(square, -1, 1));
            squares.extend(self.reachable_from_direction(square, 1, -1));
            squares.extend(self.reachable_from_direction(square, -1, -1));
        }

        if straight {
            squares.extend(self.reachable_from_direction(square, 1, 0));
            squares.extend(self.reachable_from_direction(square, -1, 0));
            squares.extend(self.reachable_from_direction(square, 0, 1));
            squares.extend(self.reachable_from_direction(square, 0, -1));
        }

        // squares.iter().for_each(|it| println!("{},{}", it.file, it.rank));

        squares
    }

    pub fn reachable_from_direction(&self, square: &Square, file_offset: i8, rank_offset: i8) -> Vec<Square> {
        let mut squares = Vec::new();

        let mut i = 1;

        loop {
            let s = square.mv(file_offset * i, rank_offset * i);
            
            if !s.is_ok() {
                break;
            }

            let s = s.unwrap();
            squares.push(s);

            if self.get_piece(s.file, s.rank) != Piece::None {
                break;
            }

            i += 1;
        }

        squares
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

#[derive(PartialEq,Clone,Copy)]
pub struct Square {
    file: u8,
    rank: u8,
}

impl Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let file = ('a' as u8 + self.file -1) as char;
        write!(f, "{}{}", file, self.rank)
    }
}

impl fmt::Debug for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let file = ('a' as u8 + self.file -1) as char;
        write!(f, "{}{}", file, self.rank)
    }
}

impl Square {

    pub fn new(file: u8, rank: u8) -> Result<Square,String> {

        if file >= 1 && file <= 8 && rank >= 1 && rank <= 8 {
            Ok(Square{file, rank })
        } else {
            Err(format!("Invalid square indices ({},{}).", file, rank))
        }
    }

    pub fn from_string(square: &String) -> Result<Square,String> {
        if square.len() != 2 {
            return Err(format!("Invalid square {}.", square));
        }

        let file_char = square.chars().next().unwrap();
        let rank_char = square.chars().skip(1).next().unwrap();

        let rank = rank_char as u8 - '0' as u8;
        let file = file_char as u8 - 'a' as u8  + 1;

        Square::new(file, rank)
    }

    pub fn north(&self) -> Result<Square,String> {
        self.mv(0, 1)
    }

    pub fn south(&self) -> Result<Square,String> {
        self.mv(0, -1)
    }

    pub fn east(&self) -> Result<Square,String> {
        self.mv(1, 0)
    }

    pub fn west(&self) -> Result<Square,String> {
        self.mv(-1, 0)
    }

    pub fn mv(&self, file: i8, rank: i8) -> Result<Square,String> {
        let file = self.file as i8 + file;
        let rank = self.rank as i8 + rank;

        Square::new(file as u8, rank as u8)
    }

}

const INITIAL_POSITION: ChessPosition = 
    ChessPosition{active_color: ChessColor::White, half_move_clock: 0, full_move_number: 1, 
    white_king_side_castling: true, black_king_side_castling: true, white_queen_side_castling: true, black_queen_side_castling: true,
    board: INITIAL_BOARD, en_passant_target_square: None};

#[derive(PartialEq,Clone,Copy)]
pub struct ChessPosition {
    pub active_color: ChessColor,
    pub half_move_clock: u16,
    pub full_move_number: u16,
    pub white_king_side_castling: bool,
    pub black_king_side_castling: bool,
    pub white_queen_side_castling: bool,
    pub black_queen_side_castling: bool,
    pub board: ChessBoard,
    pub en_passant_target_square: Option<Square>,
}

impl ChessPosition {
    pub fn initial_position() -> ChessPosition {
        INITIAL_POSITION
    }

    pub fn apply_move(&mut self, san_move: &str) -> Option<String> {
        let capture = san_move.contains("x");

        let check = san_move.contains("+");

        let check_mate = san_move.contains("#");

        let mut mv = String::new();
        mv.push_str(san_move);

        mv = mv.replace('x', "");
        mv = mv.replace('+', "");
        mv = mv.replace('?', "");
        mv = mv.replace('!', "");
        mv = mv.replace('#', "");
        mv = mv.replace('=', "");

        if &mv == "O-O" || &mv == "0-0" {
            self.king_side_castling()
        } else if &mv == "O-O-O" || &mv == "0-0-0" {
            self.queen_side_castling()
        } else {
            let first = mv.chars().next().unwrap();

            let mut promotion = None;

            let piece_type = if first >= 'A' && first <= 'Z' {
                mv = mv.chars().skip(1).collect();
                char_to_piece_type(first)
            } else {
                PieceType::Pawn
            };
            
            let last : char = mv.chars().skip(mv.len() -1).take(1).next().unwrap();

            if !last.is_digit(10) {
                promotion = Some(char_to_piece_type(last));
                mv = mv.chars().take(mv.len() -1).collect();
            }

            // disambiguation: the from square has been specified 
            if mv.len() == 4 {
                let from_result = Square::from_string(&mv.chars().take(2).collect());

                if from_result.is_err() {
                    return Some(from_result.unwrap_err());
                }

                let from = from_result.unwrap();

                let to_result = Square::from_string(&mv.chars().skip(2).take(2).collect());

                if to_result.is_err() {
                    return Some(to_result.unwrap_err());
                }

                let to = to_result.unwrap();

                self.move_piece(from.file, from.rank, to.file, to.rank);

                // TODO promotion
            } else {
                let mut from_file : Option<u8> = None;
                let mut from_rank : Option<u8> = None;

                if mv.len() == 3 {
                    let first = mv.chars().next().unwrap();

                    if first.is_digit(10) {
                        if first < '1' || first > '8' {
                            return Some(format!("move {}, invalid rank {}", san_move, first))
                        }
                        from_rank = Some(first as u8 - '0' as u8);
                    } else {
                        if first < 'a' || first > 'h' {
                            return Some(format!("move {}, invalid file {}", san_move, first))
                        }
                        from_file = Some(first as u8 - 'a' as u8 + 1);
                    }

                    mv = mv.chars().skip(1).collect();
                }

                let to_result = Square::from_string(&mv);

                if to_result.is_err() {
                    return Some(to_result.unwrap_err());
                }

                let to = to_result.unwrap();

                let to_piece = self.board.get_piece(to.file, to.rank);

                // println!("To {},{}", to.file, to.rank);

                let piece = piece_type_to_piece(piece_type, self.active_color);

                // TODO I think I don't need it
                let mut from_squares = self.board.find_piece(piece);

                let from = if from_squares.len() == 1 {
                    from_squares.pop().unwrap()
                } else {
                    from_squares = from_squares.iter().filter(|it| {
                        if from_file.is_some() {
                            it.file == from_file.unwrap()
                        } else {
                            true
                        }
                    }).filter(|it| {
                        if from_rank.is_some() {
                            it.rank == from_rank.unwrap()
                        } else {
                            true
                        }
                    }).cloned().collect();

                    if from_squares.len() == 1 {
                        // println!("found one", );
                        from_squares.pop().unwrap()
                    } else {
                        // println!("found more {}s", piece_type);

                        // from_squares.iter().for_each(|it| println!("found in {},{}", it.file, it.rank));

                        let reachable = match piece_type {
                            PieceType::Pawn => self.board.reachable_from_pawn(&to, capture, self.active_color),
                            PieceType::King => ChessBoard::reachable_from_king(&to),
                            PieceType::Knight => ChessBoard::reachable_from_knight(&to),
                            PieceType::Bishop => self.board.reachable_from_sliding_piece(&to, true, false),
                            PieceType::Rook => self.board.reachable_from_sliding_piece(&to, false, true),
                            PieceType::Queen => self.board.reachable_from_sliding_piece(&to, true, true),
                            _ => vec![]
                        };

                        from_squares.retain(|it| reachable.contains(it) && self.valid_move(&it, &to, &piece, &to_piece, capture));

                        if from_squares.len() == 1 {
                            from_squares.pop().unwrap()
                        } else {

                            let cloned = self.clone();

                            let mut from_square : Option<Square> = None;

                            for from in &from_squares {
                                let do_move_result = self.do_move(&from, &to, piece_type, capture, promotion);

                                if do_move_result.is_some() {
                                    return Some(format!("move {}, error applying move: {}", san_move, do_move_result.unwrap()));
                                }

                                if !self.king_in_check(self.active_color) {
                                    if from_square.is_some() {
                                        return Some(format!("move {}, cannot disambiguate move during check test", san_move));
                                    }
                                    from_square = Some(*from)
                                }

                                self.update_with(cloned);
                            }

                            if from_square.is_some() {
                                from_square.unwrap()
                            } else {
                                return Some(format!("move {}, cannot disambiguate move, multiple from squares: {:?}", san_move, &from_squares));
                            }
                        }
                    }
                };

                self.do_move(&from, &to, piece_type, capture, promotion);

            }
        }

        self.next_move();
                
        None

    }

    fn queen_side_castling(&mut self) -> () {
        if self.active_color == ChessColor::White {
            self.move_piece(5, 1, 3, 1);
            self.move_piece(1, 1, 4, 1);
            self.white_queen_side_castling = false;
        } else {
            self.move_piece(5, 8, 3, 8);
            self.move_piece(1, 8, 4, 8);
            self.black_queen_side_castling = false;
        }
    }

    fn king_side_castling(&mut self) -> () {
        if self.active_color == ChessColor::White {
            self.move_piece(5, 1, 7, 1);
            self.move_piece(8, 1, 6, 1);
            self.white_king_side_castling = false;
        } else {
            self.move_piece(5, 8, 7, 8);
            self.move_piece(8, 8, 6, 8);
            self.black_king_side_castling = false;
        }
    }

    fn update_with(&mut self, cloned: ChessPosition) {
        self.board = cloned.board;
        self.en_passant_target_square = cloned.en_passant_target_square;
        self.full_move_number = cloned.full_move_number;
        self.half_move_clock = cloned.half_move_clock;
        self.active_color = cloned.active_color;
        self.black_king_side_castling = cloned.black_king_side_castling;
        self.white_king_side_castling = cloned.white_king_side_castling;
        self.black_queen_side_castling = cloned.black_queen_side_castling;
        self.white_queen_side_castling = cloned.white_queen_side_castling;
    }

    fn do_move(&mut self, from: &Square, to: &Square, piece_type: PieceType, capture: bool, promotion: Option<PieceType>) -> Option<String> {
        self.move_piece(from.file, from.rank, to.file, to.rank); // move to board

        if capture && piece_type == PieceType::Pawn && 
                self.en_passant_target_square.is_some() && 
                self.en_passant_target_square.unwrap() == *to {
                    
            if self.active_color == ChessColor::White {
                self.board.set_piece(to.file, to.rank - 1, Piece::None);
            } else {
                self.board.set_piece(to.file, to.rank + 1, Piece::None);
            }
        }

        if promotion.is_some() {
            let promotion_piece = piece_type_to_piece(promotion.unwrap(), self.active_color);
            self.board.set_piece(to.file, to.rank, promotion_piece);
        }

        if piece_type == PieceType::Pawn && (from.rank as i8 - to.rank as i8).abs() == 2 {
            let result = if self.active_color == ChessColor::White {
                from.north()
            } else {
                from.south()
            };
            
            if result.is_err() {
                return Some(result.unwrap_err());
            }
            self.en_passant_target_square = Some(result.unwrap());
        } else {
            self.en_passant_target_square = None;
        }                
        None
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();

        if self.active_color == ChessColor::White {
            s = s.add("White to move\n");
        } else {
            s = s.add("Black to move\n");
        }

        s = s.add(&self.board.to_string());

        s
    }

    pub fn king_in_check(&self, king_color: ChessColor) -> bool {
        let king = piece_type_to_piece(PieceType::King, king_color);
        let kings = self.board.find_piece(king);
        let king_square = kings.iter().next().unwrap();
        !self.board.is_safe(&king_square, other_color(king_color))
    }

    fn valid_move(&self, from: &Square, to: &Square, piece: &Piece, to_piece: &Piece, capture: bool) -> bool {
        if self.board.get_piece(from.file, from.rank) != *piece {
            false
        } else if capture {
            let en_passant = (*piece == Piece::WhitePawn || *piece == Piece::BlackPawn) && self.en_passant_target_square.is_some() &&
                self.en_passant_target_square.unwrap() == *to;
            en_passant || *to_piece != Piece::None && piece_color(to_piece) == Some(other_color(self.active_color))
        } else {
            *to_piece == Piece::None
        }
    }

    fn next_move(&mut self) {
        if self.active_color == ChessColor::White {
            self.active_color = ChessColor::Black;
            self.full_move_number += 1; // TODO is it correct?
        } else {
            self.active_color = ChessColor::White;
        }

        self.half_move_clock += 1;
    }

    fn move_piece(&mut self, from_file: u8, from_rank: u8, to_file: u8, to_rank: u8) {
        let piece = self.board.get_piece(from_file, from_rank);
        self.board.set_piece(to_file, to_rank, piece);
        self.board.set_piece(from_file, from_rank, Piece::None);
    }
}

#[derive(Display,Debug,PartialEq,Clone,Copy)]
pub enum ChessColor {
    White,
    Black
}

fn other_color(color: ChessColor) -> ChessColor {
    if color == ChessColor::White {
        ChessColor::Black
    } else {
        ChessColor::White
    }
}

#[cfg(test)]

#[test]
fn apply_move() {
    let mut position = ChessPosition::initial_position();

    position.apply_move("e4");
    assert_eq!(Piece::WhitePawn, position.board.get_piece(5, 4));

    position.apply_move("e6");
    assert_eq!(Piece::BlackPawn, position.board.get_piece(5, 6));

    position.apply_move("Ke2");
    assert_eq!(Piece::WhiteKing, position.board.get_piece(5, 2));

    position.apply_move("Nf6");
    assert_eq!(Piece::BlackKnight, position.board.get_piece(6, 6));

    position.apply_move("Ke3");
    position.apply_move("Bb4");

    assert_eq!(Piece::BlackBishop, position.board.get_piece(2, 4));

    position.apply_move("Qg4");
    assert_eq!(Piece::WhiteQueen, position.board.get_piece(7, 4));

    let mut position_before_moving_rook = position.clone();

    position.apply_move("Rf8");
    assert_eq!(Piece::BlackRook, position.board.get_piece(6, 8));

    position_before_moving_rook.apply_move("O-O");
    assert_eq!(position_before_moving_rook.active_color, ChessColor::White);
    assert_eq!(Piece::BlackRook, position_before_moving_rook.board.get_piece(6, 8));
    assert_eq!(Piece::BlackKing, position_before_moving_rook.board.get_piece(7, 8));

    // println!("{}", position.board.to_string());

}