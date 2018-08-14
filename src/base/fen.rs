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
        let mut file = 'a';
        let mut offset = 0;
        //Chessboard chessboard = new Chessboard();
        let mut dataType = 0;
        let mut activeColor = ChessColor::White;
        let mut whiteKingSideCastling = false;
        let mut blackKingSideCastling = false;
        let mut whiteQueenSideCastling = false;
        let mut blackQueenSideCastling = false;
        let mut enPassantTargetSquareString = String::new();
//        Square enPassantTargetSquare = null;
        let mut halfMoveClockString = String::new();
        let mut halfMoveClock = 0;
        let mut fullMoveNumberString = String::new();
        let mut fullMoveNumber = 1;

        for c in fen.chars() {
            offset += 1;
            
            if c == ' ' {
                dataType += 1;
                continue;
            // skip CR LF
            } else if c <= '\n' {
                continue;
            }

            if dataType == 0 {
                     //new rank
                if c == '/' {
                    rank -= 1;
                    file = 'a';
                    //empty cells
                }else if c >= '1' && c <= '8' {
                    let mut file_i = file as u8;
                    file_i += c.to_string().parse::<u8>().unwrap();
                    file = file_i as char;
                //finally a piece
                }else{
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
                    let mut file_i = file as u8;
                    file_i += 1;
                    file = file_i as char;
                }

            } else if dataType == 1 {
                if c == 'b' {
                    activeColor = ChessColor::Black;
                }else{
                    activeColor = ChessColor::White;
                }
            } else if dataType == 2 {
                if c == 'K' {
                    whiteKingSideCastling = true;
                } else if c == 'k' {
                    blackKingSideCastling = true;
                } else if c == 'Q' {
                    whiteQueenSideCastling = true;
                } else if c == 'q' {
                        blackQueenSideCastling = true;
                } else if c == '-' {
                } else {
                    // TODO 
                    /*
                        throw new IllegalFormatException(
                            "Unknown castling information at offset " + 
                            offset + " (" + new String(cbuf) + ")");
                    */
                }

            } else if dataType == 3 {
                if c != '-' {
                    enPassantTargetSquareString.push(c);
                }
            } else if dataType == 4 {
                if c != '-' {
                    halfMoveClockString.push(c);
                }
            } else if dataType == 5 {
                if c != '-' {
                    fullMoveNumberString.push(c);
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
        
        if halfMoveClockString.len() > 0 {
            match halfMoveClockString.parse::<u16>() {
                Ok(num) => halfMoveClock = num,
                Err(e) => 
                    return Result::Err(format!("Unknown half move clock ({})", 
                        halfMoveClockString))
            }
        }

        if fullMoveNumberString.len() > 0 {
            match fullMoveNumberString.parse::<u16>() {
                Ok(num) => fullMoveNumber = num,
                Err(e) => 
                    return Result::Err(format!("Unknown full move number ({})", 
                        halfMoveClockString))
            }
        }

        Result::Ok(ChessPosition{active_color: activeColor, half_move_clock: halfMoveClock,
            full_move_number: fullMoveNumber})
    }

}

pub struct ChessPosition {
    pub active_color: ChessColor,
    pub half_move_clock: u16,
    pub full_move_number: u16,
}

#[derive(Display,Debug,PartialEq)]
pub enum ChessColor {
    White,
    Black
}
