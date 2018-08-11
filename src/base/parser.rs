use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::char;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::str::FromStr;
use base::tag::*;

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

type Int = u64;

pub struct ChessParserIterator {
    file_reader: BufReader<File>,
    buf: String,
    moves: Vec<String>,
    curr_move: String,
    status: Status,
    last_char: char,
    not_parsed: String,
    resultFromMoves: String,
    tags: HashMap<String,String>, // TODO use a linked hash map?
    end_parse: bool,
    variations: HashMap<Int,Vec<String>>,
    after_variations_comments: HashMap<Int,HashMap<Int,String>>,
    comments: HashMap<Int,String>,
    tag_key: Option<String>,
    tag_value: Option<String>,
    reason: GameResultReason,
    result_from_tag: String,
}

enum GameResultReason {
    abandoned,
    adjudication,
    death,
    emergency,
    normal,
    rules_infraction,
    time_forfait,
    undeterminated
}

fn result_from_pgn(s: String) -> Result<GameResultReason, ()> {
    match s.as_ref() {
        "abandoned" => Ok(GameResultReason::abandoned),
        "adjudication" => Ok(GameResultReason::adjudication),
        "death" => Ok(GameResultReason::death),
        "emergency" => Ok(GameResultReason::emergency),
        "normal" => Ok(GameResultReason::normal),
        "rules infraction" => Ok(GameResultReason::rules_infraction),
        "time forfait" => Ok(GameResultReason::time_forfait),
        "undeterminated" => Ok(GameResultReason::undeterminated),
        _ => Err(()),
    }
}


impl ChessParserIterator {

    pub fn new(file_reader: BufReader<File>) -> Self {
        return ChessParserIterator{file_reader: file_reader, buf: String::new(), moves: Vec::new(), 
            curr_move: String::new(), status: Status::headings, last_char: char::from_digit(0, 10).unwrap(),
            not_parsed: String::new(), resultFromMoves: String::new(), tags: HashMap::new(), end_parse: false,
            variations: HashMap::new(), after_variations_comments: HashMap::new(), comments: HashMap::new(),
            tag_key: None, tag_value: None, reason: GameResultReason::normal, result_from_tag: "".to_string()};
    }

    fn get_game(&mut self) -> (bool, Option<ChessGame>) {
        // no char has been parsed, it's not a game
        if self.status == Status::ready {
            return (false, None);
        }
        // it can happens if there's no result, but it's wrong, since PGN format says it's mandatory. 
        // However it may happen in variations
        if self.status == Status::move_ && self.not_parsed.len() > 0 {
            self.moves.push(self.not_parsed.clone());
            // TODO check if I need a new instance
            self.not_parsed.clear();
            self.status = Status::moves;
        // the file is ended just after the result
        } else if (self.status == Status::gameResult && self.not_parsed.len() > 0) {
            self.resultFromMoves = self.not_parsed.clone();
            // TODO check if I need a new instance
            self.not_parsed.clear();
            self.status = Status::moves;
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
        return (false, Some(ChessGame{moves: self.moves.clone()}))
    }

    fn parse_comments(&mut self, c: char) {
        if c == '}' {
            let last_move_index = self.moves.last_index();
            let variations = self.variations.get(&last_move_index);
            if variations.is_some() && variations.unwrap().len() > 0 {
                let avMoveComments = match self.after_variations_comments.entry(last_move_index) {
                    Entry::Occupied(o) => o.into_mut(),
                    Entry::Vacant(v) => {
                        v.insert(HashMap::new())
                    }
                };
                avMoveComments.insert(self.variations[&last_move_index].last_index(), self.not_parsed.trim_right().to_string());
            } else {
                println!("{}", self.not_parsed.trim_right().to_string());
                self.comments.insert(last_move_index, self.not_parsed.trim_right().to_string());
            }
            self.not_parsed.clear();
            self.status = Status::moves;
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
            self.status = Status::headings;
            self.tag_key = None;
            self.tag_value = None;
            self.not_parsed.clear();
        } else if c == '"' {
            self.tag_key = Some(self.not_parsed.clone());
            self.status = Status::headingValue;
            self.not_parsed.clear();
        } else if c.is_whitespace() {
        } else {
            self.not_parsed += &c.to_string();
        }
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
        headings,
        heading,
        headingValue,
        moves,
        variation,
        comment,
        moveUnknown, // can be a move number or a game result  
        moveNumber, 
        gameResult,
        move_,
        numericAnnotationGlyph,
        ready // no char has been parsed
    }

impl Iterator for ChessParserIterator {
    type Item = ChessGame;

    fn next(&mut self) -> Option<ChessGame> {
        if self.end_parse {
            return None;
        }

        loop {
            let count = self.file_reader.read_line(&mut self.buf);
            
            if count.unwrap() <= 0 {
                if self.moves.is_empty() {
                    return None;
                } else {
                    // TODO empty vector
                    return Some(ChessGame{moves: Vec::new()})
                }
            } else {
                    let line = self.buf.to_string();

                    for c in line.chars() {
                        self.last_char = c;

                        if self.status == Status::ready {
                            if !c.is_whitespace() {
                                self.status = Status::headings;
                            } else {
                                continue;
                            }
                        }

                        if c == '\n' {
                            if self.last_char == '\n' {
                                // it's the new line after headings, so now there's moves
                                if self.status == Status::headings {
                                    self.status = Status::moves;
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

                        if self.status == Status::comment {
                            self.parse_comments(c);
                            continue;
                        }

                        if self.status == Status::headings {
                            if c == '[' {
                                self.not_parsed.clear();
                                self.status = Status::heading;
                                continue;
                            } else if c.is_whitespace() {
                                continue;
                            } else {
                                self.status = Status::moves;
                            }
                        }

                        if self.status == Status::heading {
                            self.parse_heading(c);
                        }

                        // ...


                        if self.status == Status::moves {
                            if c == '{' {
                                self.status = Status::comment;
                            } else if c == '(' {
                                self.status = Status::variation;
                            } else if c == '$' {
                                self.status = Status::numericAnnotationGlyph;
                            } else if c == '*' {
                                self.status = Status::gameResult;
                                self.not_parsed += &c.to_string();
                            } else if c.is_digit(10) {
                                self.status = Status::moveUnknown;
                                self.not_parsed += &c.to_string();
                            } else if c.is_whitespace() {

                            } else {
                                self.status = Status::move_;
                                self.not_parsed += &c.to_string();
                            }
                            continue;
                        }

                    //println!("{}", line);
                }
                self.buf.clear();
                return Some(ChessGame{moves: self.moves.clone()});
            }
        }
    }

}

pub struct ChessGame {
    pub moves: Vec<String>
}