fn is_perfect_square(num: i32) -> bool {
    if num == 1 { 
        return true;
    }
    let (mut low, mut high) = (0, num as i64 / 2 as i64);
    while low <= high {
        let mid = low + (high - low) / 2;
        if mid * mid == i64::from(num) {
            return true;
        } else if mid * mid < i64::from(num) {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    false
}

#[cfg(test)]
pub mod tests {
    use crate::is_perfect_square;
    #[test]
    fn run_tc1() {
        let num = 16;
        assert_eq!(is_perfect_square(num), true);
    }
    #[test]
    fn run_tc2() {
        let num = 14;
        assert_eq!(is_perfect_square(num), false);
    }
    #[test]
    fn run_tc3() {
        let num = 1;
        assert_eq!(is_perfect_square(num), true);
    }
}
fn main() {
    let num = 16;
    assert_eq!(is_perfect_square(num), true);
}
