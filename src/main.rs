mod wordle;
mod solver;
mod patterns;
mod render;
mod discover;

use clap::Parser;

use discover::{build_index, discover_patterns};
use patterns::{format_pattern, get_preset, parse_pattern};
use render::{print_art, print_pattern};
use solver::build_art;

const GUESSES: &str = include_str!("../words/guesses.txt");
const ANSWERS: &str = include_str!("../words/answers.txt");

#[derive(Parser)]
#[command(about = "Generate Wordle tile art for a given answer word")]
struct Cli {
    /// The 5-letter answer word
    answer: String,

    /// Discover all achievable patterns for the answer
    #[arg(long)]
    discover: bool,

    /// Custom pattern rows (e.g. GBBBG BGBGB). G=green, Y=yellow, B=black.
    /// The final all-green row for the answer is added automatically.
    #[arg(short, long)]
    pattern: Vec<String>,

    /// Use a named preset pattern
    #[arg(long, default_value = "diagonal", value_parser = clap::builder::PossibleValuesParser::new(patterns::list_presets()))]
    preset: String,
}

fn parse_words(text: &str) -> Vec<String> {
    text.lines()
        .map(|s| s.trim().to_lowercase())
        .filter(|s| s.len() == 5)
        .collect()
}

fn load_all_words() -> Vec<String> {
    let mut words = parse_words(GUESSES);
    words.extend(parse_words(ANSWERS));
    words.sort();
    words.dedup();
    words
}

fn main() {
    let cli = Cli::parse();
    let answer = cli.answer.to_lowercase();

    if answer.len() != 5 || !answer.chars().all(|c| c.is_ascii_lowercase()) {
        eprintln!("Error: answer must be exactly 5 lowercase letters");
        std::process::exit(1);
    }

    let all_words = load_all_words();

    if cli.discover {
        let index = build_index(&answer, &all_words);
        let found = discover_patterns(&index);
        println!("Achievable patterns for \"{}\" ({} total):\n", answer, found.len());
        for (p, word) in &found {
            print_pattern(p);
            println!("  {} ({})\n", format_pattern(p), word);
        }
        return;
    }

    // Custom patterns override preset
    let target_patterns = if cli.pattern.is_empty() {
        get_preset(&cli.preset).expect("invalid preset (clap should have caught this)")
    } else {
        cli.pattern.iter().map(|s| parse_pattern(s)).collect()
    };

    match build_art(&answer, &target_patterns, &all_words) {
        Ok(art_guesses) => print_art(&answer, &art_guesses),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
