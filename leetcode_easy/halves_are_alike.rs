fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        'A' | 'E' | 'I' | 'O' | 'U' => true,
        _ => false,
    }
}

fn halves_are_alike(s: String) -> bool {
    let (mut l, mut r) = (0, 0);
    let n = s.len();
    for (i, c) in s.chars().enumerate() {
        if is_vowel(c) {
            if i < n / 2 {
                l += 1;
            } else {
                r += 1;
            }
        }
    }
    l == r
}

#[cfg(test)]
pub mod tests {
    use crate::halves_are_alike;
    #[test]
    fn run_tc1() {
        let s = String::from("book");
        assert_eq!(halves_are_alike(s), true);
    }
    #[test]
    fn run_tc2() {
        let s = String::from("textbook");
        assert_eq!(halves_are_alike(s), false);
    }
}

fn main() {
    //println!("Hello, world!");
    let s = String::from("book");
    assert_eq!(halves_are_alike(s), true);
}
