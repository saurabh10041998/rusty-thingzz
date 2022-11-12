fn is_palindrome(s: String) -> bool {
    let mut new_str: String =  String::new();
    for c in s.chars() {
        if is_alphanum(&c) {
            new_str.push(c.to_ascii_lowercase());
        }
    }
    new_str == new_str.chars().rev().collect::<String>()
}

fn is_alphanum(c: &char) -> bool {
    match c {
        'a'..='z' => true,
        'A'..='Z' => true,
        '0'..='9' => true,
        _ => false,
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{is_palindrome, is_alphanum};
    #[test]
    fn run_tc1() {
        let s = String::from("A man, a plan, a canal: Panama");
        assert_eq!(is_palindrome(s), true);
    }

    #[test]
    fn run_tc2() {
        let s = String::from("race a car");
        assert_eq!(is_palindrome(s), false);
    }

    #[test]
    fn run_tc3() {
        let s = String::from(" ");
        assert_eq!(is_palindrome(s), true);
    }

    #[test]
    fn test_alphanum() {
        let c = 'A';
        assert_eq!(is_alphanum(&c), true);
    }
}

fn main() {
    println!("Hello, world!");
    let s = String::from("A man, a plan, a canal: Panama");
    assert_eq!(is_palindrome(s), true);
}
