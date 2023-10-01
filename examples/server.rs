use serde::{Serialize, Deserialize};
use chess_network_protocol::*;

use std::io::prelude::*;
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8384")?;

    // accept connections and process them serially
    let (stream, _addr) = listener.accept()?;

    //receive
    let mut de = serde_json::Deserializer::from_reader(&stream);
    let deserialized = ClientToServerHandshake::deserialize(&mut de)?;
    println!("Received: {:?}", deserialized);

    let handshake = ServerToClientHandshake {
        features: vec![
            Features::EnPassant, 
            Features::Castling, 
            Features::Promotion
            ],
        board: [[Piece::BlackBishop; 8]; 8],
        moves: vec![Move {
            start_x: 0,
            start_y: 0,
            end_x: 1,
            end_y: 1,
            promotion: Piece::None,
        }],
        joever: Joever::Ongoing,
    };

    //send
    serde_json::to_writer(&stream, &handshake).unwrap();

    //assumes that the client is white
    //receive
    let mut de = serde_json::Deserializer::from_reader(&stream);
    let deserialized = ClientToServer::deserialize(&mut de)?;
    println!("Received: {:?}", deserialized);

    let state = ServerToClient::State {
        board: [[Piece::BlackBishop; 8]; 8],
        moves: vec![Move {
            start_x: 0,
            start_y: 0,
            end_x: 0,
            end_y: 0,
            promotion: Piece::None,
        }],
        joever: Joever::Ongoing,
        //should be the move recieved from the client
        move_made: Move {
            start_x: 0,
            start_y: 0,
            end_x: 0,
            end_y: 0,
            promotion: Piece::None,
        }
    };
    
    //send
    serde_json::to_writer(&stream, &state).unwrap();

    let state = ServerToClient::State {
        board: [[Piece::BlackBishop; 8]; 8],
        moves: vec![Move {
            start_x: 0,
            start_y: 0,
            end_x: 0,
            end_y: 0,
            promotion: Piece::None,
        }],
        joever: Joever::Ongoing,
        //should be the move made by the server
        move_made: Move {
            start_x: 0,
            start_y: 0,
            end_x: 0,
            end_y: 0,
            promotion: Piece::None,
        }
    };

    //send
    serde_json::to_writer(&stream, &state).unwrap();
    Ok(())
}