use std::env;
use std::io::{self, Read};
use std::process::ExitCode;

use cowsay_rust::{CowKind, available_cow_names, random_cow_kind, render_with_kind};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args_os().skip(1);
    let mut width = 40usize;
    let mut eyes = "oo".to_string();
    let mut tongue = String::new();
    let mut no_wrap = false;
    let mut color_mode = false;
    let mut kind = CowKind::Cow;
    let mut message_parts = Vec::new();

    while let Some(arg) = args.next() {
        let value = arg.to_string_lossy();
        match value.as_ref() {
            "-h" | "--help" => {
                print_help();
                return Ok(());
            }
            "-l" | "--list" => {
                print_cow_list();
                return Ok(());
            }
            "-r" | "--random" => {
                kind = random_cow_kind();
            }
            "-C" | "--color" => {
                color_mode = true;
            }
            "-e" | "--eyes" => {
                let next = args.next().ok_or_else(|| "Missing value for --eyes".to_string())?;
                eyes = next.to_string_lossy().to_string();
            }
            "-T" | "--tongue" => {
                let next = args.next().ok_or_else(|| "Missing value for --tongue".to_string())?;
                tongue = next.to_string_lossy().to_string();
            }
            "-W" | "--width" => {
                let next = args.next().ok_or_else(|| "Missing value for --width".to_string())?;
                width = next
                    .to_string_lossy()
                    .parse()
                    .map_err(|_| format!("Invalid width: {}", next.to_string_lossy()))?;
            }
            "-n" | "--no-wrap" => no_wrap = true,
            "-f" | "--cowfile" => {
                let next = args.next().ok_or_else(|| "Missing value for --cowfile".to_string())?;
                kind = parse_cowfile(&next.to_string_lossy());
            }
            "-b" => kind = CowKind::Borg,
            "-d" => kind = CowKind::Dead,
            "-g" => kind = CowKind::Greedy,
            "-p" => kind = CowKind::Paranoid,
            "-s" => kind = CowKind::Stoned,
            "-t" => kind = CowKind::Tired,
            "-w" => kind = CowKind::Wired,
            "-y" => kind = CowKind::Youthful,
            "--" => {
                message_parts.extend(args.map(|item| item.to_string_lossy().to_string()));
                break;
            }
            _ => message_parts.push(value.to_string()),
        }
    }

    if message_parts.is_empty() {
        let mut stdin = String::new();
        io::stdin().read_to_string(&mut stdin).map_err(|err| err.to_string())?;
        if !stdin.trim().is_empty() {
            message_parts.push(stdin.trim_end_matches('\n').to_string());
        }
    }

    let mut message = message_parts.join(" ");
    if message.is_empty() {
        message = "".to_string();
    }

    if no_wrap {
        width = message.chars().count().max(1);
    }

    let rendered = render_with_kind(&message, width, &eyes, &tongue, kind);
    let output = if color_mode {
        colorize(rendered)
    } else {
        rendered
    };
    println!("{output}");
    Ok(())
}

fn colorize(text: String) -> String {
    let mut colored = String::new();
    for (index, ch) in text.chars().enumerate() {
        if ch == '\n' {
            colored.push(ch);
            continue;
        }
        let code = match index % 6 {
            0 => "\u{001b}[31m",
            1 => "\u{001b}[32m",
            2 => "\u{001b}[33m",
            3 => "\u{001b}[34m",
            4 => "\u{001b}[35m",
            _ => "\u{001b}[36m",
        };
        colored.push_str(code);
        colored.push(ch);
        colored.push_str("\u{001b}[0m");
    }
    colored
}

fn parse_cowfile(name: &str) -> CowKind {
    match name.to_ascii_lowercase().as_str() {
        "cow" | "default" => CowKind::Cow,
        "beavis.zen" | "beavis_zen" | "beavis" => CowKind::BeavisZen,
        "stegosaurus" => CowKind::Stegosaurus,
        _ => CowKind::Cow,
    }
}

fn print_help() {
    println!("Usage: cowsay-rust [OPTION]... [MESSAGE]");
    println!();
    println!("  -b, -d, -g, -p, -s, -t, -w, -y  Appearance modes");
    println!("  -C, --color     Enable ANSI color output");
    println!("  -e, --eyes      Select eye characters");
    println!("  -f, --cowfile  Select a built-in cow style or file name");
    println!("  -h, --help      Display this help text");
    println!("  -l, --list      List available cow files");
    println!("  -n, --no-wrap  Disable word wrapping");
    println!("  -r, --random   Pick a random cow style");
    println!("  -T, --tongue   Select tongue character(s)");
    println!("  -W, --width    Set the width of the speech bubble");
    println!();
    println!("Built-in cow styles: {}", available_cow_names().join(", "));
}

fn print_cow_list() {
    for name in available_cow_names() {
        println!("{name}");
    }
}
