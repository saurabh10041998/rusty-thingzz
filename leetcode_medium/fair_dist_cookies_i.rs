struct Backtracker {
    temp: Vec<i32>,
    ans: i32,
}

impl Backtracker {
    fn new(k: usize) -> Self {
        let temp = vec![0; k];
        Backtracker {
            temp,
            ans: i32::MAX,
        }
    }

    fn get_ans(&self) -> i32 {
        self.ans
    }

    fn solve(&mut self, cookie_idx: usize, cookies: &[i32], k: usize) {
        if cookie_idx == cookies.len() {
            let mut maxi = i32::MIN;
            for &c in self.temp.iter() {
                maxi = i32::max(c, maxi);
            }
            self.ans = i32::min(self.ans, maxi);
            return;
        }
        for i in 0..k {
            self.temp[i] += cookies[cookie_idx];
            self.solve(cookie_idx + 1, cookies, k);
            self.temp[i] -= cookies[cookie_idx];
        }
    }
}

fn distribute_cookies(cookies: Vec<i32>, k: i32) -> i32 {
    let mut bk = Backtracker::new(k as usize);
    bk.solve(0, &cookies, k as usize);
    bk.get_ans()
}

#[cfg(test)]
pub mod tests {
    use crate::distribute_cookies;
    #[test]
    fn run_tc1() {
        let cookies = vec![8, 15, 10, 20, 8];
        let k = 2;
        assert_eq!(distribute_cookies(cookies, k), 31);
    }
    #[test]
    fn run_tc2() {
        let cookies = vec![6, 1, 3, 2, 2, 4, 1, 2];
        let k = 3;
        assert_eq!(distribute_cookies(cookies, k), 7);
    }
}

fn main() {
    let cookies = vec![6, 1, 3, 2, 2, 4, 1, 2];
    let k = 3;
    assert_eq!(distribute_cookies(cookies, k), 7);
}
