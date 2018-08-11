#[derive(ToString,Debug)]
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub enum Tag {
        Event, // the name of the tournament or match event.
        Site, // the location of the event. This is in "City, Region COUNTRY" format, where COUNTRY is the three-letter International Olympic Committee code for the country. An example is "New York City, NY USA".
        Date, // the starting date of the game, in YYYY.MM.DD form. "??" are used for unknown values.
        Round, // the playing round ordinal of the game within the event.
        White, // the player of the white pieces, in "last name, first name" format.
        Black, // the player of the black pieces, same format as White.
        Result, // the result of the game. This can only have four possible values: "1-0" (White won), "0-1" (Black won), "1/2-1/2" (Draw), or "*" (other, e.g., the game is ongoing).
        Annotator, // The person providing notes to the game.
        PlyCount, // String value denoting total number of half-moves played.
        TimeControl, // "40/7200:3600" (moves per seconds: sudden death seconds)
        Time, // Time the game started, in "HH:MM:SS" format, in local clock time.
        Termination, // Gives more details about the termination of the game. It may be "abandoned", "adjudication" (result determined by third-party adjudication), "death", "emergency", "normal", "rules infraction", "time forfeit", or "unterminated".
        Mode, // "OTB" (over-the-board) "ICS" (Internet Chess Server)
        FEN, // The initial position of the chess board, in Forsyth-Edwards Notation. This is used to record partial games (starting at some initial position). It is also necessary for chess variants such as Fischer random chess, where the initial position is not always the same as traditional chess.
        SetUp, // If a FEN tag is used, a separate tag pair "SetUp" must also appear and have its value set to "1".
        ECO // ECO classification
}

pub fn mandatory_tag(tag: Tag) -> bool {
    return tag == Tag::Event ||
        tag == Tag::Site ||
        tag == Tag::Date ||
        tag == Tag::Round ||
        tag == Tag::White ||
        tag == Tag::Black ||
        tag == Tag::Result;
}