fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}
fn max_vowels(s: String, k: i32) -> i32 {
    let mut max = 0;
    let mut cnt = 0;
    let chars = s.chars().collect::<Vec<char>>();
    let n = chars.len();
    for i in 0..n {
        if i >= k as usize && is_vowel(chars[i - k as usize]) {
            cnt -= 1;
        }
        if is_vowel(chars[i]) {
            cnt += 1;
        }
        max = i32::max(cnt, max);
    }
    max
}

#[cfg(test)]
pub mod tests {
    use crate::max_vowels;
    #[test]
    fn run_tc1() {
        let s = String::from("abciiidef");
        let k = 3;
        assert_eq!(max_vowels(s, k), 3);
    }
    #[test]
    fn run_tc2() {
        let s = String::from("aeiou");
        let k = 2;
        assert_eq!(max_vowels(s, k), 2);
    }
    #[test]
    fn run_tc3() {
        let s = String::from("leetcode");
        let k = 3;
        assert_eq!(max_vowels(s, k), 2);
    }
}

fn main() {
    let s = String::from("abciiidef");
    let k = 3;
    assert_eq!(max_vowels(s, k), 3);
}
