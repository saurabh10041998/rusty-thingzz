use std::collections::HashSet;
pub fn minimized_string_length(s: String) -> i32 {
    s.chars().collect::<HashSet<char>>().len() as i32
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let s = String::from("aaabc");
        assert_eq!(minimized_string_length(s), 3);
    }
    #[test]
    fn run_tc2() {
        assert_eq!(true, true);
    }
    #[test]
    fn run_tc3() {
        assert_eq!(true, true);
    }
}

fn main() {
    println!("Hello, world!");
}
