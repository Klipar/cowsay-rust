use std::collections::BTreeSet;
use std::env;
use std::fs;

mod bundled_cows;

use bundled_cows::BUNDLED_COWS;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CowKind {
    Cow,
    BeavisZen,
    Stegosaurus,
    Borg,
    Dead,
    Greedy,
    Paranoid,
    Stoned,
    Tired,
    Wired,
    Youthful,
    File(String),
}

pub fn available_cow_names() -> Vec<String> {
    let mut names = BTreeSet::new();

    for (name, _) in BUNDLED_COWS {
        names.insert((*name).to_string());
    }

    for name in [
        "default",
        "beavis.zen",
        "stegosaurus",
        "borg",
        "dead",
        "greedy",
        "paranoid",
        "stoned",
        "tired",
        "wired",
        "young",
    ] {
        names.insert(name.to_string());
    }

    for dir in cowfile_search_dirs() {
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };

        for entry in entries.flatten() {
            let path = entry.path();
            let Some(file_name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };

            if !file_name.ends_with(".cow") {
                continue;
            }

            let name = file_name.strip_suffix(".cow").unwrap_or(file_name);
            let normalized = name.replace('_', ".");
            let cleaned = normalized.trim();
            if !cleaned.is_empty() {
                names.insert(cleaned.to_string());
            }
        }
    }

    let mut ordered = names.into_iter().collect::<Vec<_>>();
    ordered.sort();
    ordered
}

pub fn random_cow_kind() -> CowKind {
    let names = available_cow_names();
    if names.is_empty() {
        return CowKind::Cow;
    }

    let pick = &names[rand::random::<usize>() % names.len()];
    if pick == "default" {
        CowKind::Cow
    } else if pick == "beavis.zen" {
        CowKind::BeavisZen
    } else if pick == "stegosaurus" {
        CowKind::Stegosaurus
    } else {
        CowKind::File(pick.clone())
    }
}

fn cowfile_search_dirs() -> Vec<String> {
    let mut dirs = Vec::new();

    let bundled = format!("{}/assets/cows", env!("CARGO_MANIFEST_DIR"));
    if fs::metadata(&bundled).is_ok() {
        dirs.push(bundled);
    }

    for candidate in [
        "/usr/share/cowsay/cows",
        "/usr/share/cowsay/site-cows",
        "/usr/local/share/cowsay/cows",
        "/usr/local/share/cowsay/site-cows",
    ] {
        if fs::metadata(candidate).is_ok() {
            dirs.push(candidate.to_string());
        }
    }

    if let Ok(value) = env::var("COWPATH") {
        for part in value.split(':') {
            if !part.is_empty() && fs::metadata(part).is_ok() {
                dirs.push(part.to_string());
            }
        }
    }

    dirs
}

fn resolve_cowfile_path(name: &str) -> Option<String> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return None;
    }

    let path = std::path::Path::new(trimmed);
    if path.is_absolute() && path.exists() {
        return Some(path.to_string_lossy().to_string());
    }

    let bare = trimmed.strip_suffix(".cow").unwrap_or(trimmed);
    let candidates = [
        trimmed.to_string(),
        format!("{trimmed}.cow"),
        bare.to_string(),
        format!("{bare}.cow"),
    ];

    for candidate in candidates {
        if std::path::Path::new(&candidate).exists() {
            return Some(candidate);
        }
    }

    for dir in cowfile_search_dirs() {
        for candidate in [
            format!("{dir}/{trimmed}"),
            format!("{dir}/{trimmed}.cow"),
            format!("{dir}/{bare}"),
            format!("{dir}/{bare}.cow"),
        ] {
            if std::path::Path::new(&candidate).exists() {
                return Some(candidate);
            }
        }
    }

    None
}

fn extract_cow_template(contents: &str) -> Option<String> {
    let mut in_template = false;
    let mut marker: Option<String> = None;
    let mut lines = Vec::new();

    for line in contents.lines() {
        if !in_template {
            if !line.contains("$the_cow") {
                continue;
            }

            let after = line.split("$the_cow").nth(1).unwrap_or("");
            let marker_part = after
                .find("<<")
                .map(|pos| &after[pos + 2..])
                .unwrap_or("");

            if let Some(stripped) = marker_part.strip_prefix('"') {
                if let Some(end) = stripped.find('"') {
                    marker = Some(stripped[..end].to_string());
                }
            } else {
                let normalized = marker_part.trim_end_matches(';');
                if !normalized.trim().is_empty() {
                    marker = Some(normalized.to_string());
                }
            }

            in_template = marker.is_some();
            continue;
        }

        if marker.as_ref().is_some_and(|m| line.contains(m)) {
            break;
        }

        lines.push(line.to_string());
    }

    if lines.is_empty() {
        None
    } else {
        Some(lines.join("\n"))
    }
}

fn bundled_cow_template(name: &str) -> Option<&'static str> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return None;
    }

    let bare = trimmed.strip_suffix(".cow").unwrap_or(trimmed);
    let normalized = bare.replace('_', ".");
    let candidates = [trimmed, bare, normalized.as_str()];

    BUNDLED_COWS
        .iter()
        .find(|(key, _)| candidates.iter().any(|candidate| *key == *candidate))
        .map(|(_, template)| *template)
}

fn render_cowfile_art(name: &str, eyes: &str, tongue: &str) -> Option<String> {
    if let Some(raw) = bundled_cow_template(name) {
        let template = extract_cow_template(raw).unwrap_or(raw.to_string());
        let rendered = template
            .replace("$thoughts", "\\")
            .replace("$eyes", eyes)
            .replace("$tongue", tongue)
            .replace("\\\\", "\\");
        return Some(rendered);
    }

    let path = resolve_cowfile_path(name)?;
    let content = fs::read_to_string(path).ok()?;
    let template = extract_cow_template(&content)?;

    let rendered = template
        .replace("$thoughts", "\\")
        .replace("$eyes", eyes)
        .replace("$tongue", tongue)
        .replace("\\\\", "\\");

    Some(rendered)
}

pub fn wrap_text(message: &str, width: usize) -> Vec<String> {
    let trimmed = message.trim();
    if trimmed.is_empty() {
        return vec![String::new()];
    }

    if width == 0 {
        return vec![trimmed.to_string()];
    }

    let mut lines = Vec::new();
    let mut current = String::new();
    let split_width = if width <= 1 { 1 } else { width - 1 };

    for word in trimmed.split_whitespace() {
        if word.chars().count() > width {
            if !current.is_empty() {
                lines.push(current.clone());
                current.clear();
            }

            let mut chunk = String::new();
            for ch in word.chars() {
                if chunk.chars().count() + 1 > split_width {
                    lines.push(chunk.clone());
                    chunk.clear();
                }
                chunk.push(ch);
            }
            if !chunk.is_empty() {
                lines.push(chunk);
            }
            continue;
        }

        let candidate = if current.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current, word)
        };

        if candidate.chars().count() <= width {
            current = candidate;
        } else {
            lines.push(current.clone());
            current = word.to_string();
        }
    }

    if !current.is_empty() {
        lines.push(current);
    }

    if lines.is_empty() {
        vec![String::new()]
    } else {
        lines
    }
}

pub fn render_cowsay(message: &str, width: usize, eyes: &str, tongue: &str) -> String {
    render_with_kind(message, width, eyes, tongue, CowKind::Cow)
}

pub fn render_with_kind(
    message: &str,
    width: usize,
    eyes: &str,
    tongue: &str,
    kind: CowKind,
) -> String {
    let wrapped = wrap_text(message, width);
    let max_len = wrapped.iter().map(|line| line.chars().count()).max().unwrap_or(0);

    if wrapped.len() == 1 {
        let top = "_".repeat(max_len + 2);
        let bottom = "-".repeat(max_len + 2);
        let bubble = format!("< {} >", wrapped[0]);
        let art = cow_art(kind, eyes, tongue);
        return format!(" {top}\n{bubble}\n {bottom}\n{art}");
    }

    let border_width = max_len + 2;
    let top = "_".repeat(border_width);
    let bottom = "-".repeat(border_width);
    let bubble = wrapped
        .iter()
        .enumerate()
        .map(|(idx, line)| {
            if idx == 0 {
                format!("/ {:<width$} \\", line, width = max_len)
            } else if idx == wrapped.len() - 1 {
                format!("\\ {:<width$} /", line, width = max_len)
            } else {
                format!("| {:<width$} |", line, width = max_len)
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    let art = cow_art(kind, eyes, tongue);
    format!(" {top}\n{bubble}\n {bottom}\n{art}")
}

fn cow_art(kind: CowKind, eyes: &str, tongue: &str) -> String {
    if let CowKind::File(name) = &kind {
        if let Some(custom) = render_cowfile_art(name, eyes, tongue) {
            return custom;
        }
    }

    let effective_eyes = match kind {
        CowKind::Cow => eyes,
        CowKind::BeavisZen => "xx",
        CowKind::Stegosaurus => "oo",
        CowKind::Borg => "==",
        CowKind::Dead => "xx",
        CowKind::Greedy => "$$",
        CowKind::Paranoid => "@@",
        CowKind::Stoned => "**",
        CowKind::Tired => "--",
        CowKind::Wired => "OO",
        CowKind::Youthful => "..",
        CowKind::File(_) => unreachable!(),
    };

    let effective_tongue = if tongue.trim().is_empty() { "" } else { tongue };

    match kind {
        CowKind::BeavisZen => {
            String::from("          __     __\n         /  \\___/  \\\n         \\  (o o)  /\n          /|  .-.  \\\n          \\|  |_|  |/\n              / \\\n             /   \\\n             \\___/\n")
        }
        CowKind::Stegosaurus => String::from("       _______________                 _______________\n     _/               \\_           _/               \\_\n    /  _   ___   ___   \\         /  _   ___   ___   \\\n   /  / \\ / _ \\ / _ \\  \\\n      \\_/ \\___/ \\___/  \\_\n"),
        _ => {
            let tongue_line = if effective_tongue.is_empty() {
                "                ||----w |".to_string()
            } else {
                format!("             {effective_tongue} ||----w |")
            };

            format!(
                "        \\   ^__^\n         \\  ({effective_eyes})\\_______\n            (__)\\       )\\/\\\n{tongue_line}\n                ||     ||"
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrap_text_respects_width() {
        let wrapped = wrap_text("hello there friend", 8);
        assert_eq!(wrapped, vec!["hello", "there", "friend"]);
    }

    #[test]
    fn render_cowsay_contains_bubble_and_cow() {
        let rendered = render_cowsay("hello", 40, "oo", " ");
        assert!(rendered.contains("hello"));
        assert!(rendered.contains("^__^"));
        assert!(rendered.contains("||"));
    }

    #[test]
    fn wrap_text_handles_unicode_words() {
        let wrapped = wrap_text("віііііііііііііііііііііі", 7);
        assert!(!wrapped.is_empty());
        assert!(wrapped.iter().all(|line| line.chars().count() <= 7));
    }

    #[test]
    fn cow_catalog_contains_real_names() {
        let names = available_cow_names();
        assert!(names.iter().any(|name| name == "default" || name == "beavis.zen"));
    }

    #[test]
    fn random_cow_kind_is_valid() {
        let kind = random_cow_kind();
        assert!(matches!(
            kind,
            CowKind::Cow
                | CowKind::BeavisZen
                | CowKind::Stegosaurus
                | CowKind::Borg
                | CowKind::Dead
                | CowKind::Greedy
                | CowKind::Paranoid
                | CowKind::Stoned
                | CowKind::Tired
                | CowKind::Wired
                | CowKind::Youthful
                | CowKind::File(_)
        ));
    }

    #[test]
    fn custom_cow_file_renders_from_system_template() {
        let rendered = render_with_kind("hello", 40, "oo", " ", CowKind::File("dragon".to_string()));
        assert!(rendered.contains("/ \\"));
    }

    #[test]
    fn built_in_tongue_is_inserted_before_the_muzzle() {
        let rendered = render_with_kind("hello", 40, "oo", "..", CowKind::Cow);
        assert!(rendered.contains(".. ||----w |"));
        assert!(!rendered.contains("||..----w |"));
    }
}
