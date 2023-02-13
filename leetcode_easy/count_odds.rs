fn ceil(a: i32, b: i32) -> i32 {
    a / b + (a % b != 0) as i32
}
fn count_odds(low: i32, high: i32) -> i32 {
    ceil(high, 2) - low / 2
}

#[cfg(test)]
pub mod tests {
    use crate::count_odds;
    #[test]
    fn run_tc1() {
        let low = 3;
        let high = 7;
        assert_eq!(count_odds(low, high), 3);
    }
    #[test]
    fn run_tc2() {
        let low = 8;
        let high = 10;
        assert_eq!(count_odds(low, high), 1);
    }
}

fn main() {
    let low = 3;
    let high = 7;
    assert_eq!(count_odds(low, high), 3);
}
