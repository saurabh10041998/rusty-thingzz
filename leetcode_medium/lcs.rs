use std::collections::HashMap;

pub struct Solver {
    dp: HashMap<(i32, i32), i32>,
}

impl Solver {
    fn new() -> Self {
        Solver { dp: HashMap::new() }
    }

    fn solve(&mut self, text1: String, text2: String) -> i32 {
        let n = text1.len();
        let m = text2.len();
        self.solve_inner(&text1, &text2, (n - 1) as i32, (m - 1) as i32)
    }

    fn solve_inner(&mut self, text1: &String, text2: &String, n: i32, m: i32) -> i32 {
        if n < 0 || m < 0 {
            return 0;
        }
        match self.dp.get(&(n, m)) {
            Some(&v) => {
                return v;
            }
            None => {}
        }
        if &text1[n as usize..(n + 1) as usize] == &text2[m as usize..(m + 1) as usize] {
            let ans = 1 + self.solve_inner(text1, text2, n - 1, m - 1);
            self.dp.insert((n, m), ans);
            return ans;
        } else {
            let ans = i32::max(
                self.solve_inner(text1, text2, n - 1, m),
                self.solve_inner(text1, text2, n, m - 1),
            );
            self.dp.insert((n, m), ans);
            return ans;
        }
    }
}

fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let mut solver = Solver::new();
    solver.solve(text1, text2)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let text1 = String::from("abcde");
        let text2 = String::from("ace");
        assert_eq!(longest_common_subsequence(text1, text2), 3);
    }

    #[test]
    fn run_tc2() {
        let text1 = String::from("abc");
        let text2 = String::from("abc");
        assert_eq!(longest_common_subsequence(text1, text2), 3);
    }

    #[test]
    fn run_tc3() {
        let text1 = String::from("abc");
        let text2 = String::from("def");
        assert_eq!(longest_common_subsequence(text1, text2), 0);
    }
}

fn main() {
    println!("Hello, world!");
    let text1 = String::from("abcde");
    let text2 = String::from("ace");
    assert_eq!(longest_common_subsequence(text1, text2), 3);
}
