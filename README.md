# How to use
Remember to install `serde_json` to your Cargo.toml.
Add this repo as a git dependency in the Cargo.toml.
There are two files that can be used as boilerplate. One is an example
demonstrating how to connect to a server and send commands and receive data. 
The other one is an example of receiving and sending data as a server. They are both located in the `examples` folder and can be run with `cargo run --example <name>`.
This repository is a library, so you can use it as a dependency in your own project with:
```toml
[dependencies]
chess-network-protocol = { git = "https://github.com/INDA23PlusPlus/chess-network-protocol" }
```

# How it works
Serde is used to define a JSON schema. We use port 8384 as the default port for historical reasons.
The server is authorative. When a tcp connection is established, the server sends the ServerToClient struct to the client. The client then responds with the ClientToServer struct(This means client will always be white should add a handshake variant to ClientToServer so the interaction starts with the client sending to server). The client then waits for the server to send the ServerToClient struct indicating the next move. And then it repeats. If the server at any point thinks that a move is invalid, it will send an identical massage back to the client until the client makes a legal move 
