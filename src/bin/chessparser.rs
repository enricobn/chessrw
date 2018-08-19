extern crate chessparser;

use std::fs::File;
use std::env;
use chessparser::base::parser::*;
use std::time::Instant;

/**
 * ficsgamesdb_201801_standard_nomovetimes_14117.pgn
 * secs: 1, nanos: 891236467
 */
pub fn main() {
    let args: Vec<String> = env::args().collect();
        // let fun = |tags: &HashMap<String,String>| tags.get("Result").map_or_else(|| false, |r| *r != "1/2-1/2".to_string() && 
    // *r != "0-1".to_string() && *r != "1-0".to_string());
    //let fun = |tags: &HashMap<String,String>| tags.get("Result").map_or_else(|| false, |r| *r == "0-0".to_string());
    //let fun = |tags: &HashMap<String,String>| tags.get("Result").is_none();
    println!("Reding file {} ...", &args[1]);

    let mut builder = ChessParserBuilder::new();
    // builder.ignore_comments();
    // builder.ignore_variations();
    //builder.tag_filter(&fun);
    let p = builder.build();

    //let file = File::open("/home/enrico/Documents/PGN/ficsgamesdb_201801_standard_nomovetimes_14117.pgn");
    let file = File::open(&args[1]);

    let start = Instant::now();
    println!("{} games red in {:?}.", p.parse(file.unwrap()).count(), start.elapsed());
}