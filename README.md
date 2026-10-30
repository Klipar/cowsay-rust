# cowsay-rust

A Rust implementation of the classic `cowsay` utility. The program prints text inside a speech bubble and displays a cow or another styled template beneath it.

The project aims to stay as close as possible to the original `cowsay`: it supports the main flags, line wrapping, different cow styles, and bundled cowfile templates so it can run without depending on the system-installed `cowsay`.

## What is this?

`cowsay` is a small terminal utility that displays text as speech from a character, usually a cow. It is often used for:

- displaying short messages in the console
- showing information in a styled speech bubble
- demonstrating different cow patterns and mascot art
- showcasing text wrapping and formatting behavior

This project provides a Rust version designed to closely match the original tool in the main usage scenarios.

## Requirements

- Rust toolchain (latest stable version recommended)
- Git

## Clone the repository

```bash
git clone <repository-url>
cd cowsay-rust
```

## Run without installing

```bash
cargo run -- "hello from rust"
```

### More example commands

```bash
cargo run -- -e xx "hello"
cargo run -- -W 20 "a longer message"
cargo run -- -f beavis.zen "hello"
cargo run -- -l
cargo run -- -n "hello   world"
```

## Install after cloning

To install the binary into the Cargo directory (`$HOME/.cargo/bin`) and run it from anywhere:

```bash
cargo install --path .
```

After that, you can run it like this:

```bash
cowsay-rust --help
cowsay-rust "hello"
cowsay-rust -f dragon "hello"
```

If the `cowsay-rust` command is not found, make sure the Cargo bin directory is in your `PATH`:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

## Supported options

- `-e`, `--eyes` — custom eye characters
- `-T`, `--tongue` — custom tongue characters
- `-W`, `--width` — speech bubble width
- `-n`, `--no-wrap` — disable automatic wrapping
- `-f`, `--cowfile` — choose a cow style or custom cowfile
- `-b`, `-d`, `-g`, `-p`, `-s`, `-t`, `-w`, `-y` — preset appearance modes
- `-l`, `--list` — list available styles
- `-r`, `--random` — random cow style
- `-h`, `--help` — show help

## License

The project preserves the appropriate GPL license for bundled cowfile resources and graphic templates so the `cowsay` analog can be used as a standalone tool without depending on the system utility.

## Development

```bash
cargo build
cargo test
```

## Summary

After cloning the repository, you can:

1. run the program with `cargo run`,
2. install it with `cargo install --path .`,
3. use it as a local `cowsay` alternative in the terminal.
