# Flux Systems Language Compiler

Flux is a minimalist systems programming language compiler written in Rust. It is designed to parse source code syntax and compile it directly into raw x86 16-bit real-mode machine binaries for custom operating system development, completely bypassing conventional operating system layers.

## Project Structure
* `src/main.rs`: The core Rust compiler engine that parses double-quoted strings and translates them into x86 assembly lines.
* `main.fx`: A sample program demonstrating bare-metal string compilation.
* `build.bat`: A localized automation script to compile the source, assemble it via NASM, and spin up the QEMU emulator sandbox.

## How to Run Locally

1. Clone this repository to your machine.
2. Build the compiler executable using the Rust toolchain:
   ```bash
   cargo build --release
