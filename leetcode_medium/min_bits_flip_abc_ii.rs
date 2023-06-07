fn min_flips(a: i32, b: i32, c: i32) -> i32 {
    ((a | b) ^ c).count_ones() as i32 + ((a & b) & ((a | b) ^ c)).count_ones() as i32
}

#[cfg(test)]
pub mod tests {
    use crate::min_flips;
    #[test]
    fn run_tc1() {
        let a = 2;
        let b = 6;
        let c = 5;
        assert_eq!(min_flips(a, b, c), 3);
    }
    #[test]
    fn run_tc2() {
        let a = 4;
        let b = 2;
        let c = 7;
        assert_eq!(min_flips(a, b, c), 1);
    }
}

fn main() {
    let a = 4;
    let b = 2;
    let c = 7;
    assert_eq!(min_flips(a, b, c), 1);
}
