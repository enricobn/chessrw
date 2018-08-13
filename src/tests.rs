//extern crate chessparser;
#[cfg(test)]

use base::parser::*;
use std::fs::File;

#[test]
fn it_works() {
    //assert_eq!(2 + 2, 4);
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
