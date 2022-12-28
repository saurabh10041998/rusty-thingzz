
fn is_power_of_two(n: i32) -> bool {
    n != 0 && n as i64 & (n as i64 - 1) ==  0
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let n = 1;
        assert_eq!(is_power_of_two(n), true);
    }
    #[test]
    fn run_tc2() {
        let n = 16;
        assert_eq!(is_power_of_two(n), true);
    }
    #[test]
    fn run_tc3() {
        let n = 3;
        assert_eq!(is_power_of_two(n), false);
    }
    #[test]
    fn run_tc4() {
        let n = 0;
        assert_eq!(is_power_of_two(n), false);
    }
}
fn main() { 
    let n = 1;
    assert_eq!(is_power_of_two(n), true);
}