fn is_capital(x: char) -> bool {
    match x {
        'A'..='Z' => true,
        _ => false,
    }
}
fn detect_capital_use(word: String) -> bool {
    let mut cap = 0;
    for w in word.chars() {
        if is_capital(w) {
            cap += 1;
        }
    }
    if cap == word.len() || cap == 0 {
        return true;
    }
    cap == 1 && is_capital(*&word[0..1].chars().nth(0).unwrap())
}

#[cfg(test)]
pub mod tests {
    use crate::detect_capital_use;
    #[test]
    fn run_tc1() {
        let word = String::from("USA");
        assert_eq!(detect_capital_use(word), true);
    }
    #[test]
    fn run_tc2() {
        let word = String::from("FlaG");
        assert_eq!(detect_capital_use(word), false);
    }
}

fn main() {
    assert_eq!(detect_capital_use(String::from("Hello worLD")), false);
}
