extern crate chessparser;
extern crate clap;

use std::fs::File;
use chessparser::base::parser::*;
use chessparser::base::writer::*;
use std::time::Instant;
use clap::{Arg, App};
use std::collections::HashMap;

/**
 * ficsgamesdb_201801_standard_nomovetimes_14117.pgn
 * secs: 1, nanos: 964518173
 */
pub fn main() {

    let matches = 
        App::new("Chess read / write")
            .version("0.1.0")
            .arg(Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1))
            .arg(Arg::with_name("OUTPUT")
                .help("Sets the output file to use")
                .required(false)
                .index(2))
            .arg(Arg::with_name("nocomments").long("nocomments"))
            .arg(Arg::with_name("novariations").long("novariations"))
            .arg(Arg::with_name("whitewins").long("whitewins"))
            .arg(Arg::with_name("blackwins").long("blackwins"))
            .arg(Arg::with_name("draw").long("draw"))
            .get_matches();

    let input = matches.value_of("INPUT").unwrap();
    println!("Reding file {} ...", &input);

    let mut builder = ChessParserBuilder::new();
    if matches.is_present("nocomments") {
        builder.ignore_comments();
    }

    if matches.is_present("novariations") {
        builder.ignore_variations();
    }

    if matches.is_present("whitewins") {
        builder.tag_filter(&::white_wins);
    }

    if matches.is_present("blackwins") {
        builder.tag_filter(&::black_wins);
    }

    if matches.is_present("draw") {
        builder.tag_filter(&::draw);
    }

    //builder.tag_filter(&fun);
    let p = builder.build();

    let file = File::open(&input);

    let start = Instant::now();
    if matches.is_present("OUTPUT") {
        let file_to_write = File::create(matches.value_of("OUTPUT").unwrap());
        let chess_writer_builder = ChessWriterBuilder{};

        let mut chess_writer = chess_writer_builder.build(file_to_write.unwrap());
        
        let mut count = 0;
        for game in p.parse(file.unwrap()) {
            chess_writer.write(&game).unwrap();
            count += 1;
        }
        println!("{} games written in {:?}.", count, start.elapsed());
    } else {
        println!("{} games red in {:?}.", p.parse(file.unwrap()).count(), start.elapsed());
    }
}

fn white_wins(tags: &HashMap<String,String>) -> bool {
    tags.get("Result").map_or_else(|| false, |r| r == "1-0")
}

fn black_wins(tags: &HashMap<String,String>) -> bool {
    tags.get("Result").map_or_else(|| false, |r| r == "0-1")
}

fn draw(tags: &HashMap<String,String>) -> bool {
    tags.get("Result").map_or_else(|| false, |r| r == "1/2-1/2")
}