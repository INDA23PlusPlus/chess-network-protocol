# How to use
Remember to install `serde_json` to your Cargo.toml.
Add this repo as a git dependency in the Cargo.toml.
There are two files that can be used as boilerplate. One is an example
demonstrating how to connect to a server and send commands and receive data. 
The other one is an example of receiving and sending data as a server. They are both located in the `examples` folder and can be run with `cargo run --example <name>`. 
They show how the handshake and first move should be made and just prints everything received instead of doing any logic with it.
This repository is a library, so you can use it as a dependency in your own project with:
```toml
[dependencies]
chess-network-protocol = { git = "https://github.com/INDA23PlusPlus/chess-network-protocol" }
```

Some parts of the protocol are optional like you do not have to implement draw or resign and if you do not do any kind of animation or rendering of previous moves the `move_made` field can be ignored. Just make sure you implement the minimal protocol described in the `lib.rs` file.

# How it works
Serde is used to define a JSON schema. We use port 8384 as the default port for historical reasons.
The server is the one that listens on port 8384 and the client is the one that connects to the server.
The server is the one that uses its chess backend for all the chess logik and the client essentially only renders the state of the game and sends the moves to the server.

# Protocol
After a TCP connection is established the protocol is as follows:
1. Client sends the `ClientToServerHandshake` struct
2. Server sends the `ServerToClientHandshake` struct
3. Client sends the `ClientToServer` struct when it is its turn to make a move.
4. Server responds with the `ServerToClient` struct of the `State` variant if the move was legal containing the move of the client in the `move_made` field. If the move was illegal, the server responds with the `ServerToClient` struct of the `Error` variant and we step back to step 3 but the client uses the information in the `Error` variant which ahould be the same as in the previous `State` variant.
5. Server sends the `ServerToClient` struct of the `State` variant when the server has made a move.
6. repeat from step 3.

This is only if the client is white. If the client is black, the server sends the `ServerToClient` struct of the `State` variant first. meaning step 5 would be before step 3.

Draw and resign are optional features that are nice to have so games can be ended without having to play them out or just closing the connection abruptly. They don't have to be implemented but are described in the `lib.rs` file.