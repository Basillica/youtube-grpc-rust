# Working with gRPC with Rust

This tutorial provides a simple and quick implementation of how to get started working with gRPC in Rust

## Requirements
To follow this tutorial, the following are requireed
1. protobuf installed locally
2. tonic -  a simple rust implementation of gRPC
3. prost -  for generating rust code from .proto files

## To run this project
To run this project, simple clone and run the two binaries in seperate terminals
```bash
cargo run --bin grpc-server
cargo run --bin grpc-client
```

Happy hacking

## Documentation
1. Tonic - docs.rs/tonic/0.11.0
2. Prost - docs.rs/prost/0.12.3
3. Protobuf releases - https://github.com/protocolbuffers/protobuf/releases