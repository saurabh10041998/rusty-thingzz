struct BrowserHistory {
    history: Vec<String>,
    curr: usize,
    end: usize,
}

impl BrowserHistory {
    fn new(homepage: String) -> Self {
        let mut history = vec![String::new(); 5001];
        history[0] = homepage;
        BrowserHistory {
            history,
            curr: 0,
            end: 0,
        }
    }
    fn visit(&mut self, url: String) {
        if self.history[self.end] != url {
            self.curr += 1;
            self.history[self.curr] = url;
            self.end = self.curr;
        }
    }
    fn back(&mut self, steps: i32) -> String {
        self.curr = match self.curr.checked_sub(steps as usize) {
            Some(idx) => idx,
            None => 0,
        };
        self.history[self.curr].clone()
    }
    fn forward(&mut self, steps: i32) -> String {
        self.curr = usize::min(self.end, self.curr + steps as usize);
        self.history[self.curr].clone()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let mut bs = BrowserHistory::new(String::from("leetcode.com"));
        bs.visit(String::from("google.com"));
        bs.visit(String::from("facebook.com"));
        bs.visit(String::from("youtube.com"));
        assert_eq!(bs.back(1), String::from("facebook.com"));
        assert_eq!(bs.back(1), String::from("google.com"));
        assert_eq!(bs.forward(1), String::from("facebook.com"));
        bs.visit(String::from("linkedin.com"));
        assert_eq!(bs.forward(2), String::from("linkedin.com"));
        assert_eq!(bs.back(2), String::from("google.com"));
        assert_eq!(bs.back(7), String::from("leetcode.com"));
    }
}

fn main() {
    let mut bs = BrowserHistory::new(String::from("leetcode.com"));
    bs.visit(String::from("google.com"));
    bs.visit(String::from("facebook.com"));
    bs.visit(String::from("youtube.com"));
    assert_eq!(bs.back(1), String::from("facebook.com"));
    assert_eq!(bs.back(1), String::from("google.com"));
    assert_eq!(bs.forward(1), String::from("facebook.com"));
    bs.visit(String::from("linkedin.com"));
    assert_eq!(bs.forward(2), String::from("linkedin.com"));
    assert_eq!(bs.back(2), String::from("google.com"));
    assert_eq!(bs.back(7), String::from("leetcode.com"));
}
