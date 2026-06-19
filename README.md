# 🔍 hexviewer

A lightweight command-line hex viewer written in Rust. Displays binary file contents in classic hex dump format with optional ASCII output export.

---

## 📸 Preview

```
00000000  7f 45 4c 46 02 01 01 00  00 00 00 00 00 00 00 00  .ELF............
00000010  02 00 3e 00 01 00 00 00  60 10 40 00 00 00 00 00  ..>.....`.@.....
00000020  40 00 00 00 00 00 00 00  c0 36 00 00 00 00 00 00  @........6......
```

---

## 📁 File Tree

```
hexviewer/
├── Cargo.toml
├── Cargo.lock
├── Makefile
└── src/
    ├── main.rs
    └── process.rs
```

---

## ⚙️ Prerequisites

### 1. Install Rust (rustc & cargo)

The recommended way is via [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then follow the on-screen instructions. After installation, verify:

```bash
rustc --version
cargo --version
```

> **Windows users:** Download and run the installer from [https://rustup.rs](https://rustup.rs).

---

### 2. Install Make

**Linux (Debian/Ubuntu):**
```bash
sudo apt update && sudo apt install make
```

**Linux (Arch):**
```bash
sudo pacman -S make
```

**macOS:**
```bash
xcode-select --install
```

**Windows:** Install via [Chocolatey](https://chocolatey.org/):
```bash
choco install make
```

Or use [WSL](https://learn.microsoft.com/en-us/windows/wsl/) for a native Linux environment.

---

## 🚀 Installation

Clone the repository and enter the project directory:

```bash
git clone https://github.com/zipherle/hexviewer.git
cd hexviewer
```

---

## 🛠️ Usage with Makefile

### Build (release mode)

Compiles the project with full optimizations:

```bash
make build
```

Output binary: `target/release/hexeditor`

---

### Run

Runs the compiled release binary:

```bash
make run
```

> ⚠️ You'll need to pass arguments directly. See [CLI Usage](#-cli-usage) below for running with arguments.

---

### Debug

Compiles and runs in debug mode (includes debug symbols, faster compile time):

```bash
make debug
```

Output binary: `target/debug/hexeditor`

---

### Clean

Removes all build artifacts:

```bash
make clean
```

---

## 💻 CLI Usage

After building, run the binary directly:

```bash
# View hex dump of a file
./target/release/hexeditor <filename>

# View hex dump and export ASCII characters to an output file
./target/release/hexeditor <filename> <output_ascii_file>
```

### Examples

```bash
# Hex dump of a binary
./target/release/hexeditor /bin/ls

# Hex dump and save ASCII output
./target/release/hexeditor secret.bin output.txt
```

---

## 📄 License

This project is open source. Feel free to use and modify it.

---

## 👤 Author

**Zipher Le**
