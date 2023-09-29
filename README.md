# How to use
Remember to install serde_json to your Cargo.toml.
Add this repo as a git dependency in the Cargo.toml.
Then use the following boilerplate code to establish a connection and 
start receiving commands:
```rust
...
```

To become server, use this boilerplate instead:
```rust
...
```

# How it works
Serde is used to define a JSON schema. We use port 8384 as the default port for historical reasons.
The server is authorative. The client always asks the server to perform a move on its behalf. The server will then
always respond with the resulting state.
