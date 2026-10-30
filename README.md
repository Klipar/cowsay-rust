# cowsay-rust

A Rust port of the classic `cowsay` program with the same core behavior: a speech bubble, wrapping text, and a default cow drawing.

## Usage

```bash
cargo run -- "hello world"
cargo run -- -e xx "hello"
cargo run -- -W 20 "a longer message"
cargo run -- -f beavis.zen "hello"
```

## Supported options

- `-e`, `--eyes` — custom eye characters
- `-T`, `--tongue` — custom tongue characters
- `-W`, `--width` — speech bubble width
- `-n`, `--no-wrap` — disable automatic wrapping
- `-f`, `--cowfile` — built-in style selection (`cow`, `beavis.zen`, `stegosaurus`)
- `-b`, `-d`, `-g`, `-p`, `-s`, `-t`, `-w`, `-y` — preset modes
- `-h`, `--help` — help output

## Notes

This implementation preserves the classic `cowsay` look and CLI ergonomics while keeping the code small and maintainable in Rust.
