use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ServerToClient {
    pub board: [[Piece; 8]; 8],
    pub moves: Vec<Move>,
    pub joever: Joever,
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

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Joever {
    White,
    Black,
    Tie,
    Indeterminate,
    Ongoing,
}

///sent from client to server
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub start_x: usize,
    pub start_y: usize,
    pub end_x: usize,
    pub end_y: usize,
    pub promotion: Piece,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClientToServer {
    Move(Move),
    ///if the client wants to resign send this to the server
    Resign,
}