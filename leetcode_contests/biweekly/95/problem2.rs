use std::collections::HashMap;
use std::collections::VecDeque;
struct DataStream {
    value: i32,
    k: i32,
    history: HashMap<i32, i32>,
    window: VecDeque<i32>,
}
impl DataStream {
    fn new(value: i32, k: i32) -> Self {
        DataStream {
            value,
            k,
            history: HashMap::new(),
            window: VecDeque::new(),
        }
    }

    fn consec(&mut self, num: i32) -> bool {
        if self.window.len() == self.k as usize {
            let x = self.window.pop_front().unwrap();
            let freq = self.history.get(&x).unwrap();
            if *freq == 1 {
                self.history.remove(&x);
            } else {
                self.history.entry(x).and_modify(|v| *v -= 1);
            }
        }
        self.window.push_back(num);
        self.history.entry(num).and_modify(|v| *v += 1).or_insert(1);
        if self.history.len() > 1 {
            return false;
        } else {
            match self.history.get(&self.value) {
                Some(&v) => {
                    if v == self.k {
                        true
                    } else {
                        false
                    }
                }, None => {
                    false
                }
            }
        }
    }
}
#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let mut ds = DataStream::new(4, 3);
        assert_eq!(ds.consec(4), false);
        assert_eq!(ds.consec(4), false);
        assert_eq!(ds.consec(4), true);
        assert_eq!(ds.consec(3), false);
    }
    #[test]
    fn run_tc2() {
        let mut ds = DataStream::new(4, 3);
        assert_eq!(ds.consec(4), false);
        assert_eq!(ds.consec(4), false);
        assert_eq!(ds.consec(4), true);
        assert_eq!(ds.consec(4), true);
        assert_eq!(ds.consec(4), true);
        assert_eq!(ds.consec(4), true);
        assert_eq!(ds.consec(4), true);
        assert_eq!(true, true);
    }
    #[test]
    fn run_tc3() {
        assert_eq!(true, true);
    }
}
fn main() {
    println!("Hello, world!");
}
