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

    let mut count = 0;
    for game in p.parse(file.unwrap()) {
        if count == 0 {
            for move_ in game.moves {
                println!("{}", move_);
            }
        }
        count += 1;
    }
    println!("{} games", count);
    assert_eq!(count, 40);
}
