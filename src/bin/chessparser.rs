extern crate chessparser;

use std::fs::File;
use std::env;
use chessparser::base::parser::*;

pub fn main() {
    let args: Vec<String> = env::args().collect();
        // let fun = |tags: &HashMap<String,String>| tags.get("Result").map_or_else(|| false, |r| *r != "1/2-1/2".to_string() && 
    // *r != "0-1".to_string() && *r != "1-0".to_string());
    //let fun = |tags: &HashMap<String,String>| tags.get("Result").map_or_else(|| false, |r| *r == "0-0".to_string());
    //let fun = |tags: &HashMap<String,String>| tags.get("Result").is_none();


    let mut builder = ChessParserBuilder::new();
    // builder.ignore_comments();
    // builder.ignore_variations();
    //builder.tag_filter(&fun);
    let p = builder.build();

    //let file = File::open("/home/enrico/Documents/PGN/ficsgamesdb_201801_standard_nomovetimes_14117.pgn");
    let file = File::open(&args[1]);

    println!("{} games red.", p.parse(file.unwrap()).count());
}