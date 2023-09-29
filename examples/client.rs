use serde::{Serialize, Deserialize};
use chess_network_protocol::*;

use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let state_test = ClientToServer::Move(Move { 
        start_x: 0, 
        start_y: 0, 
        end_x: 1, 
        end_y: 1, 
        promotion: Piece::None, 
    });

    let stream = TcpStream::connect("127.0.0.1:5000")?;

    //receive
    let mut de = serde_json::Deserializer::from_reader(&stream);
    let deserialized = ServerToClient::deserialize(&mut de)?;

    println!("Recieved: {:?}", deserialized);

    //send
    serde_json::to_writer(&stream, &state_test).unwrap();
    Ok(())
}