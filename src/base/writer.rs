use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::io::Error;
use base::parser::ChessGame;

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

        self.w.flush()
    }

/*
    fn exportMoves(&mut self, game: &ChessGame) -> Result<(),Error> {
        String comment;
        ChessPosition position = (ChessPosition) game.getInitialPosition();
        if(position.getActiveColor() == Color.BLACK) {
            writer.write(position.getFullMoveNumber() + ". ...");
        }
        for (int m=0; m<game.getMoves().size(); m++) {
            if((m % 6) == 0 && m > 0) {
                writer.write("\n");
            }
            if(position.getActiveColor() == Color.WHITE) {
                writer.write(position.getFullMoveNumber() + ". ");
            }else{
                writer.write(" ");
            }
            Move move = game.getMoves().get(m);
            SANMove sanMove = new SANMove(game.getGameType(), position, move, "=", false);
            writer.write(sanMove.toString() + " ");
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
            }
            
            position = position.positionAfterMove(move);
        }
    }
    */

}