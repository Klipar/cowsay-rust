# CowSay-Rust

We all know that __one of the most important utilities__ in the Linux world is `cowsay`. This unassuming little program is actually installed on countless machines.

However, so far, I haven't been able to find a secure version of it rewritten in **Rust**. This means it could potentially be unsafe and contain critical security vulnerabilities.

So, to address this gap and get some hands-on experience with modern `vibe-coding` tools, this repository was created.

It is a clone of the [cowsay](https://en.wikipedia.org/wiki/Cowsay) program [github](https://github.com/cowsay-org/cowsay?tab=readme-ov-file).

``` bash
$ cargo run -- "hello"
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/cowsay-rust hello`
 _______
< hello >
 -------
        \   ^__^
         \  (oo)\_______
            (__)\       )\/\
                ||----w |
                ||     ||
```

# How to use (if you want to make your `cowsay` experience safe :D)

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
