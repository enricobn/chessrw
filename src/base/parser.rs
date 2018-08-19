use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::char;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use base::tag::*;
use base::fen::*;

#[derive(Clone)]
pub struct ChessParserConfig<'a> {
    ignore_comments: bool,
    ignore_variations: bool,
    tag_filter: Option<&'a Fn(&HashMap<String,String>) -> bool>,
}

pub struct ChessParserBuilder<'a> {
    config: ChessParserConfig<'a>,
}

impl <'a> ChessParserBuilder<'a> {

    pub fn new() -> Self {
        return ChessParserBuilder{config: ChessParserConfig{ignore_comments: false, ignore_variations: false, 
            tag_filter: None}};
    }

    pub fn ignore_comments(&mut self) {
        self.config.ignore_comments = true;
    }

    pub fn ignore_variations(&mut self) {
        self.config.ignore_variations = true;
    }

    pub fn tag_filter(&mut self, filter: &'a Fn(&HashMap<String,String>) -> bool) {
        self.config.tag_filter = Some(filter);
    }

    pub fn build(&self) -> ChessParserImpl {
        return ChessParserImpl::new(&self.config);
    }

}

pub trait ChessParser<'a> {

    fn parse(&self, file: File) -> ChessParserIterator;

}

pub struct ChessParserImpl<'a> {
    config: &'a ChessParserConfig<'a>,
}

impl <'a> ChessParser<'a> for ChessParserImpl<'a> {

    fn parse(&self, file: File) -> ChessParserIterator {
        let reader = BufReader::new(file);
        return ChessParserIterator::new(&self.config, reader);
    }

}

impl <'a> ChessParserImpl<'a> {

    pub fn new(config: &'a ChessParserConfig<'a>) -> Self {
        return ChessParserImpl::<'a>{config: config};
    }

}

type Int = i16;

pub struct ChessParserIterator<'a> {
    config: &'a ChessParserConfig<'a>,
    file_reader: BufReader<File>,
    buf: String,
    moves: Vec<String>,
    curr_move: String,
    status: Status,
    last_char: char,
    not_parsed: String,
    result_from_moves: String,
    tags: HashMap<String,String>, // TODO use a linked hash map?
    end_parse: bool,
    variations: HashMap<Int,Vec<String>>,
    after_variations_comments: HashMap<Int,HashMap<Int,String>>,
    comments: HashMap<Int,String>,
    tag_key: String,
    tag_value: String,
    reason: GameResultReason,
    result_from_tag: String,
    variation_count: i32,
    nags: HashMap<Int,Vec<String>>,
    ch: char,
    skip_game: bool,
}

enum GameResultReason {
    Abandoned,
    Adjudication,
    Death,
    Emergency,
    Normal,
    RulesInfraction,
    TimeForfait,
    Undeterminated,
}

fn result_from_pgn(s: String) -> Result<GameResultReason, ()> {
    match s.as_ref() {
        "abandoned" => Ok(GameResultReason::Abandoned),
        "adjudication" => Ok(GameResultReason::Adjudication),
        "death" => Ok(GameResultReason::Death),
        "emergency" => Ok(GameResultReason::Emergency),
        "normal" => Ok(GameResultReason::Normal),
        "rules infraction" => Ok(GameResultReason::RulesInfraction),
        "time forfait" => Ok(GameResultReason::TimeForfait),
        "undeterminated" => Ok(GameResultReason::Undeterminated),
        _ => Err(()),
    }
}


impl <'a> ChessParserIterator<'a> {

    pub fn new(config: &'a ChessParserConfig<'a>, file_reader: BufReader<File>) -> Self {
        return ChessParserIterator{config: config, file_reader: file_reader, buf: String::new(), moves: Vec::new(), 
            curr_move: String::new(), status: Status::Headings, last_char: char::from_digit(0, 10).unwrap(),
            not_parsed: String::new(), result_from_moves: String::new(), tags: HashMap::new(), end_parse: false,
            variations: HashMap::new(), after_variations_comments: HashMap::new(), comments: HashMap::new(),
            tag_key: String::new(), tag_value: String::new(), reason: GameResultReason::Normal, result_from_tag: String::new(),
            variation_count: 0, nags: HashMap::new(), ch: char::from_digit(0, 10).unwrap(), skip_game: false};
    }

    fn get_game(&mut self) -> (bool, Option<ChessGame>) {
        // no char has been parsed, it's not a game
        if self.status == Status::Ready {
            return (false, None);
        }
        // it can happens if there's no result, but it's wrong, since PGN format says it's mandatory. 
        // However it may happen in variations
        if self.status == Status::Move && self.not_parsed.len() > 0 {
            self.moves.push(self.not_parsed.clone());
            // TODO check if I need a new instance
            self.not_parsed.clear();
            self.status = Status::Moves;
        // the file is ended just after the result
        } else if self.status == Status::GameResult && self.not_parsed.len() > 0 {
            self.result_from_moves = self.not_parsed.clone();
            // TODO check if I need a new instance
            self.not_parsed.clear();
            self.status = Status::Moves;
        }

        // TODO can I remove clone with a borrow?
        let result = Some(ChessGame{tags: self.tags.clone(), moves: self.moves.clone(), 
            comments: self.comments.clone(), variations: self.variations.clone(),
            after_variations_comments: self.after_variations_comments.clone(),
            game_result: self.result_from_moves.clone(), nags: self.nags.clone()});
        return (false, result)
    }

    fn parse_comments(&mut self, c: char) {
        if c == '}' {
            if !self.config.ignore_comments {
                let last_move_index = self.moves.last_index();
                let variations = self.variations.get(&last_move_index);
                if variations.is_some() && variations.unwrap().len() > 0 {
                    let av_move_comments = match self.after_variations_comments.entry(last_move_index) {
                        Entry::Occupied(o) => o.into_mut(),
                        Entry::Vacant(v) => {
                            v.insert(HashMap::new())
                        }
                    };
                    av_move_comments.insert(self.variations[&last_move_index].last_index(), self.not_parsed.trim_right().to_string());
                } else {
                    self.comments.insert(last_move_index, self.not_parsed.trim_right().to_string());
                }
            }
            self.not_parsed.clear();
            self.status = Status::Moves;
        } else if self.config.ignore_comments {
        } else if c == '\n' {
            self.not_parsed.push(' ');
        } else {
            self.not_parsed.push(c);
        }
    }

    fn parse_heading(&mut self, c: char) {
        if c == ']' {
            if !self.tag_key.is_empty() && !self.tag_value.is_empty() {
                self.tags.insert(self.tag_key.clone(), self.tag_value.clone());
                if &self.tag_key as &str == "Termination" {
                    // TODO check result of result_from_pgn, avoid clone
                    self.reason = result_from_pgn(self.tag_value.clone()).unwrap();
                } else if &self.tag_key as &str == "Result" {
                    self.result_from_tag = self.tag_value.clone();
                }
            }
            self.status = Status::Headings;
            self.tag_key.clear();
            self.tag_value.clear();
            self.not_parsed.clear();
        } else if c == '"' {
            self.tag_key.push_str(&self.not_parsed);
            self.status = Status::HeadingValue;
            self.not_parsed.clear();
        } else if c.is_whitespace() {
        } else {
            self.not_parsed.push(c);
        }
    }

    fn parse_variation(&mut self, c: char) {
        if c == ')' {
            self.variation_count -= 1;
            if self.variation_count < 0 {
                if !self.config.ignore_variations {
                    let last_move_index = self.moves.last_index();
                    let moves_variations = match self.variations.entry(last_move_index) {
                        Entry::Occupied(o) => o.into_mut(),
                        Entry::Vacant(v) => {
                            v.insert(Vec::new())
                        }
                    };
                    moves_variations.push(self.not_parsed.trim_right().to_string());
                }
                self.not_parsed.clear();
                self.status = Status::Moves;
                self.variation_count = 0;
            }
        } else if c == '(' {
            self.variation_count += 1;
            if !self.config.ignore_variations {
                self.not_parsed.push(c);
            }
        } else if self.config.ignore_variations {
        } else if c == '\n' {
            self.not_parsed.push(' ');
        } else {
            self.not_parsed.push(c);
        }
    }

    fn parse_move(&mut self, c: char) {
        if c.is_whitespace() {
            //println!("{}", self.not_parsed.trim_right().to_string());
            self.moves.push(self.not_parsed.clone());
            self.not_parsed.clear();
            self.status = Status::Moves;
        } else {
            self.not_parsed.push(c);
        }
    }

    fn parse_numeric_annotation_glyph(&mut self, c: char) {
        if !c.is_digit(10) {
            if self.not_parsed.len() > 0 {
                let last_move_index = self.moves.last_index();
                let move_nags = match self.nags.entry(last_move_index) {
                    Entry::Occupied(o) => o.into_mut(),
                    Entry::Vacant(v) => {
                        v.insert(Vec::new())
                    }
                };
                move_nags.push(self.not_parsed.to_string());
            }
            self.not_parsed.push(c);
            self.status = Status::Moves;
        } else {
            self.not_parsed.push(c);
        }
    }

    fn clear(&mut self) {
        self.buf.clear();
        self.moves.clear(); 
        self.curr_move.clear();
        self.status = Status::Headings;
        self.not_parsed.clear();
        self.result_from_moves.clear();
        self.tags.clear();
        self.variations.clear();
        self.after_variations_comments.clear();
        self.comments.clear();
        self.tag_key.clear();
        self.tag_value.clear();
        self.reason = GameResultReason::Normal;
        self.result_from_tag = String::new();
        self.variation_count= 0;
        self.nags.clear();
        self.skip_game = false;
    }

}

trait Sizable<T> {

    fn size(&self) -> Int;

    fn last_index(&self) -> Int;

}

impl <T> Sizable<T> for Vec<T> {

    fn size(&self) -> Int {
        return self.len() as Int;
    }

    fn last_index(&self) -> Int {
        return self.size() -1;
    }

}

#[derive(Eq, PartialEq)]
enum Status {
        Headings,
        Heading,
        HeadingValue,
        Moves,
        Variation,
        Comment,
        MoveUnknown, // can be a move number or a game result  
        MoveNumber, 
        GameResult,
        Move,
        NumericAnnotationGlyph,
        Ready, // no char has been parsed
    }

impl <'a> Iterator for ChessParserIterator<'a> {
    type Item = ChessGame;

    fn next(&mut self) -> Option<ChessGame> {
        if self.end_parse {
            return None;
        }

        self.clear();

        loop {
            let count = self.file_reader.read_line(&mut self.buf);
            
            if count.is_err() {
                // TODO
                continue;
            }

            if count.unwrap() <= 0 {
                if self.moves.is_empty() {
                    return None;
                } else {
                    let (_, game) = self.get_game();
                    if game.is_some() {
                        return game;
                    }
                }
            } else {
                let line = self.buf.to_string();

                for c in line.chars() {

                    if c == '\r' {
                        continue;
                    }

                    self.last_char = self.ch.clone();
                    self.ch = c.clone();
                    if self.status == Status::Ready {
                        if !c.is_whitespace() {
                            self.status = Status::Headings;
                        } else {
                            continue;
                        }
                    }

                    if c == '\n' {
                        if self.last_char == '\n' {
                            // it's the new line after headings, so now there's moves
                            if self.status == Status::Headings {
                                self.status = Status::Moves;
                                self.skip_game = self.config.tag_filter.map_or_else(|| false, |f| !f(&self.tags));
                            } else {

                                if self.skip_game {
                                    self.clear();
                                    continue;
                                }
                                // it's the new line after end of moves, so I add the new game and prepare to parse another

                                // if cancelled
                                let (cancelled, game) = self.get_game();
                                if cancelled {
                                    self.end_parse = true;
                                }

                                if game.is_some() {
                                    return game;
                                }
                            }
                        }
                    }

                    if self.skip_game {
                        continue;
                    }

                    if self.status == Status::Comment {
                        self.parse_comments(c);
                        continue;
                    }

                    if self.status == Status::Variation {
                        self.parse_variation(c);
                        continue;
                    }

                    if self.status == Status::Headings {
                        if c == '[' {
                            self.not_parsed.clear();
                            self.status = Status::Heading;
                            continue;
                        } else if c.is_whitespace() {
                            continue;
                        } else {
                            self.skip_game = self.config.tag_filter.map_or_else(|| false, |f| !f(&self.tags));

                            self.status = Status::Moves;

                            if self.skip_game {
                                continue;
                            }
                        }
                    }

                    if self.status == Status::Heading {
                        self.parse_heading(c);
                        continue;
                    }

                    if self.status == Status::HeadingValue {
                        if c == '"' {
                            self.tag_value.push_str(&self.not_parsed);
                            self.status = Status::Heading;
                            self.not_parsed.clear();
                        } else {
                            self.not_parsed.push(c);
                        }
                    }

                    if self.status == Status::MoveUnknown {
                        if c == '.' {
                            self.status = Status::MoveNumber;
                            self.not_parsed.clear();
                        } else if c == '-' || c == '*' {
                            self.status = Status::GameResult;
                            self.not_parsed.push(c);
                        } else {
                            self.not_parsed.push(c);
                        }
                        continue;
                    }
                    
                    if self.status == Status::GameResult {
                        if c.is_whitespace() {
                            self.result_from_moves = self.not_parsed.clone();
                            self.not_parsed.clear();
                            self.status = Status::Moves;
                        } else {
                            self.not_parsed.push(c);
                        }
                        continue;
                    }

                    if self.status == Status::MoveNumber {
                        if c.is_whitespace() || c == '.' {
                        } else {
                            self.status = Status::Move;
                            self.not_parsed.push(c);
                        }
                        continue;
                    }

                    if self.status == Status::Move {
                        self.parse_move(c);
                        continue;
                    }

                    if self.status == Status::NumericAnnotationGlyph {
                        self.parse_numeric_annotation_glyph(c);
                        continue;
                    }

                    if self.status == Status::Moves {
                        if c == '{' {
                            self.status = Status::Comment;
                        } else if c == '(' {
                            self.status = Status::Variation;
                        } else if c == '$' {
                            self.status = Status::NumericAnnotationGlyph;
                        } else if c == '*' {
                            self.status = Status::GameResult;
                            self.not_parsed.push(c);
                        } else if c.is_digit(10) {
                            self.status = Status::MoveUnknown;
                            self.not_parsed.push(c);
                        } else if c.is_whitespace() {

                        } else {
                            self.status = Status::Move;
                            self.not_parsed.push(c);
                        }
                    }
                }
                self.buf.clear();
            }
        }
    }

}

pub struct ChessGame {
    tags: HashMap<String,String>,
    moves: Vec<String>,
    comments: HashMap<Int,String>,
    variations: HashMap<Int,Vec<String>>,
    after_variations_comments: HashMap<Int,HashMap<Int,String>>,
    game_result: String,
    nags: HashMap<Int,Vec<String>>,
}

lazy_static! {
    static ref FEN_PARSER: FENParser = {
        FENParserBuilder::new().build()
    };
}

impl ChessGame {

    pub fn get_tags(&self) -> &HashMap<String,String> {
        &self.tags
    }

    pub fn get_moves(&self) -> &Vec<String> {
        &self.moves
    }

    pub fn get_before_moves_comment(&self) -> Option<&String> {
        self.comments.get(&-1)
    }

    /** 
     * # Arguments
     * * `after_move` is zero based.
     */
    pub fn get_comment(&self, after_move: Int) -> Option<&String> {
        self.comments.get(&after_move)
    }

    pub fn get_game_result(&self) -> &String {
        &self.game_result
    }

    pub fn initial_position(&self) -> Result<ChessPosition,String> {
        match self.tags.get(&Tag::FEN.to_string()) {
            // TODO error handling
            Some(fen) => FEN_PARSER.parse(fen),
            _ => Result::Ok(ChessPosition::initial_position())
        }
    }

    /** 
     * # Arguments
     * * `after_move` is zero based.
     */
    pub fn get_nags(&self, after_move: Int) -> Option<&Vec<String>> {
        self.nags.get(&after_move)
    }

    /** 
     * # Arguments
     * * `after_move` is zero based.
     */
    pub fn get_variations(&self, after_move: Int) -> Option<&Vec<String>> {
        self.variations.get(&after_move)
    }

    /** 
     * # Arguments
     * * `after_move` is zero based.
     * * `after_variation_move` is zero based
     */
    pub fn get_after_variation_comment(&self, after_move: Int, after_variation_move: Int) -> Option<&String> {
        match self.after_variations_comments.get(&after_move) {
            Some(avc) => avc.get(&after_variation_move),
            _ => None
        }
    }
}