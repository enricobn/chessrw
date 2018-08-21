extern crate chessrw;
extern crate clap;

use std::fs::File;
use chessrw::base::parser::*;
use chessrw::base::writer::*;
use std::time::Instant;
use clap::{Arg, App, ArgMatches};
use std::collections::HashMap;

/**
 * ficsgamesdb_201801_standard_nomovetimes_14117.pgn
 * Downloaded from https://www.ficsgames.org/download.html :
 * Standard (all ratings)
 * 2018 / January
 * No move times
 * 
 * Without write file nor other filters:
 * 81886 games red in Duration {secs: 1, nanos: 877824005}.
 * 
 * Without write file and --blackwins:
 * 37541 games red in Duration { secs: 1, nanos: 286754330 }.
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
            .arg(Arg::with_name("minplycount").long("minplycount").takes_value(true))
            .arg(Arg::with_name("draw").long("draw"))
            .get_matches();

    let input = matches.value_of("INPUT").unwrap();
    println!("Reading file {} ...", &input);

    let tags_filter = TagsFilter::new(&matches);

    let fun = |tags: &HashMap<String,String>| tags_filter.filter(tags);

    let mut builder = ChessParserBuilder::new();
    if matches.is_present("nocomments") {
        builder.ignore_comments();
    }

    if matches.is_present("novariations") {
        builder.ignore_variations();
    }

    if tags_filter.apply() {
        builder.tag_filter(&fun);
    }

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

struct TagsFilter<'a> {
    white_wins: bool,
    black_wins: bool,
    draw: bool,
    min_ply_count: Option<&'a str>,
}

impl <'a> TagsFilter<'a> {

    fn new(matches: &'a ArgMatches<'a>) -> TagsFilter {
        TagsFilter{ white_wins: matches.is_present("whitewins"), black_wins: matches.is_present("blackwins"), 
            draw: matches.is_present("draw"), min_ply_count: matches.value_of("minplycount")}
    }

    fn filter(&self, tags: &HashMap<String,String>) -> bool {
        (!self.apply_result() || self.filter_result(tags)) &&
        (!self.apply_ply_count() || self.filter_ply_count(tags))
    }

    fn filter_result(&self, tags: &HashMap<String,String>) -> bool {
        tags.get("Result").map_or_else(|| false, |r| 
            self.white_wins && r == "1-0" ||
            self.black_wins && r == "0-1" ||
            self.draw && r == "1/2-1/2"
        )
    }

    fn filter_ply_count(&self, tags: &HashMap<String,String>) -> bool {
        // TODO parse error
        let min_ply_count = self.min_ply_count.unwrap().parse::<i32>().unwrap();
        tags.get("PlyCount").map_or_else(|| false, |r| 
            match r.parse::<i32>() {
                Ok(ply_count) => ply_count >= min_ply_count,
                _ => false
            }
        )
    }

    fn apply(&self) -> bool {
        self.apply_result() || self.apply_ply_count()
    }

    fn apply_result(&self) -> bool {
        self.white_wins || self.black_wins || self.draw
    }

    fn apply_ply_count(&self) -> bool {
        self.min_ply_count.is_some()
    }

}