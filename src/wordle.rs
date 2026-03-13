pub fn score(answer: &str, guess: &str) -> [u8; 5] {
    let mut result = [0u8; 5];
    let mut counts = [0u8; 26];

    let a = answer.as_bytes();
    let g = guess.as_bytes();

    for &c in a {
        counts[(c - b'a') as usize] += 1;
    }

    // greens
    for i in 0..5 {
        if g[i] == a[i] {
            result[i] = 2;
            counts[(g[i] - b'a') as usize] -= 1;
        }
    }

    // yellows
    for i in 0..5 {
        if result[i] != 0 {
            continue;
        }

        let idx = (g[i] - b'a') as usize;

        if counts[idx] > 0 {
            result[i] = 1;
            counts[idx] -= 1;
        }
    }

    result
}
