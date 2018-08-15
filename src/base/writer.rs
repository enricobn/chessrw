use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::io::Error;
use std::io::ErrorKind;
use base::parser::ChessGame;
use base::fen::*;

pub struct ChessWriterBuilder{
}

impl ChessWriterBuilder {
    
    pub fn build(&self, file: File) -> ChessWriter {
        ChessWriter::new(file)
    }

}

pub struct ChessWriter{
    w: BufWriter<File>,
}

impl ChessWriter {

    pub fn new(file: File) -> ChessWriter {
        ChessWriter{w: BufWriter::new(file)}
    }

    pub fn write(&mut self, game: &ChessGame) -> Result<(), Error> {
        for (tag_key,tag_value) in game.get_tags() {
            write!(&mut self.w, "[{} \"{}\"]\n", tag_key, tag_value);
        }

        write!(&mut self.w, "\n");

        let ok = match game.get_before_moves_comment() {
            Some(comment) => write!(&mut self.w, "{}\n", comment),
            None => Result::Ok(())
        };

        if ok.is_err() {
            return ok;
        }

        // TODO error
        self.export_moves(game);

        write!(&mut self.w, " {}\n\n", game.get_game_result());

        self.w.flush()
    }

    fn export_moves(&mut self, game: &ChessGame) -> Result<(),Error> {
        //let comment: Option<String> = None;
        let position = match game.initial_position() {
            Ok(p) => p,
            Err(e) => return Result::Err(Error::new(ErrorKind::Other, e))
        };

        let mut active_color = position.active_color;
//        let mut half_move_clock = position.half_move_clock;
        let mut full_move_number = position.full_move_number;

        if active_color == ChessColor::Black {
             write!(&mut self.w, "{}... ", full_move_number);
        }

        let mut m = -1;
        for mv in game.get_moves() {
            m += 1;
            if (m % 6) == 0 && m > 0 {
                write!(&mut self.w, "\n");
            }
            if active_color == ChessColor::White {
                write!(&mut self.w, "{}. ", full_move_number);
            }

/*            Move move = game.getMoves().get(m);
            SANMove sanMove = new SANMove(game.getGameType(), position, move, "=", false);
            */
            write!(&mut self.w, "{} ", mv);

            /*
            List<MoveAnnotation> annotations = game.getAnnotations(m);
            
            if (annotations != null) {
                for (MoveAnnotation annotation : annotations) {
                    writer.write(annotation.getValue() + " ");
                }
            }
            
            comment = game.getComment(m); 
            if (comment != null) {
                writer.write("{" + comment + "} ");
            }
            
            if (game.getVariations(m) != null) {
                int i = 0;
                for (String variation : game.getVariations(m)) {
                    writer.write("(" + variation + ") ");
                    String variationComment = game.getAfterVariationComment(m, i);
                    if (variationComment != null) {
                        writer.write("{" + variationComment + "} ");
                    }
                    i++;
                }
            }*/
            
            if active_color == ChessColor::White {
                active_color = ChessColor::Black;
            } else {
                active_color = ChessColor::White;
            }

            if active_color == ChessColor::Black {
                full_move_number += 1;
            }

            //position = position.positionAfterMove(move);
        }
        Result::Ok(())
    }

}