use std::collections::HashSet;
pub struct PowerGenerator {
    powers: HashSet<usize>,
    base: usize,
}
impl PowerGenerator {
    fn new(base: usize) -> Self {
        PowerGenerator {
            powers: HashSet::new(),
            base,
        }
    }
    fn gen_power_upto(&mut self, max_limit: usize) {
        self.powers.clear();
        self.powers.insert(1);
        let mut start = 1;
        while start <= max_limit {
            self.powers.insert(start);
            start = start * self.base;
        }
    }
}
fn is_power_of_three(n: i32) -> bool {
    let mut pw = PowerGenerator::new(3);
    pw.gen_power_upto(i32::MAX as usize);
    let n = n as usize;
    pw.powers.contains(&n)
}

#[cfg(test)]
pub mod tests {
    use crate::is_power_of_three;
    #[test]
    fn run_tc1() {
        let n = 27;
        assert_eq!(is_power_of_three(n), true);
    }
    #[test]
    fn run_tc2() {
        let n = 0;
        assert_eq!(is_power_of_three(n), false);
    }
    #[test]
    fn run_tc3() {
        let n = -1;
        assert_eq!(is_power_of_three(n), false);
    }
}

fn main() {
    let n = 27;
    assert_eq!(is_power_of_three(n), true);
}
