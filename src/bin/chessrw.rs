extern crate chessrw;
extern crate clap;
extern crate separator;

use std::fs::File;
use chessrw::base::parser::*;
use chessrw::base::writer::*;
use std::time::Instant;
use std::time::Duration;
use clap::{Arg, App, ArgMatches};
use std::collections::HashMap;
use std::fs;
use separator::Separatable;

/**
 * ficsgamesdb_201801_standard_nomovetimes_14117.pgn
 * Downloaded from https://www.ficsgames.org/download.html :
 * Standard (all ratings)
 * 2018 / January
 * No move times
 * 
 * Without write file nor other filters --noprogress:
 * 81,886 games red in 1 second 329 millis.
 * 
 * Without write file and --blackwins --noprogress:
 * 37,541 games red in 0 seconds 964 millis.
 */
pub fn main() -> std::io::Result<()> {

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
            .arg(Arg::with_name("notags").long("notags"))
            .arg(Arg::with_name("whitewins").long("whitewins"))
            .arg(Arg::with_name("blackwins").long("blackwins"))
            .arg(Arg::with_name("minplycount").long("minplycount").takes_value(true))
            .arg(Arg::with_name("draw").long("draw"))
            .arg(Arg::with_name("noprogress").long("noprogress").help("No progress bar is showed (faster)."))
            .arg(Arg::with_name("onlymoves").long("onlymoves").help("Write only moves."))
            .arg(Arg::with_name("players").long("players").takes_value(true).help("A comma separated list of players. \
                Put an * as first character to get only games between players. \
                Put a +, - or = as first character of a player to get only wins, loses or draws for that player."))
            .get_matches();

    let input = matches.value_of("INPUT").unwrap();
    println!("Reading file {} ...", &input);

    let tags_filter = TagsFilter::new(&matches);

    let fun = |tags: &HashMap<String,String>| tags_filter.filter(tags);

    let mut builder = ChessParserBuilder::new();
    let only_moves = matches.is_present("onlymoves");
    if matches.is_present("nocomments")  || only_moves {
        builder.ignore_comments();
    }

    if matches.is_present("novariations") || only_moves {
        builder.ignore_variations();
    }

    if tags_filter.apply() {
        builder.tag_filter(&fun);
    }

    let metadata = fs::metadata(&input)?;
    let file = File::open(&input);
    println!("File size: {} bytes.", metadata.len().separated_string());

    if !matches.is_present("noprogress") {
        builder.file_size(metadata.len());
    }

    let p = builder.build();

    let start = Instant::now();

    if matches.is_present("OUTPUT") {
        let file_to_write = File::create(matches.value_of("OUTPUT").unwrap());
        let mut chess_writer_builder = ChessWriterBuilder::new();

        if matches.is_present("notags") || only_moves {
            chess_writer_builder.notags();
        }

        let mut chess_writer = chess_writer_builder.build(file_to_write.unwrap());
        
        let mut count = 0;

        let mut parsed = p.parse(file.unwrap());

        while parsed.next_temp() {
            chess_writer.write(&parsed).unwrap();
            count += 1;
        }
        println!("{} games written in {:?}.", count.separated_string(), start.elapsed());
    } else {
        let count = p.parse(file.unwrap()).size();
        println!("{} games red in {}.", count.separated_string(), format_duration(start.elapsed()));
    }

    Result::Ok(())
}

fn format_duration(duration: Duration) -> String {
    if duration.as_secs() == 1 {
        format!("1 second {} millis", duration.subsec_millis().separated_string())
    } else {
        format!("{} seconds {} millis", duration.as_secs().separated_string(), duration.subsec_millis().separated_string())
    }
}

struct TagsFilter<'a> {
    white_wins: bool,
    black_wins: bool,
    draw: bool,
    min_ply_count: Option<&'a str>,
    players: Option<&'a str>,
}

impl <'a> TagsFilter<'a> {

    fn new(matches: &'a ArgMatches<'a>) -> TagsFilter {
        TagsFilter{ white_wins: matches.is_present("whitewins"), black_wins: matches.is_present("blackwins"), 
            draw: matches.is_present("draw"), min_ply_count: matches.value_of("minplycount"),
            players : matches.value_of("players")}
    }

    fn filter(&self, tags: &HashMap<String,String>) -> bool {
        (!self.apply_result() || self.filter_result(tags)) &&
        (!self.apply_ply_count() || self.filter_ply_count(tags)) &&
        (!self.apply_players() || self.filter_players(tags))
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

    fn filter_players(&self, tags: &HashMap<String,String>) -> bool {
        tags.get("White").map_or_else(|| false, |wp| 
            tags.get("Black").map_or_else(|| false, |bp| {
                let mut result = false;
                let mut players = self.players.unwrap().to_lowercase();
                let between = players.chars().next().unwrap() == '*';

                if between {
                    players = players[1..].to_string();
                }

                let mut white = false;
                let mut black = false;
                for mut player in players.split(",") {
                    let first_char = player.chars().next().unwrap();
                    if first_char == '+' {
                        player = &player[1..];
                        white |= wp.to_lowercase().contains(player) && self.white_wins(tags);
                        black |= bp.to_lowercase().contains(player) && self.black_wins(tags);

                    } else if first_char == '-' {
                        player = &player[1..];
                        white |= wp.to_lowercase().contains(player) && self.black_wins(tags);
                        black |= bp.to_lowercase().contains(player) && self.white_wins(tags);
                    } else if first_char == '=' {
                        player = &player[1..];
                        white |= wp.to_lowercase().contains(player) && self.draw(tags);
                        black |= bp.to_lowercase().contains(player) && self.draw(tags);
                    } else {
                        white |= wp.to_lowercase().contains(player);
                        black |= bp.to_lowercase().contains(player);
                    }

                    if between {
                        result |= white && black;
                    } else {
                        result |= white || black;
                    }
                }
                result
            })
        )
    }

    fn white_wins(&self, tags: &HashMap<String,String>) -> bool {
        tags.get("Result").map_or_else(|| false, |r| r == "1-0")
    }

    fn black_wins(&self, tags: &HashMap<String,String>) -> bool {
        tags.get("Result").map_or_else(|| false, |r| r == "0-1")
    }

    fn draw(&self, tags: &HashMap<String,String>) -> bool {
        tags.get("Result").map_or_else(|| false, |r| r == "1/2-1/2")
    }

    fn apply(&self) -> bool {
        self.apply_result() || self.apply_ply_count() || self.apply_players()
    }

    fn apply_result(&self) -> bool {
        self.white_wins || self.black_wins || self.draw
    }

    fn apply_ply_count(&self) -> bool {
        self.min_ply_count.is_some()
    }

    fn apply_players(&self) -> bool {
        self.players.is_some()
    }
}

#[cfg(test)]

fn tags_filter_players(players: &str) -> TagsFilter {
    TagsFilter{
        white_wins: false,
        black_wins: false,
        draw: false,
        min_ply_count: None,
        players: Some(players),
    }
}

fn white_vs_black(white: &str, black: &str, result: Option<&str>) -> HashMap<String,String> {
    let mut tags : HashMap<String,String> = HashMap::new();
    tags.insert("White".to_string(), white.to_string());
    tags.insert("Black".to_string(), black.to_string());

    if result.is_some() {
        tags.insert("Result".to_string(), result.unwrap().to_string());
    }
    tags
}

#[test]
fn filter_players() {
    let tags_filter = tags_filter_players("capablanca");

    let tags = white_vs_black("Capablanca, Jose Raul", "Corzo y Prinzipe, Juan", None);

    assert_eq!(tags_filter.filter_players(&tags), true);
}

#[test]
fn filter_players_wins() {
    let tags_filter = tags_filter_players("+capablanca");

    let tags = white_vs_black("Capablanca, Jose Raul", "Corzo y Prinzipe, Juan", Some("1-0"));

    assert_eq!(tags_filter.filter_players(&tags), true);
}

#[test]
fn filter_players_wins_fail() {
    let tags_filter = tags_filter_players("+capablanca");

    let tags = white_vs_black("Capablanca, Jose Raul", "Corzo y Prinzipe, Juan", Some("0-1"));

    assert_eq!(tags_filter.filter_players(&tags), false);
}

#[test]
fn filter_players_loose() {
    let tags_filter = tags_filter_players("-capablanca");

    let tags = white_vs_black("Capablanca, Jose Raul", "Corzo y Prinzipe, Juan", Some("0-1"));

    assert_eq!(tags_filter.filter_players(&tags), true);
}

#[test]
fn filter_players_between() {
    let tags_filter = tags_filter_players("*capablanca,corzo");

    let tags = white_vs_black("Capablanca, Jose Raul", "Corzo y Prinzipe, Juan", None);

    assert_eq!(tags_filter.filter_players(&tags), true);
}

#[test]
fn filter_players_between_wins() {
    let tags_filter = tags_filter_players("*+capablanca,corzo");

    let tags = white_vs_black("Capablanca, Jose Raul", "Corzo y Prinzipe, Juan", Some("1-0"));

    assert_eq!(tags_filter.filter_players(&tags), true);
}

#[test]
fn filter_players_between_wins_fail() {
    let tags_filter = tags_filter_players("*+capablanca,corzo");

    let tags = white_vs_black("Capablanca, Jose Raul", "Corzo y Prinzipe, Juan", Some("0-1"));

    assert_eq!(tags_filter.filter_players(&tags), false);
}

#[test]
/**
 * Impossible since both cannot win!!!
 */
fn filter_players_between_wins_impossible_fail() {
    let tags_filter = tags_filter_players("*+capablanca,+corzo");

    let tags = white_vs_black("Capablanca, Jose Raul", "Corzo y Prinzipe, Juan", Some("1-0"));

    assert_eq!(tags_filter.filter_players(&tags), false);
}