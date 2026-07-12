mod git;
mod input;

use braille_bar::braille_bar;
use colorize::AnsiColor;
use git::GitInfo;
use input::Input;

fn format_age(ms: u64) -> String {
    let secs = ms / 1000;
    let mins = secs / 60;
    let hours = mins / 60;
    let days = hours / 24;
    if days > 0 {
        format!("{}d{}h", days, hours % 24)
    } else if hours > 0 {
        format!("{}h{}m", hours, mins % 60)
    } else if mins > 0 {
        format!("{}m", mins)
    } else {
        format!("{}s", secs)
    }
}

fn build_output(input: &Input, git: Option<GitInfo>) -> String {
    let used = input.context_window.used_percentage.unwrap_or(0.0);
    let bar = braille_bar(used);

    let colored_bar = if used >= 80.0 {
        bar.red()
    } else if used >= 40.0 {
        bar.yellow()
    } else {
        bar.grey()
    };

    let model_name = input
        .model
        .display_name
        .split_whitespace()
        .next()
        .unwrap_or(&input.model.display_name);
    let effort_glyph = input.effort.as_ref().and_then(|e| match e.level.as_str() {
        "low" => Some('\u{25CB}'),    // ○
        "medium" => Some('\u{25D0}'), // ◐
        "high" => Some('\u{25CF}'),   // ●
        "xhigh" => Some('\u{25C9}'),  // ◉
        "max" => Some('\u{2B24}'),    // ⬤
        _ => None,
    });
    let model_display = match effort_glyph {
        Some(glyph) => format!("[{} {}]", model_name, glyph).cyan(),
        None => format!("[{}]", model_name).cyan(),
    };
    let home = std::env::var("HOME").ok();
    let shorten = |path: &str| match &home {
        Some(home) => path.replacen(home, "~", 1),
        None => path.to_string(),
    };
    let dir_display = shorten(&input.workspace.current_dir).magenta();
    let cost_display = format!("${:.2}", input.cost.total_cost_usd).yellow();
    let age_display = format_age(input.cost.total_duration_ms).grey();

    let worktree_display = if git.as_ref().is_some_and(|g| g.is_worktree) {
        " \u{f1bb}".blue()
    } else {
        String::new()
    };

    let mut line = match git.as_ref().and_then(|g| g.branch.as_ref()) {
        Some(branch) => {
            let branch_display = format!("[{}]", branch).green();
            format!(
                "{model_display} {dir_display} {branch_display}{worktree_display} {cost_display} {age_display} {colored_bar}"
            )
        }
        None => {
            format!("{model_display} {dir_display} {cost_display} {age_display} {colored_bar}")
        }
    };

    if let Some(git) = git {
        let mut parts = Vec::new();
        if git.untracked > 0 {
            parts.push(format!("?{}", git.untracked).cyan());
        }
        if git.modified > 0 {
            parts.push(format!("*{}", git.modified).yellow());
        }
        if git.staged > 0 {
            parts.push(format!("+{}", git.staged).green());
        }
        if git.renamed > 0 {
            parts.push(format!(">{}", git.renamed).magenta());
        }
        if git.deleted > 0 {
            parts.push(format!("!{}", git.deleted).red());
        }
        if !parts.is_empty() {
            line.push_str("\n[Git Status] ");
            line.push_str(&parts.join(" "));
        }
    }

    for dir in &input.workspace.added_dirs {
        line.push_str("\n\u{f114} ");
        line.push_str(&shorten(dir).magenta());
    }

    line
}

#[cfg(test)]
mod tests {
    use super::format_age;

    #[test]
    fn formats_age() {
        assert_eq!(format_age(0), "0s");
        assert_eq!(format_age(45_000), "45s");
        assert_eq!(format_age(60_000), "1m");
        assert_eq!(format_age(12 * 60_000), "12m");
        assert_eq!(format_age(65 * 60_000), "1h5m");
        assert_eq!(format_age(26 * 3_600_000), "1d2h");
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = Input::from_stdin()?;
    let git = GitInfo::from_dir(&input.workspace.current_dir);
    let output = build_output(&input, git);
    print!("{output}");
    Ok(())
}
