fn is_vowel(ch: char) -> bool {
    match ch {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}
fn max_vowels(s: String, k: i32) -> i32 {
    let chars = s.chars().collect::<Vec<char>>();
    let count = chars
        .iter()
        .take(k as usize)
        .filter(|&ch| is_vowel(*ch))
        .count();
    chars
        .iter()
        .skip(k as usize)
        .zip(chars.iter())
        .fold((count, count), |ans, x| {
            let (max, cnt) = ans;
            let (curr, k_dist_behind) = x;
            match (is_vowel(*curr), is_vowel(*k_dist_behind)) {
                (false, true) => (max, cnt - 1),
                (true, false) => (usize::max(max, cnt + 1), cnt + 1),
                _ => (max, cnt),
            }
        })
        .0 as i32
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
    max_vowels(s, k);
}
