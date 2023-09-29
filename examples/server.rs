use serde::{Serialize, Deserialize};
use chess_network_protocol::*;

use std::io::prelude::*;
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8384")?;

    // accept connections and process them serially
    let (stream, _addr) = listener.accept()?;

    let state = ServerToClient {
        board: [[Piece::BlackBishop; 8]; 8],
        moves: vec![Move {
            start_x: 0,
            start_y: 0,
            end_x: 0,
            end_y: 0,
            promotion: Piece::None,
        }],
        joever: Joever::Ongoing,
    };
    
    //send
    serde_json::to_writer(&stream, &state).unwrap();

    //receive
    let mut de = serde_json::Deserializer::from_reader(&stream);
    let deserialized = ClientToServer::deserialize(&mut de)?;

    println!("Recieved: {:?}", deserialized);


    Ok(())
}