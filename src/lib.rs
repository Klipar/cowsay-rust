#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    };

    let effective_tongue = if tongue.trim().is_empty() { "" } else { tongue };

    match kind {
        CowKind::BeavisZen => {
            String::from("          __     __\n         /  \\___/  \\\n         \\  (o o)  /\n          /|  .-.  \\\n          \\|  |_|  |/\n              / \\\n             /   \\\n             \\___/\n")
        }
        CowKind::Stegosaurus => String::from("       _______________                 _______________\n     _/               \\_           _/               \\_\n    /  _   ___   ___   \\         /  _   ___   ___   \\\n   /  / \\ / _ \\ / _ \\  \\\n      \\_/ \\___/ \\___/  \\_\n"),
        _ => {
            let tongue_segment = if effective_tongue.is_empty() {
                String::new()
            } else {
                effective_tongue.to_string()
            };

            format!(
                "        \\   ^__^\n         \\  ({effective_eyes})\\_______\n            (__)\\       )\\/\\\n                ||{tongue_segment}----w |\n                ||     ||"
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
}
