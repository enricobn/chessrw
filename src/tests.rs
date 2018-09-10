use std::fs::File;
use std::fs;
use std::collections::HashMap;
use std::io::Read;

use base::fen::FENParserBuilder;
use base::game::*;
use base::parser::*;
use base::writer::ChessWriterBuilder;
use base::position::*;

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
fn parse_double_newline() {    
    let builder = ChessParserBuilder::new();
    let p = builder.build();

    let file = File::open("testresources/test.pgn");

    let games : Vec<ChessGameImpl> = collect(p.parse(file.unwrap()));

    games[4].get_moves().iter().for_each(|it| println!("{}", it));

    assert_eq!(5, games[4].get_moves().len());
    
    assert_eq!("d4", games[4].get_moves()[0]);
}


#[test]
fn parse_string() {
    let builder = ChessParserBuilder::new();
    let p = builder.build();
    //let games = collect(p.parse_string(&"1. d4 Nf6 2. c4 e6 3. Nc3 *".to_string()));

    let games : Vec<ChessGameImpl> = p.parse_string(&"1. d4 Nf6 2. c4 e6 3. Nc3 *".to_string()).collect();

    assert_eq!(5, games[0].get_moves().len());
}

#[test]
fn apply_moves_kramnik() {
    let builder = ChessParserBuilder::new();
    let p = builder.build();

    let file = File::open("testresources/kramnik.pgn");

    let mut count = 1;

    for game in p.parse(file.unwrap()) {
        let mut position = game.initial_position().unwrap().clone();

        let mut move_count = 1;
        for mv in game.get_moves() {
            let result = position.apply_move(mv);

            if result.is_some() {
                println!("{}", position.to_string());

                panic!("{} for game n. {} move {} (n. {})", result.unwrap(), count, mv, move_count);
            }

            move_count += 1;
        }

        count += 1;
    }
}

#[test]
fn apply_move_en_passant() {
    let fen_parser_builder = FENParserBuilder::new();
    let fen_parser = fen_parser_builder.build();
    let mut position = fen_parser.parse("4k3/8/8/8/4p3/8/5P2/4K3 w - - 1 1").unwrap();

    position.apply_move("f4");
    assert_eq!(Piece::WhitePawn, position.board.get_piece(6, 4));
    assert_eq!(Some(Square::new(6, 3)), position.en_passant_target_square);

    position.apply_move("exf3");
    assert_eq!(Piece::BlackPawn, position.board.get_piece(6, 3));
    assert_eq!(Piece::None, position.board.get_piece(6, 4));
}

#[test]
fn apply_move_two_pawns() {
    let fen_parser_builder = FENParserBuilder::new();
    let fen_parser = fen_parser_builder.build();
    let mut position = fen_parser.parse("r1bqkb1r/5p1p/p1np1p2/1p1Np3/4P3/N7/PPP2PPP/R2QKB1R b KQkq - 1 10").unwrap();

    assert_eq!(None, position.apply_move("f5"));

    assert_eq!(Piece::BlackPawn, position.board.get_piece(6, 5));
}

#[test]
fn king_in_check_knight() {
    let fen_parser_builder = FENParserBuilder::new();
    let fen_parser = fen_parser_builder.build();
    let mut position = fen_parser.parse("4k3/8/3N4/8/8/8/8/4K3 w KQkq - 0 1").unwrap();

    assert_eq!(true, position.king_in_check(ChessColor::Black));
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