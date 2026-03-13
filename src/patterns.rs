pub type Pattern = [u8; 5];

pub fn parse_pattern(s: &str) -> Pattern {
    let mut p = [0u8; 5];
    for (i, c) in s.chars().enumerate() {
        p[i] = match c {
            'G' | 'g' => 2,
            'Y' | 'y' => 1,
            _ => 0,
        };
    }
    p
}

pub fn format_pattern(p: &Pattern) -> String {
    p.iter()
        .map(|&v| match v {
            2 => 'G',
            1 => 'Y',
            _ => 'B',
        })
        .collect()
}

const G: u8 = 2;
const B: u8 = 0;
const Y: u8 = 1;

pub fn get_preset(name: &str) -> Option<Vec<Pattern>> {
    match name {
        "diagonal" => Some(diagonal()),
        "x" => Some(x()),
        "cross" => Some(cross()),
        "bullseye" => Some(bullseye()),
        "chevron-left" | "chevron_left" => Some(chevron_left()),
        "chevron-right" | "chevron_right" => Some(chevron_right()),
        "1" => Some(digit_1()),
        "2" => Some(digit_2()),
        "3" => Some(digit_3()),
        "4" => Some(digit_4()),
        "5" => Some(digit_5()),
        "6" => Some(digit_6()),
        "7" => Some(digit_7()),
        "8" => Some(digit_8()),
        "9" => Some(digit_9()),
        "star" => Some(star()),
        "yin-yang" | "yin_yang" => Some(yin_yang()),
        "question" => Some(question_mark()),
        _ => None,
    }
}

pub fn list_presets() -> &'static [&'static str] {
    &[
        "diagonal",
        "x",
        "cross",
        "bullseye",
        "chevron-left",
        "chevron-right",
        "1", "2", "3", "4", "5", "6", "7", "8", "9",
        "star",
        "yin-yang",
        "question",
    ]
}

pub fn diagonal() -> Vec<Pattern> {
    vec![
        [G, B, B, B, B],
        [B, G, B, B, B],
        [B, B, G, B, B],
        [B, B, B, G, B],
        [B, B, B, B, G],
    ]
}

pub fn x() -> Vec<Pattern> {
    vec![
        [G, B, B, B, G],
        [B, G, B, G, B],
        [B, B, G, B, B],
        [B, G, B, G, B],
        [G, B, B, B, G],
    ]
}

pub fn cross() -> Vec<Pattern> {
    vec![
        [B, B, G, B, B],
        [B, B, G, B, B],
        [Y, G, G, G, Y],
        [B, B, G, B, B],
        [B, B, G, B, B],
    ]
}

pub fn bullseye() -> Vec<Pattern> {
    vec![
        [B, Y, Y, Y, B],
        [Y, B, B, B, Y],
        [Y, B, G, B, Y],
        [Y, B, B, B, Y],
        [B, Y, Y, Y, B],
    ]
}

pub fn chevron_left() -> Vec<Pattern> {
    vec![
        [B, B, B, G, B],
        [B, B, G, B, B],
        [B, G, B, B, B],
        [B, B, G, B, B],
        [B, B, B, G, B],
    ]
}

pub fn chevron_right() -> Vec<Pattern> {
    vec![
        [B, G, B, B, B],
        [B, B, G, B, B],
        [B, B, B, G, B],
        [B, B, G, B, B],
        [B, G, B, B, B],
    ]
}

pub fn digit_1() -> Vec<Pattern> {
    vec![
        [B, B, G, B, B],
        [B, G, G, B, B],
        [B, B, G, B, B],
        [B, B, G, B, B],
        [B, G, G, G, B],
    ]
}

pub fn digit_2() -> Vec<Pattern> {
    vec![
        [B, G, G, G, B],
        [G, B, B, B, G],
        [B, B, G, G, B],
        [B, G, B, B, B],
        [Y, G, G, G, Y],
    ]
}

pub fn digit_3() -> Vec<Pattern> {
    vec![
        [B, G, G, G, B],
        [B, B, B, B, G],
        [B, G, G, G, B],
        [B, B, B, B, G],
        [B, G, G, G, B],
    ]
}

pub fn digit_4() -> Vec<Pattern> {
    vec![
        [G, B, B, G, B],
        [G, B, B, G, B],
        [Y, Y, Y, Y, Y],
        [B, B, B, G, B],
        [B, B, B, G, B],
    ]
}

pub fn digit_5() -> Vec<Pattern> {
    vec![
        [Y, Y, Y, Y, Y],
        [G, B, B, B, B],
        [G, G, G, G, B],
        [B, B, B, B, G],
        [G, G, G, G, B],
    ]
}

pub fn digit_6() -> Vec<Pattern> {
    vec![
        [B, G, G, G, B],
        [G, B, B, B, B],
        [G, G, G, G, B],
        [G, B, B, B, G],
        [B, G, G, G, B],
    ]
}

pub fn digit_7() -> Vec<Pattern> {
    vec![
        [B, G, G, G, B],
        [B, B, B, B, G],
        [B, B, B, G, B],
        [B, B, G, B, B],
        [B, G, B, B, B],
    ]
}

pub fn digit_8() -> Vec<Pattern> {
    vec![
        [B, G, G, G, B],
        [B, G, Y, G, B],
        [B, G, G, G, B],
        [B, G, Y, G, B],
        [B, G, G, G, B],
    ]
}

pub fn digit_9() -> Vec<Pattern> {
    vec![
        [B, G, G, G, B],
        [G, B, B, B, G],
        [B, G, G, G, G],
        [B, B, B, B, G],
        [B, G, G, G, B],
    ]
}

pub fn star() -> Vec<Pattern> {
    vec![
        [B, B, B, B, B],
        [B, G, B, G, B],
        [B, B, G, B, B],
        [B, G, B, G, B],
        [B, B, B, B, B],
    ]
}

pub fn yin_yang() -> Vec<Pattern> {
    vec![
        [B, G, G, G, B],
        [B, B, G, G, G],
        [G, B, B, G, G],
        [G, G, B, B, B],
        [B, G, G, G, B],
    ]
}

pub fn question_mark() -> Vec<Pattern> {
    vec![
        [B, G, G, G, B],
        [G, B, B, B, G],
        [B, B, G, G, B],
        [B, B, G, B, B],
        [B, B, G, B, B],
    ]
}
