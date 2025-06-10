# gRPCService 🚀

A simple Rust project demonstrating how to build and consume a gRPC API using [Tonic](https://docs.rs/tonic/) and Protocol Buffers.

This project includes:
- ✅ A gRPC server that implements a basic `SayHello` service.
- ✅ A gRPC client that connects to the server and sends a request.
- ✅ Code generation from `.proto` files using `build.rs`.

---

## 📦 Project Structure

gRPCService/
├── Cargo.toml
├── build.rs
├── proto/
│ └── helloworld.proto
├── src/
│ ├── main.rs # gRPC Server
│ └── client.rs # gRPC Client



---

## 🔧 Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- Protocol Buffers Compiler (`protoc`)

Install `protoc` (Ubuntu/Debian):

```bash
sudo apt update
sudo apt install -y protobuf-compiler

---

## ⚙️ Build the Project

cargo build

---

## 🚀 Run the Server
cargo run


# Server will listen on:
[::1]:50051

## 💬 Run the Client
In another terminal:

cargo run --bin client

RESPONSE="Hello, itz_klasic!"

---

## 📚 Concepts Covered

- ✅ Writing .proto definitions

- ✅ Using tonic-build to generate Rust gRPC code

- ✅ Creating gRPC services and clients in Rust

- ✅ gRPC communication using HTTP/2 under the hood

---

## 🛠 Dependencies Used
Check Cargo.toml 

---

## 📝 License
This project is licensed under the MIT License.

---

# 🙌 Credits
Built with ❤️ by itz_klasic while learning backend fundamentals using Rust and gRPC.


