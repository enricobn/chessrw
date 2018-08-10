use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub struct ChessParserBuilder {

}

impl ChessParserBuilder {

    pub fn new() -> Self {
        return ChessParserBuilder{};
    }

    pub fn build(&self) -> ChessParserImpl {
        return ChessParserImpl::new();
    }

}

pub trait ChessParser {

    fn parse(&self, file: File) -> ChessParserIterator;

}

pub struct ChessParserImpl {
    
}

impl ChessParser for ChessParserImpl {

    fn parse(&self, file: File) -> ChessParserIterator {
        let reader = BufReader::new(file);
        return ChessParserIterator::new(reader);
    }

}

impl ChessParserImpl {

    pub fn new() -> Self {
        return ChessParserImpl{};
    }

}

pub struct ChessParserIterator {
    file_reader: BufReader<File>,
    buf: String,
}

impl ChessParserIterator {

    pub fn new(file_reader: BufReader<File>) -> Self {
        return ChessParserIterator{file_reader: file_reader, buf: String::new()};
    }

}

impl Iterator for ChessParserIterator {
    type Item = ChessGame;

    fn next(&mut self) -> Option<ChessGame> {
        let count = self.file_reader.read_line(&mut self.buf);
        
        if count.unwrap() <= 0 {
            return None;
        } else {
            {
                let line = self.buf.trim_right();
                println!("{}", line);
            }
            self.buf.clear();
            return Some(ChessGame{});
        }
    }
}

pub struct ChessGame {

}