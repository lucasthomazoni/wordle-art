use crate::wordle::score;

fn tile(v: u8) -> &'static str {
    match v {
        2 => "🟩",
        1 => "🟨",
        _ => "⬛",
    }
}

pub fn print_row(answer: &str, guess: &str) {
    let pattern = score(answer, guess);
    let tiles: String = pattern.iter().map(|&v| tile(v)).collect();
    println!("{}  {}", tiles, guess);
}

pub fn print_art(answer: &str, guesses: &[String]) {
    for g in guesses {
        print_row(answer, g);
    }
}

pub fn print_pattern(p: &[u8; 5]) {
    let tiles: String = p.iter().map(|&v| tile(v)).collect();
    println!("{}", tiles);
}
