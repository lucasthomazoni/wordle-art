# wordle-art

A CLI tool that generates Wordle tile art for a given 5-letter answer word. It finds real words that produce specific colored tile patterns when scored against the answer using standard Wordle rules, then displays the result as emoji art.

## Install

```bash
cargo install --path .
```

## Usage

```bash
wordle-art <ANSWER> [OPTIONS]
```

### Examples

```bash
# Default diagonal pattern
wordle-art crane

# Use a preset pattern
wordle-art stare --preset bullseye

# Custom pattern rows (G=green, Y=yellow, B=black)
wordle-art crane -p GBBBG -p BGBGB -p BBBBB

# Discover all achievable patterns for a word
wordle-art crane --discover
```

### Output

```
⬛🟩🟩🟩⬛  krans
🟩⬛⬛⬛🟩  coste
⬛⬛🟩🟩⬛  plans
⬛⬛🟩⬛⬛  topoi
⬛⬛🟩⬛⬛  small
🟩🟩🟩🟩🟩  crane
```

## Options

| Flag | Description |
|---|---|
| `--preset <NAME>` | Use a named preset pattern (default: `diagonal`) |
| `-p, --pattern <PATTERN>` | Custom pattern row(s). The final all-green row is added automatically |
| `--discover` | List all achievable patterns for the given answer |
| `-h, --help` | Print help |

## Presets

| Name | Description |
|---|---|
| `diagonal` | Diagonal line from top-left to bottom-right |
| `x` | X shape |
| `cross` | Plus/cross shape |
| `bullseye` | Concentric square (yellow border) |
| `chevron-left` | Left-pointing chevron |
| `chevron-right` | Right-pointing chevron |
| `star` | Star shape |
| `yin-yang` | Yin-yang symbol |
| `?` / `question` | Question mark |
| `1` - `9` | Digit shapes |

## How it works

1. Both word lists (`words/answers.txt` and `words/guesses.txt`) are embedded into the binary at compile time.
2. For each row in the selected pattern, the tool finds all words that produce that exact tile pattern when scored against the answer using Wordle rules (greens placed first, then yellows).
3. A random matching word is picked for each row, so every run gives different results.
4. The answer itself is appended as the final all-green row.
5. If a pattern row can't be matched by any word, an error is shown indicating which row failed.
