# Flux Systems Language Compiler

Flux is a minimalist systems programming language compiler written in Rust. It parses custom source code syntax and outputs raw x86 16-bit real-mode machine binaries, allowing you to execute code natively on bare-metal hardware or emulators (like QEMU) without an underlying operating system.

## 📁 Repository Map
* **`src/main.rs`** - The core Rust engine. It parses the custom tokens (like double-quoted strings) and handles the structural code generation.
* **`Cargo.toml`** - Manages the Rust dependencies and optimized system build profiles.
* **`main.fx`** - A sample Flux source script showcasing bare-metal text generation.
* **`build.bat`** - A development automation script that links the compiler output to NASM and spins up QEMU.

## 🛠️ Developer Toolchain Requirements
To work with or modify this codebase, your local environment needs:
1. **Rust Toolchain** (`cargo`, `rustc`)
2. **NASM** (Netwide Assembler, added to system PATH or using absolute path mapping)
3. **QEMU** (`qemu-system-x86_64` machine emulator)

## 🚀 Building & Running From Source
Other developers can pull this repository and launch it locally by running:

```bash
# 1. Compile the Rust project into an optimized release binary
cargo build --release

# 2. Run the automation pipeline to compile, assemble, and emulate
# (Ensure your compiled 'flux.exe' tool is placed next to your scripts)
./build.bat
