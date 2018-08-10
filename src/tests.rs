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

    for game in p.parse(file.unwrap()) {
        //println!("Game", );
        if game.moves.len() > 0 {
            println!("{}", game.moves[0]);
        }

    }
}
