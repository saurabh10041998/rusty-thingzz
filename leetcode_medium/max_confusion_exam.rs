fn helper(chars: &[char], replace_char: char, k: i32) -> usize {
    let mut ans = 0;
    let mut left = 0;
    let mut max_allowed = 0;
    for (right, &c) in chars.iter().enumerate() {
        if c == replace_char {
            max_allowed += 1;
            while max_allowed > k {
                if chars[left] == replace_char {
                    max_allowed -= 1;
                }
                left += 1;
            }
        }
        ans = usize::max(ans, right - left + 1);
    }
    ans
}

fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
    let chars = answer_key.chars().collect::<Vec<char>>();
    let ans1 = helper(&chars, 'T', k);
    let ans2 = helper(&chars, 'F', k);
    i32::max(ans1 as i32, ans2 as i32)
}

#[cfg(test)]
pub mod tests {
    use crate::max_consecutive_answers;
    #[test]
    fn run_tc1() {
        let answer_key = String::from("TTFF");
        let k = 2;
        assert_eq!(max_consecutive_answers(answer_key, k), 4);
    }
    #[test]
    fn run_tc2() {
        let answer_key = String::from("TFFT");
        let k = 1;
        assert_eq!(max_consecutive_answers(answer_key, k), 3);
    }
    #[test]
    fn run_tc3() {
        let answer_key = String::from("TTFTTFTT");
        let k = 1;
        assert_eq!(max_consecutive_answers(answer_key, k), 5);
    }
}

fn main() {
    let answer_key = String::from("TTFTTFTT");
    let k = 1;
    assert_eq!(max_consecutive_answers(answer_key, k), 5);
}
