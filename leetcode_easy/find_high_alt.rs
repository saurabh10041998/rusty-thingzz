fn largest_altitude(gain: Vec<i32>) -> i32 {
    gain.into_iter()
        .fold((0, 0), |t, x| (i32::max(t.0, t.1 + x), t.1 + x))
        .0
}

#[cfg(test)]
pub mod tests {
    use crate::largest_altitude;
    #[test]
    fn run_tc1() {
        let gain = vec![-5, 1, 5, 0, -7];
        assert_eq!(largest_altitude(gain), 1);
    }
    #[test]
    fn run_tc2() {
        let gain = vec![-4, -3, -2, -1, 4, 3, 2];
        assert_eq!(largest_altitude(gain), 0);
    }
}

fn main() {
    let gain = vec![-4, -3, -2, -1, 4, 3, 2];
    assert_eq!(largest_altitude(gain), 0);
}
