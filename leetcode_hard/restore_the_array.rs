use std::collections::HashMap;

const MOD: i64 = i64::pow(10, 9) + 7;

fn helper(s: &Vec<char>, k: i64, idx: usize, dp: &mut HashMap<usize, i32>) -> i32 {
    if idx == s.len() {
        return 1;
    }
    if s[idx] == '0' {
        return 0;
    }
    match dp.get(&idx) {
        Some(val) => return *val,
        None => {}
    };
    let mut ans = 0;
    let mut num = 0i64;
    for j in idx..s.len() {
        num = num * 10 + s[j].to_string().parse::<i64>().unwrap();
        if num > k {
            break;
        }
        ans = (ans + helper(s, k, j + 1, dp) as i64) % MOD;
    }
    dp.entry(idx).or_insert(ans as i32);
    ans as i32
}

fn number_of_arrays(s: String, k: i32) -> i32 {
    let mut dp = HashMap::new();
    let s = s.chars().collect::<Vec<char>>();
    helper(&s, k as i64, 0, &mut dp)
}

#[cfg(test)]
pub mod tests {
    use crate::number_of_arrays;
    #[test]
    fn run_tc1() {
        let s = String::from("1000");
        let k = 10000;
        assert_eq!(number_of_arrays(s, k), 1);
    }

    #[test]
    fn run_tc2() {
        let s = String::from("1000");
        let k = 10;
        assert_eq!(number_of_arrays(s, k), 0);
    }

    #[test]
    fn run_tc3() {
        let s = String::from("1317");
        let k = 2000;
        assert_eq!(number_of_arrays(s, k), 8);
    }

    #[test]
    fn run_tc4() {
        let s = String::from("600342244431311113256628376226052681059918526204");
        let k = 703;
        assert_eq!(number_of_arrays(s, k), 411743991);
    }
}

fn main() {
    let s = String::from("1000");
    let k = 10000;
    assert_eq!(number_of_arrays(s, k), 1);
}
