use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::char;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use base::tag::*;
use base::fen::*;

pub struct ChessParserBuilder {

}

impl ChessParserBuilder {

    pub fn new() -> Self {
        return ChessParserBuilder{};
    }

    pub fn build(&self) -> ChessParserImpl {
        return ChessParserImpl::new();
    }

}

pub trait ChessParser {

    fn parse(&self, file: File) -> ChessParserIterator;

}

pub struct ChessParserImpl {
    
}

impl ChessParser for ChessParserImpl {

    fn parse(&self, file: File) -> ChessParserIterator {
        let reader = BufReader::new(file);
        return ChessParserIterator::new(reader);
    }

}

impl ChessParserImpl {

    pub fn new() -> Self {
        return ChessParserImpl{};
    }

}

type Int = i16;

pub struct ChessParserIterator {
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
    tag_key: Option<String>,
    tag_value: Option<String>,
    reason: GameResultReason,
    result_from_tag: String,
    variation_count: i32,
    nags: HashMap<Int,Vec<String>>,
    ch: char,
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


impl ChessParserIterator {

    pub fn new(file_reader: BufReader<File>) -> Self {
        return ChessParserIterator{file_reader: file_reader, buf: String::new(), moves: Vec::new(), 
            curr_move: String::new(), status: Status::Headings, last_char: char::from_digit(0, 10).unwrap(),
            not_parsed: String::new(), result_from_moves: String::new(), tags: HashMap::new(), end_parse: false,
            variations: HashMap::new(), after_variations_comments: HashMap::new(), comments: HashMap::new(),
            tag_key: None, tag_value: None, reason: GameResultReason::Normal, result_from_tag: "".to_string(),
            variation_count: 0, nags: HashMap::new(), ch: char::from_digit(0, 10).unwrap()};
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

        /*
        Date date = getDate(tags);
        ChessPosition position = getPosition(tags);
        
        ChessGame game = new ChessGame(_gameType, date, position);
        String result = resultFromMoves;
        if (result == null) {
            result = resultFromTag;
        }
        if (result != null) {
            if (reason == null) {
                game.end(GameResult.fromPGN(result));
            } else {
                game.end(new GameResult(GameResult.fromPGN(result).getWinner(), reason));
            }
        }
        game.addTags(tags);
        int i = 0;
        for (String move : moves) {
            SANMove sanMove = new SANMove(_gameType, game.getPosition(), move);
            game.addMove(sanMove.getMove());
            if (sanMove.getNAG() != null) {
                game.addMoveAnnotation(i, sanMove.getNAG());
            }
            i++;
        }

        game.addNAGs(nags);
        
        for (Map.Entry<Integer, String> comment: comments.entrySet()) {
            game.addComment(comment.getKey(), comment.getValue());
        }
        
        for (Map.Entry<Integer, List<String>> entry: variations.entrySet()) {
            for (String variation : entry.getValue()) {
                game.addVariation(entry.getKey(), variation);
            }
        }
        
        for (Entry<Integer, Map<Integer, String>> avMoveComments: afterVariationsComments.entrySet()) {
            for (Map.Entry<Integer, String> comment: avMoveComments.getValue().entrySet()) {
                game.addAfterVariationComment(avMoveComments.getKey(), comment.getKey(), comment.getValue());
            }
        }
        
        games.add(game);
        for (GamesImporterListener<ChessPosition> listener : _listeners) {
            if (listener.gameLoaded(game)) {
                return true;
            }
        }
        */
        let result = Some(ChessGame{tags: self.tags.clone(), moves: self.moves.clone(), 
            comments: self.comments.clone(), variations: self.variations.clone(),
            after_variations_comments: self.after_variations_comments.clone(),
            game_result: self.result_from_moves.clone()});
        self.clear();
        return (false, result)
    }

    fn parse_comments(&mut self, c: char) {
        if c == '}' {
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
            self.not_parsed.clear();
            self.status = Status::Moves;
        } else if c == '\n' {
            self.not_parsed += " ";
        } else {
            self.not_parsed += &c.to_string();
        }
    }

    fn parse_heading(&mut self, c: char) {
        if c == ']' {
            match self.tag_key {
                Some(ref tag_key) => {
                    match self.tag_value {
                        Some(ref tag_value) => {
                            self.tags.insert(tag_key.clone(), tag_value.clone());
                            if *tag_key == Tag::Termination.to_string() {
                                // TODO check result of result_from_pgn, avoid clone
                                self.reason = result_from_pgn(tag_value.clone()).unwrap();
                            } else if *tag_key == Tag::Result.to_string() {
                                self.result_from_tag = tag_value.clone();
                            }        
                        },
                        _ => (),
                    }
                },
                _ => (),
            }
            self.status = Status::Headings;
            self.tag_key = None;
            self.tag_value = None;
            self.not_parsed.clear();
        } else if c == '"' {
            self.tag_key = Some(self.not_parsed.clone());
            self.status = Status::HeadingValue;
            self.not_parsed.clear();
        } else if c.is_whitespace() {
        } else {
            self.not_parsed += &c.to_string();
        }
    }

    fn parse_variation(&mut self, c: char) {
        if c == '(' {
            self.variation_count += 1;
            self.not_parsed += &c.to_string();
        } else if c == ')' {
            let last_move_index = self.moves.last_index();
            self.variation_count -= 1;
            if self.variation_count < 0 {
                let moves_variations = match self.variations.entry(last_move_index) {
                    Entry::Occupied(o) => o.into_mut(),
                    Entry::Vacant(v) => {
                        v.insert(Vec::new())
                    }
                };
                moves_variations.push(self.not_parsed.trim_right().to_string());
                self.not_parsed.clear();
                self.status = Status::Moves;
                self.variation_count = 0;
            }
        } else if c == '\n' {
            self.not_parsed += " ";
        } else {
            self.not_parsed += &c.to_string();
        }
    }

    fn parse_move(&mut self, c: char) {
        if c.is_whitespace() {
            //println!("{}", self.not_parsed.trim_right().to_string());
            self.moves.push(self.not_parsed.clone());
            self.not_parsed.clear();
            self.status = Status::Moves;
        } else {
            self.not_parsed += &c.to_string();
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
            self.not_parsed += &c.to_string();
            self.status = Status::Moves;
        } else {
            self.not_parsed += &c.to_string();
        }
    }

    fn clear(&mut self) {
        self.buf.clear();
        self.moves.clear(); 
        self.curr_move.clear();
        self.status = Status::Headings;
        // self.last_char = char::from_digit(0, 10).unwrap();
        self.not_parsed.clear();
        self.result_from_moves.clear();
        self.tags.clear();
        self.end_parse = false;
        self.variations.clear();
        self.after_variations_comments.clear();
        self.comments.clear();
        self.tag_key = None;
        self.tag_value = None;
        self.reason = GameResultReason::Normal;
        self.result_from_tag = "".to_string();
        self.variation_count= 0;
        self.nags.clear();
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

impl Iterator for ChessParserIterator {
    type Item = ChessGame;

    fn next(&mut self) -> Option<ChessGame> {
        if self.end_parse {
            return None;
        }

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
                            // self.last_char = c;
                            continue;
                        }
                    }

                    if c == '\n' {
                        if self.last_char == '\n' {
                            // it's the new line after headings, so now there's moves
                            if self.status == Status::Headings {
                                self.status = Status::Moves;
                            } else {
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

                    if self.status == Status::Comment {
                        self.parse_comments(c);
                        // self.last_char = c;
                        continue;
                    }

                    if self.status == Status::Variation {
                        self.parse_variation(c);
                        // self.last_char = c;
                        continue;
                    }

                    if self.status == Status::Headings {
                        if c == '[' {
                            self.not_parsed.clear();
                            self.status = Status::Heading;
                            // self.last_char = c;
                            continue;
                        } else if c.is_whitespace() {
                            // self.last_char = c;
                            continue;
                        } else {
                            self.status = Status::Moves;
                        }
                    }

                    if self.status == Status::Heading {
                        self.parse_heading(c);
                        // self.last_char = c;
                        continue;
                    }

                    if self.status == Status::HeadingValue {
                        if c == '"' {
                            self.tag_value = Some(self.not_parsed.clone());
                            self.status = Status::Heading;
                            self.not_parsed.clear();
                        } else {
                            self.not_parsed += &c.to_string();
                        }
                    }

                    if self.status == Status::MoveUnknown {
                        if c == '.' {
                            self.status = Status::MoveNumber;
                            self.not_parsed.clear();
                        } else if c == '-' || c == '*' {
                            self.status = Status::GameResult;
                            self.not_parsed += &c.to_string();
                        } else {
                            self.not_parsed += &c.to_string();
                        }
                        // self.last_char = c;
                        continue;
                    }
                    
                    if self.status == Status::GameResult {
                        if c.is_whitespace() {
                            self.result_from_moves = self.not_parsed.clone();
                            self.not_parsed.clear();
                            self.status = Status::Moves;
                        } else {
                            self.not_parsed += &c.to_string();
                        }
                        // self.last_char = c;
                        continue;
                    }

                    if self.status == Status::MoveNumber {
                        if c.is_whitespace() || c == '.' {
                        } else {
                            self.status = Status::Move;
                            self.not_parsed += &c.to_string();
                        }
                        // self.last_char = c;
                        continue;
                    }

                    if self.status == Status::Move {
                        self.parse_move(c);
                        // self.last_char = c;
                        continue;
                    }

                    if self.status == Status::NumericAnnotationGlyph {
                        self.parse_numeric_annotation_glyph(c);
                        // self.last_char = c;
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
                            self.not_parsed += &c.to_string();
                        } else if c.is_digit(10) {
                            self.status = Status::MoveUnknown;
                            self.not_parsed += &c.to_string();
                        } else if c.is_whitespace() {

                        } else {
                            self.status = Status::Move;
                            self.not_parsed += &c.to_string();
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
        self.comments.get(&-1).clone()
    }

    pub fn get_comment(&self, after_move: Int) -> Option<&String> {
        self.comments.get(&after_move).clone()
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
}