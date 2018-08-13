//extern crate chessparser;
#[cfg(test)]

use base::parser::*;
use base::writer::*;
use std::fs::File;
use std::collections::HashMap;

#[test]
fn parse_kramnik() {
    let builder = ChessParserBuilder::new();

    let p = builder.build();

    let file = File::open("testresources/kramnik.pgn");

    let unknown = "Unknown".to_string();

    let mut count = 0;
    for game in p.parse(file.unwrap()) {
        let white = game.tags.get("White").unwrap_or(&unknown);
        let black = game.tags.get("Black").unwrap_or(&unknown);
        println!("{} vs {} -> {}", white, black, game.game_result);
        count += 1;
    }
    println!("{} games", count);
    assert_eq!(count, 40);
}

#[test]
fn write_kramnik() {
    let chess_parser_builder = ChessParserBuilder::new();

    let p = chess_parser_builder.build();

    let file_to_read = File::open("testresources/kramnik.pgn");

    let file_to_write = File::create("testresources/kramnik_write.pgn");
    let chess_writer_builder = ChessWriterBuilder{};

    let mut chess_writer = chess_writer_builder.build(file_to_write.unwrap());
    
    for game in p.parse(file_to_read.unwrap()) {
        chess_writer.write(&game);
    }

}

