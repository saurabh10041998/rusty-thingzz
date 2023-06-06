fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
    arr.sort();
    arr.windows(3).all(|w| w[2] - w[1] == w[1] - w[0])
}

#[cfg(test)]
pub mod tests {
    use crate::can_make_arithmetic_progression;
    #[test]
    fn run_tc1() {
        let arr = vec![3, 5, 1];
        assert_eq!(can_make_arithmetic_progression(arr), true);
    }
    #[test]
    fn run_tc2() {
        let arr = vec![1, 2, 4];
        assert_eq!(can_make_arithmetic_progression(arr), false);
    }
}

fn main() {
    let arr = vec![3, 5, 1];
    assert_eq!(can_make_arithmetic_progression(arr), true);
}
