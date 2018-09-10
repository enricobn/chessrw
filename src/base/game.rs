use std::collections::HashMap;

use base::fen::*;
use base::position::*;
use base::tag::*;

type Int = i16;

pub trait ChessGame {

    fn get_tags(&self) -> &HashMap<String,String>;

    fn get_moves(&self) -> &Vec<String>;

    fn get_before_moves_comment(&self) -> Option<&String>;

    /** 
     * # Arguments
     * * `after_move` is zero based.
     */
    fn get_comment(&self, after_move: Int) -> Option<&String>;

    fn get_game_result(&self) -> &String;

    fn initial_position(&self) -> Result<ChessPosition,String>;

    /** 
     * # Arguments
     * * `after_move` is zero based.
     */
    fn get_nags(&self, after_move: Int) -> Option<&Vec<String>>;

    /** 
     * # Arguments
     * * `after_move` is zero based.
     */
    fn get_variations(&self, after_move: Int) -> Option<&Vec<String>>;

    /** 
     * # Arguments
     * * `after_move` is zero based.
     * * `after_variation_move` is zero based
     */
    fn get_after_variation_comment(&self, after_move: Int, after_variation_move: Int) -> Option<&String>;
}

pub struct ChessGameImpl {
    tags: HashMap<String,String>,
    moves: Vec<String>,
    comments: HashMap<Int,String>,
    variations: HashMap<Int,Vec<String>>,
    after_variations_comments: HashMap<Int,HashMap<Int,String>>,
    game_result: String,
    nags: HashMap<Int,Vec<String>>,
}

impl ChessGameImpl {

    pub fn new(tags: HashMap<String,String>,
           moves: Vec<String>,
           comments: HashMap<Int,String>,
           variations: HashMap<Int,Vec<String>>,
           after_variations_comments: HashMap<Int,HashMap<Int,String>>,
           game_result: String,
           nags: HashMap<Int,Vec<String>>) -> ChessGameImpl {
        ChessGameImpl{tags: tags, moves: moves, comments: comments, variations: variations, 
            after_variations_comments: after_variations_comments, game_result: game_result, nags: nags}
    }

}

impl ChessGame for ChessGameImpl {

    fn get_tags(&self) -> &HashMap<String,String> {
        &self.tags
    }

    fn get_moves(&self) -> &Vec<String> {
        &self.moves
    }

    fn get_before_moves_comment(&self) -> Option<&String> {
        self.comments.get(&-1)
    }

    /** 
     * # Arguments
     * * `after_move` is zero based.
     */
    fn get_comment(&self, after_move: Int) -> Option<&String> {
        self.comments.get(&after_move)
    }

    fn get_game_result(&self) -> &String {
        &self.game_result
    }

    fn initial_position(&self) -> Result<ChessPosition,String> {
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
    fn get_nags(&self, after_move: Int) -> Option<&Vec<String>> {
        self.nags.get(&after_move)
    }

    /** 
     * # Arguments
     * * `after_move` is zero based.
     */
    fn get_variations(&self, after_move: Int) -> Option<&Vec<String>> {
        self.variations.get(&after_move)
    }

    /** 
     * # Arguments
     * * `after_move` is zero based.
     * * `after_variation_move` is zero based
     */
    fn get_after_variation_comment(&self, after_move: Int, after_variation_move: Int) -> Option<&String> {
        match self.after_variations_comments.get(&after_move) {
            Some(avc) => avc.get(&after_variation_move),
            _ => None
        }
    }
}
