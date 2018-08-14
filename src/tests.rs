//extern crate chessparser;
#[cfg(test)]

use base::parser::*;
use base::writer::*;
use base::fen::*;
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
        let white = game.get_tags().get("White").unwrap_or(&unknown);
        let black = game.get_tags().get("Black").unwrap_or(&unknown);
        println!("{} vs {} -> {}", white, black, game.get_game_result());
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

#[test]
fn fen_parse() {
    let fen_parser_builder = FENParserBuilder::new();
    let fen_parser = fen_parser_builder.build();
    let chess_position = fen_parser.parse("8/8/7p/5Kpk/8/7P/6P1/8 w - - 0 1").unwrap();
    
    assert_eq!(chess_position.active_color, ChessColor::White);
    assert_eq!(chess_position.half_move_clock, 0);
    assert_eq!(chess_position.full_move_number, 1);
}

