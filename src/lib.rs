use serde::{Serialize, Deserialize};

///This is the first message that the client sends to the server just tells it what color it wants the server to play as.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ClientToServerHandshake {
    pub server_color: Color,
}

/**
This is the first message that the server sends to the client after receiving the `ClientToServerHandshake`.
Includes the start state of the game and what features the server supports.
*/
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ServerToClientHandshake {
    ///The start state of the game.
    pub board: [[Piece; 8]; 8],
    pub moves: Vec<Move>,
    pub joever: Joever,
    ///The features that the server supports. Completely optional to handle and the client can just ignore it if it wants to.
    pub features: Vec<Features>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

/**
the server sends this to the client to tell it what features it supports. 
This has no other function than communicating to the client player what the rules of the game are.
So this is completely optional to handle and the client can just ignore it if it wants to.
*/
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Features {
    ///if the server supports en passant
    EnPassant,
    ///if the server supports castling
    Castling,
    ///if the server supports pawn promotion
    Promotion,
    ///if the server is able to detect a stalemate
    Stalemate,
    ///if the server is able to generate all possible moves
    PossibleMoveGeneration,
    ///if the server supports some other feature that is not listed here that they think might be useful to communicate to the player.
    Other(String),
}

/**
The `State` and `Error` variants are the only that have to be implemented on both server and client side for everyone. Draw and Resigned are optional to implement just nice to have.
*/
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ServerToClient {
    ///The server sends this to the client to tell it what the current state of the game is after every server move and after every successful client move.
    State {
        board: [[Piece; 8]; 8],
        ///All possible moves that the client can make.
        moves: Vec<Move>,
        joever: Joever,
        ///The move that the server made.
        move_made: Move,
    },
    /**
    The server sends this to the client if the client sent an invalid move.
    The state apart from the error message should be the same as the state before the client sent the move. 
    The error is just to make it clear to the client player that the move was invalid and why.
    */
    Error {
        board: [[Piece; 8]; 8],
        ///All possible moves that the client can make.
        moves: Vec<Move>,
        joever: Joever,
        ///The error message that the server wants to send to the client.
        message: String,
    },
    ///The server sends this to the client if the server player resigned the game.
    Resigned {
        board: [[Piece; 8]; 8],
        ///Should be the color of the server.
        joever: Joever,
    },
    ///The server sends this to the client if the server player offered a draw or accepted a draw offer from the client.
    Draw {
        board: [[Piece; 8]; 8],
        moves: Vec<Move>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    #[serde(rename = "p")]
    BlackPawn,
    #[serde(rename = "n")]
    BlackKnight,
    #[serde(rename = "b")]
    BlackBishop,
    #[serde(rename = "r")]
    BlackRook,
    #[serde(rename = "q")]
    BlackQueen,
    #[serde(rename = "k")]
    BlackKing,
    #[serde(rename = "P")]
    WhitePawn,
    #[serde(rename = "N")]
    WhiteKnight,
    #[serde(rename = "B")]
    WhiteBishop,
    #[serde(rename = "R")]
    WhiteRook,
    #[serde(rename = "Q")]
    WhiteQueen,
    #[serde(rename = "K")]
    WhiteKing,
    #[serde(rename = "")]
    None,
}

///joever is the current state of the game
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Joever {
    ///if the game is over and the winner is white
    White,
    ///if the game is over and the winner is black
    Black,
    ///if the game is a draw. This includes stalemate, threefold repetition, the fifty-move rule and insufficient material.
    Draw,
    /**
    if the game is over but the winner is unknown for example if the server backend does not support complete endgame detection. 
    Clients should implement handling of this but will probably rarely be used since most servers wont use it.
    */
    Indeterminate,
    ///if the game is still ongoing
    Ongoing,
}

/**
Represents a move. Used both for possible moves and for the moves made by the players. 
Castling is represented as a king move to the square the king ends up on. 
The client should be able to figure out by itself if a possible move sent from the server is a promotion and then include the promotion piece in the move.
*/
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    ///the file of the piece that is moved where file a is `start_x = 0` and file h is `start_x = 7`.
    pub start_x: usize,
    ///the rank of the piece that is moved where rank 1 is `start_y = 0` and rank 8 is `start_y = 7`.
    pub start_y: usize,
    ///the file of the square the piece is moved to where file a is `end_x = 0` and file h is `end_x = 7`.
    pub end_x: usize,
    ///the rank of the square the piece is moved to where rank 1 is `end_y = 0` and rank 8 is `end_y = 7`.
    pub end_y: usize,
    ///the piece that is promoted to if the move is a promotion. If the move is not a promotion this is `Piece::None`.
    pub promotion: Piece,
}

///Move is the only variant that has to be implemented on both server and client side for everyone. Draw and Resigned are optional to implement just nice to have.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClientToServer {
    ///if the client wants to make a move send this to the server
    Move(Move),
    ///if the client wants to resign send this to the server
    Resign,
    ///if the client wants to offer a draw or accept a draw request the server has already sent send this to the server
    Draw,
}