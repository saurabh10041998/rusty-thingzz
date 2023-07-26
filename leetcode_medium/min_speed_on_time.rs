fn check(dist: &Vec<i32>, hour: f64, speed: usize) -> bool {
    let mut time = 0.0;
    for d in dist {
        time = f64::ceil(time);
        time += (*d as f64) / speed as f64;
    }
    time <= hour
}

fn min_speed_on_time(dist: Vec<i32>, hour: f64) -> i32 {
    if dist.len() > f64::ceil(hour) as usize {
        return -1;
    }
    let mut low = 1;
    let mut high = usize::pow(10, 7);
    while low < high {
        let offset = match high.checked_sub(low) {
            Some(offset) => offset / 2,
            None => break,
        };
        let mid = low + offset;
        if check(&dist, hour, mid) {
            high = mid
        } else {
            low = mid + 1;
        }
    }
    low as i32
}

#[cfg(test)]
pub mod tests {
    use crate::min_speed_on_time;
    #[test]
    fn run_tc1() {
        let dist = vec![1, 3, 2];
        let hour = 6;
        assert_eq!(min_speed_on_time(dist, hour as f64), 1);
    }
    #[test]
    fn run_tc2() {
        let dist = vec![1, 3, 2];
        let hour = 2.7;
        assert_eq!(min_speed_on_time(dist, hour as f64), 3);
    }
    #[test]
    fn run_tc3() {
        let dist = vec![1, 3, 2];
        let hour = 1.9;
        assert_eq!(min_speed_on_time(dist, hour as f64), -1);
    }
}

fn main() {
    let dist = vec![1, 3, 2];
    let hour = 6;
    assert_eq!(min_speed_on_time(dist, hour as f64), 1);
}
