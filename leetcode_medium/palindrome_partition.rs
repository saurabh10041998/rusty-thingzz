pub struct Solver {
    res: Vec<Vec<String>>,
}

impl Solver {
    fn new() -> Self {
        Solver { res: Vec::new() }
    }

    fn solve(&mut self, s: String) {
        let mut part = Vec::new();
        self.solve_helper(0, &s, &mut part);
    }

    fn solve_helper(&mut self, idx: usize, s: &str, part: &mut Vec<String>) {
        if idx >= s.len() {
            self.res.push(part.clone());
            return;
        }
        for k in idx..s.len() {
            if self.is_palindrome(&s[idx..k + 1]) {
                part.push(String::from(&s[idx..k + 1]));
                self.solve_helper(k + 1, s, part);
                part.pop().unwrap();
            }
        }
    }

    fn is_palindrome(&self, s: &str) -> bool {
        let c = s.chars().collect::<Vec<char>>();
        let (mut i, mut j) = (0, (c.len() - 1) as i32);
        while i <= j {
            if c[i as usize] != c[j as usize] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        return true;
    }

    fn get_res(self) -> Vec<Vec<String>> {
        self.res
    }
}

fn partition(s: String) -> Vec<Vec<String>> {
    let mut solver = Solver::new();
    solver.solve(s);
    solver.get_res()
}
#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let s = String::from("aab");
        assert_eq!(
            partition(s),
            vec![
                vec![String::from("a"), String::from("a"), String::from("b")],
                vec![String::from("aa"), String::from("b")]
            ]
        );
    }

    #[test]
    fn run_tc2() {
        let s = String::from("a");
        assert_eq!(partition(s), vec![vec![String::from("a")]]);
    }
}
fn main() {}
