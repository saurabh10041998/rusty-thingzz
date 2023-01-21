pub struct Solver {
    res: Vec<String>,
}

impl Solver {
    fn new() -> Self {
        Solver { res: Vec::new() }
    }

    fn solve(&mut self, s: String) {
        let mut ip = String::new();
        self.solve_helper(0, 0, &s, &mut ip);
    }

    fn solve_helper(&mut self, idx: usize, dot: usize, s: &str, ip: &mut String) {
        if dot == 4 && idx == s.len() {
            ip.pop().unwrap();
            self.res.push(ip.clone());
            ip.push('.');
            return;
        }
        if dot > 4 {
            return;
        }
        for j in idx..usize::min(idx + 3, s.len()) {
            if s[idx..j + 1].parse::<i32>().unwrap() < 256 && (idx == j || &s[idx..idx + 1] != "0")
            {
                for c in s[idx..j + 1].chars() {
                    ip.push(c);
                }
                ip.push('.');
                self.solve_helper(j + 1, dot + 1, s, ip);
                ip.pop().unwrap(); // Remove dot;
                for _ in s[idx..j + 1].chars() {
                    ip.pop().unwrap();
                }
            }
        }
    }

    fn get_res(self) -> Vec<String> {
        self.res
    }
}

fn restore_ip_addresses(s: String) -> Vec<String> {
    let mut solver = Solver::new();
    solver.solve(s);
    solver.get_res()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let s = String::from("25525511135");
        assert_eq!(
            restore_ip_addresses(s),
            vec![
                String::from("255.255.11.135"),
                String::from("255.255.111.35")
            ]
        );
    }

    #[test]
    fn run_tc2() {
        let s = String::from("0000");
        assert_eq!(restore_ip_addresses(s), vec![String::from("0.0.0.0")]);
    }

    #[test]
    fn run_tc3() {
        let s = String::from("101023");
        assert_eq!(
            restore_ip_addresses(s),
            vec![
                String::from("1.0.10.23"),
                String::from("1.0.102.3"),
                String::from("10.1.0.23"),
                String::from("10.10.2.3"),
                String::from("101.0.2.3")
            ]
        );
    }
}

fn main() {
    let s = String::from("101023");
    assert_eq!(
        restore_ip_addresses(s),
        vec![
            String::from("1.0.10.23"),
            String::from("1.0.102.3"),
            String::from("10.1.0.23"),
            String::from("10.10.2.3"),
            String::from("101.0.2.3")
        ]
    );
}
