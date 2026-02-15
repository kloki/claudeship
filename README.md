# claudeship

A custom statusline for [Claude Code](https://claude.ai/code).

Displays model name, working directory, git branch, session cost, context window usage bar, and git status.

## Install

```sh
cargo install claudeship
```

## Configure

Add to `~/.claude/settings.json`:

```json
{
  "statusLine": {
    "type": "command",
    "command": "claudeship"
  }
}
```

## Output

```
[Opus] /home/user/project [main] $0.42 ████████░░░░░░░░░░░░
?3 !2 +1
```

Line 1: model, directory, git branch, cost, context window usage (red at >=80%)

Line 2 (git status, non-zero only): `?` untracked, `!` modified, `+` staged, `>` renamed, `x` deleted
