use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::io::Error;
use std::io::ErrorKind;
use base::parser::ChessGame;
use base::fen::*;

#[derive(Clone)]
pub struct ChessWriterConfig {
    notags: bool,
}

pub struct ChessWriterBuilder{
    config: ChessWriterConfig,
}

impl ChessWriterBuilder {

    pub fn new() -> ChessWriterBuilder {
        ChessWriterBuilder{config: ChessWriterConfig{notags: false}}
    }
    
    pub fn build(&self, file: File) -> ChessWriter {
        ChessWriter::new(self.config.clone(), file)
    }

    pub fn notags(&mut self) {
        self.config.notags = true;
    }

}

pub struct ChessWriter{
    config: ChessWriterConfig,
    w: BufWriter<File>,
}

impl ChessWriter {

    pub fn new(config: ChessWriterConfig, file: File) -> ChessWriter {
        ChessWriter{config: config, w: BufWriter::new(file)}
    }

    pub fn write(&mut self, game: &ChessGame) -> Result<(), Error> {
        if !self.config.notags {
            let tags = game.get_tags();

            if !tags.is_empty() {
                for (tag_key,tag_value) in tags {
                    write!(&mut self.w, "[{} \"{}\"]\n", tag_key, tag_value)?;
                }

                write!(&mut self.w, "\n")?;
            }
        }

        match game.get_before_moves_comment() {
            Some(comment) => write!(&mut self.w, "{}\n", comment)?,
            None => ()
        };

        // TODO error
        self.export_moves(game)?;

        write!(&mut self.w, "{}\n\n", game.get_game_result())?;

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
             write!(&mut self.w, "{}... ", full_move_number)?;
        }

        let mut m = -1;
        for mv in game.get_moves() {
            m += 1;
            if (m % 6) == 0 && m > 0 {
                write!(&mut self.w, "\n")?;
            }
            if active_color == ChessColor::White {
                write!(&mut self.w, "{}. ", full_move_number)?;
            }


/*            Move move = game.getMoves().get(m);
            SANMove sanMove = new SANMove(game.getGameType(), position, move, "=", false);
            */
            write!(&mut self.w, "{} ", mv)?;

            match game.get_nags(m) {
                Some(ns) => for n in ns {
                    // TODO error
                    write!(&mut self.w, "${} ", n)?;
                },
                _ => ()
            }

            match game.get_comment(m) {
                // TODO error
                Some(s) => write!(&mut self.w, "{{{}}} ", s)?,
                _ => ()
            };

            /*
            List<MoveAnnotation> annotations = game.getAnnotations(m);
            
            if (annotations != null) {
                for (MoveAnnotation annotation : annotations) {
                    writer.write(annotation.getValue() + " ");
                }
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

            let mut i = 0;
            match game.get_variations(m) {   
                Some(vs) => 
                    for v in vs {
                        write!(&mut self.w, "({}) ", v)?;
                        match game.get_after_variation_comment(m, i) {
                            // TODO error
                            Some(c) => write!(&mut self.w, "{{{}}}", c)?,
                            _ => ()
                        };
                    i += 1;
                },
                _ => ()
            }
            
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