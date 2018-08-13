use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
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

    pub fn write(&mut self, game: &ChessGame) {
        for (tag_key,tag_value) in game.get_tags() {
            write!(&mut self.w, "[{} \"{}\"]\n", tag_key, tag_value);
        }

        write!(&mut self.w, "\n");

        self.w.flush();
    }

}