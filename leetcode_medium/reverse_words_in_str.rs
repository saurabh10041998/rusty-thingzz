fn reverse_words(s: String) -> String {
    let s = String::from(s.trim());
    let buffer: Vec<_> = s.split_ascii_whitespace().collect();
    println!("{:?}", buffer);
    let buffer = buffer.into_iter().rev().collect::<Vec<&str>>();
    let mut ans = String::new();
    for c in buffer {
        ans = ans + c + " ";
    }
    String::from(ans.trim())
}

#[cfg(test)]
pub mod tests {
    use crate::reverse_words;
    #[test]
    fn run_tc1() {
        let s = String::from("the sky is blue");
        assert_eq!(reverse_words(s), String::from("blue is sky the"));
    }

    #[test]
    fn run_tc2() {
        let s = String::from("  hello world  ");
        assert_eq!(reverse_words(s), String::from("world hello"));
    }

    #[test]
    fn run_tc3() {
        let s = String::from("a good   example");
        assert_eq!(reverse_words(s), String::from("example good a"));
    }
}

fn main() {
    println!("Hello, world!");
    let s = String::from("a good   example");
    assert_eq!(reverse_words(s), String::from("example good a"));
}
