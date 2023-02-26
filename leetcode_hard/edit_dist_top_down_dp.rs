use std::collections::HashMap;

const NOOP: i32 = 0;

fn min_distance_helper(
    s1: &Vec<char>,
    s2: &Vec<char>,
    i: usize,
    j: usize,
    dp: &mut HashMap<(usize, usize), i32>,
) -> i32 {
    if i == 0 {
        return j as i32;
    }
    if j == 0 {
        return i as i32;
    }
    match dp.get(&(i, j)) {
        Some(val) => return *val,
        None => {}
    };
    let ans = match s1[i.checked_sub(1).unwrap()] == s2[j.checked_sub(1).unwrap()] {
        true => NOOP + min_distance_helper(s1, s2, i - 1, j - 1, dp),
        false => i32::min(
            1 + min_distance_helper(s1, s2, i - 1, j - 1, dp),
            i32::min(
                1 + min_distance_helper(s1, s2, i - 1, j, dp),
                1 + min_distance_helper(s1, s2, i, j - 1, dp),
            ),
        ),
    };
    dp.entry((i, j)).or_insert(ans);
    ans
}

fn min_distance(word1: String, word2: String) -> i32 {
    let str1 = word1.chars().collect::<Vec<char>>();
    let str2 = word2.chars().collect::<Vec<char>>();
    let mut dp = HashMap::new();
    let n = str1.len();
    let m = str2.len();
    min_distance_helper(&str1, &str2, n, m, &mut dp)
}

#[cfg(test)]
pub mod tests {
    use crate::min_distance;
    #[test]
    fn run_tc1() {
        let str1 = String::from("horse");
        let str2 = String::from("ros");
        assert_eq!(min_distance(str1, str2), 3);
    }
    #[test]
    fn run_tc2() {
        let str1 = String::from("intention");
        let str2 = String::from("execution");
        assert_eq!(min_distance(str1, str2), 5);
    }
}

fn main() {
    let str1 = String::from("horse");
    let str2 = String::from("ros");
    assert_eq!(min_distance(str1, str2), 3);
}
