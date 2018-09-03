use base::position::*;

pub struct FENParserBuilder {
}

impl FENParserBuilder {

    pub fn new() -> Self {
        return FENParserBuilder{};
    }

    pub fn build(&self) -> FENParser {
        return FENParser{};
    }

}

pub struct FENParser {

}

impl FENParser {

    pub fn parse(&self, fen: &str) -> Result<ChessPosition,String> {
        let mut rank = 8;
        let mut file : u8 = 1; //'a'
        let mut offset = 0;
        //Chessboard chessboard = new Chessboard();
        let mut status = 0;
        let mut active_color = ChessColor::White;
        let mut white_king_side_castling = false;
        let mut black_king_side_castling = false;
        let mut white_queen_side_castling = false;
        let mut black_queen_side_castling = false;
        let mut en_passant_target_square_string = String::new();
//        Square enPassantTargetSquare = null;
        let mut half_move_clock_string = String::new();
        let mut half_move_clock = 0;
        let mut full_move_number_string = String::new();
        let mut full_move_number = 1;
        let mut chessboard = ChessBoard::new();

        for c in fen.chars() {
            offset += 1;
            
            if c == ' ' {
                status += 1;
                continue;
            // skip CR LF
            } else if c <= '\n' {
                continue;
            }

            if status == 0 {
                //new rank
                if c == '/' {
                    rank -= 1;
                    file = 1;
                //empty cells
                } else if c >= '1' && c <= '8' {
                    file += c.to_string().parse::<u8>().unwrap();
                //finally a piece
                } else {
                    /*
                    try{
                        Piece piece = pieceFromString(new String(cbuf));
                        chessboard.setPiece(Square.newSquare(file, rank), piece);
                        file++;
                    }catch(IllegalArgumentException e) {
                        throw new IllegalFormatException(
                            "Unknown piece data at offset " + offset + 
                            " (" + new String(cbuf) + ")", e);
                    }
                    */
                    chessboard.set_piece(file, rank, char_to_piece(c));
                    file += 1;
                }

            } else if status == 1 {
                if c == 'b' {
                    active_color = ChessColor::Black;
                }else{
                    active_color = ChessColor::White;
                }
            } else if status == 2 {
                if c == 'K' {
                    white_king_side_castling = true;
                } else if c == 'k' {
                    black_king_side_castling = true;
                } else if c == 'Q' {
                    white_queen_side_castling = true;
                } else if c == 'q' {
                    black_queen_side_castling = true;
                } else if c == '-' {
                } else {
                    return Result::Err(format!("Unknown castling information at offset {} ({}).", 
                        offset, c));
                }

            } else if status == 3 {
                if c != '-' {
                    en_passant_target_square_string.push(c);
                }
            } else if status == 4 {
                if c != '-' {
                    half_move_clock_string.push(c);
                }
            } else if status == 5 {
                if c != '-' {
                    full_move_number_string.push(c);
                }
            }
        }

        /*if(enPassantTargetSquareString.len() > 0 && 
                enPassantTargetSquare == null) {
            try{
                enPassantTargetSquare = Square.newSquare(
                    enPassantTargetSquareString);
            }catch(Exception e) {
                throw new IllegalFormatException(
                    "Unknown en passant target square (" + 
                    enPassantTargetSquareString + ")");                                
            }
        }
        */
        
        if half_move_clock_string.len() > 0 {
            match half_move_clock_string.parse::<u16>() {
                Ok(num) => half_move_clock = num,
                Err(_) => 
                    return Result::Err(format!("Unknown half move clock ({}).", 
                        half_move_clock_string))
            }
        }

        if full_move_number_string.len() > 0 {
            match full_move_number_string.parse::<u16>() {
                Ok(num) => full_move_number = num,
                Err(_) => 
                    return Result::Err(format!("Unknown full move number ({}).", 
                        half_move_clock_string))
            }
        }

        Result::Ok(ChessPosition{active_color: active_color, half_move_clock: half_move_clock,
            full_move_number: full_move_number, 
            white_king_side_castling: white_king_side_castling, 
            black_king_side_castling: black_king_side_castling,
            white_queen_side_castling: white_queen_side_castling,
            black_queen_side_castling: black_queen_side_castling,
            board: chessboard})
    }

}