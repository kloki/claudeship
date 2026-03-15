mod git;
mod input;

use braille_bar::braille_bar;
use colorize::AnsiColor;
use git::GitInfo;
use input::Input;

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

    let model_display = format!("[{}]", input.model.display_name).cyan();
    let dir = match std::env::var("HOME") {
        Ok(home) => input.workspace.current_dir.replacen(&home, "~", 1),
        Err(_) => input.workspace.current_dir.clone(),
    };
    let dir_display = dir.magenta();
    let cost_display = format!("${:.2}", input.cost.total_cost_usd).yellow();

    let mut line = match git.as_ref().and_then(|g| g.branch.as_ref()) {
        Some(branch) => {
            let branch_display = format!("[{}]", branch).green();
            format!("{model_display} {dir_display} {branch_display} {cost_display} {colored_bar}")
        }
        None => {
            format!("{model_display} {dir_display} {cost_display} {colored_bar}")
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

    line
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = Input::from_stdin()?;
    let git = GitInfo::from_dir(&input.workspace.current_dir);
    let output = build_output(&input, git);
    print!("{output}");
    Ok(())
}
