use base::parser::{ChessGame, ChessGameImpl, ChessParser, ChessParserBuilder, ChessParserIterator};
use base::writer::ChessWriterBuilder;
use base::fen::{ChessColor, FENParserBuilder};
use std::fs::File;
use std::fs;
use std::collections::HashMap;
use std::io::Read;

#[cfg(test)]

#[test]
fn capablanca_is_my_favorite() {
    let mut builder = ChessParserBuilder::new();
    builder.ignore_comments();
    builder.ignore_variations();
    let p = builder.build();

    let file = File::open("testresources/Capablanca.pgn");

    assert_eq!(p.parse(file.unwrap()).count(), 597);
}

#[test]
fn parse_kramnik() {
    let mut builder = ChessParserBuilder::new();
    builder.ignore_comments();
    builder.ignore_variations();
    let p = builder.build();

    let file = File::open("testresources/kramnik.pgn");

    assert_eq!(p.parse(file.unwrap()).count(), 40);
}

#[test]
fn write_kramnik() {
    let chess_parser_builder = ChessParserBuilder::new();

    let p = chess_parser_builder.build();

    let file_to_read = File::open("testresources/kramnik.pgn");

    fs::create_dir_all("target/tmp").unwrap();

    let file_to_write = File::create("target/tmp/kramnik_write.pgn");
    let chess_writer_builder = ChessWriterBuilder::new();

    let mut chess_writer = chess_writer_builder.build(file_to_write.unwrap());
    
    for game in p.parse(file_to_read.unwrap()) {
        chess_writer.write(&game).unwrap();
    }
}

#[test]
fn write_kramnik_ignore() {
    let mut chess_parser_builder = ChessParserBuilder::new();
    chess_parser_builder.ignore_comments();
    chess_parser_builder.ignore_variations();
    let p = chess_parser_builder.build();

    let file_to_read = File::open("testresources/kramnik.pgn");

    fs::create_dir_all("target/tmp").unwrap();

    let file_to_write = File::create("target/tmp/kramnik_write_ignore.pgn");
    let chess_writer_builder = ChessWriterBuilder::new();

    let mut chess_writer = chess_writer_builder.build(file_to_write.unwrap());
    
    for game in p.parse(file_to_read.unwrap()) {
        chess_writer.write(&game).unwrap();
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

#[test]
fn parse_no_tags_test() {
    let builder = ChessParserBuilder::new();
    let p = builder.build();

    let file = File::open("testresources/test.pgn");

    let games : Vec<ChessGameImpl> = collect(p.parse(file.unwrap()));

    assert_eq!(5, games[0].get_moves().len());
}

#[test]
fn parse_tags_test() {
    let builder = ChessParserBuilder::new();
    let p = builder.build();

    let file = File::open("testresources/test.pgn");

    let games : Vec<ChessGameImpl> = collect(p.parse(file.unwrap()));

    assert_eq!(5, games[1].get_moves().len());

    let tags = games[1].get_tags();

    assert_eq!(1, tags.len());

    assert_eq!("Test", tags["Event"]);
}

#[test]
fn parse_comments_test() {
    let builder = ChessParserBuilder::new();
    let p = builder.build();

    let file = File::open("testresources/test.pgn");

    let games : Vec<ChessGameImpl> = collect(p.parse(file.unwrap()));

    assert_eq!(5, games[2].get_moves().len());

    let mut comment = games[2].get_comment(0);

    assert_eq!("A comment", comment.unwrap());

    comment = games[2].get_comment(3);

    assert_eq!("Another comment", comment.unwrap());
}

#[test]
fn parse_variations_test() {
    let builder = ChessParserBuilder::new();
    let p = builder.build();

    let file = File::open("testresources/test.pgn");

    let games : Vec<ChessGameImpl> = collect(p.parse(file.unwrap()));

    assert_eq!(50, games[3].get_moves().len());

    let variations = games[3].get_variations(49);

    assert_eq!("25... Qh4 $5 $11 {is interesting}", variations.unwrap()[0]);
}

#[test]
fn parse_with_tags_filter_test() {    
    let mut builder = ChessParserBuilder::new();
    builder.tag_filter(&self::filter_tags_by_event);
    let p = builder.build();

    let file = File::open("testresources/test.pgn");

    let games : Vec<ChessGameImpl> = collect(p.parse(file.unwrap()));

    assert_eq!(1, games.len());

    assert_eq!(5, games[0].get_moves().len());
}

#[test]
fn parse_string() {
    let mut builder = ChessParserBuilder::new();
    let p = builder.build();
    //let games = collect(p.parse_string(&"1. d4 Nf6 2. c4 e6 3. Nc3 *".to_string()));

    let games : Vec<ChessGameImpl> = p.parse_string(&"1. d4 Nf6 2. c4 e6 3. Nc3 *".to_string()).collect();

    assert_eq!(5, games[0].get_moves().len());
}

fn  collect<'a,R: Read>(mut it: ChessParserIterator<'a,R>) -> Vec<ChessGameImpl> {
    let mut result = Vec::new();

    while it.next_temp() {
        result.push(it.to_game());
    }
    result
}

fn filter_tags_by_event(tags: &HashMap<String,String>) -> bool {
    tags.get("Event").map_or_else(|| false, |r| r == "Test")
}