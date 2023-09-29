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
The server is authorative. The client always asks the server to perform a move on its behalf. The server will then
always respond with the resulting state.
