fn total_trips_in_time(time: &Vec<i32>, total_time: i64) -> i64 {
    let mut trips = 0i64;
    for t in time {
        trips += total_time / *t as i64;
    }
    trips
}
fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
    let mut low = 1i64;
    let mut high = time.iter().min().copied().unwrap() as i64 * total_trips as i64;
    while low < high {
        let mid = low + (high - low) / 2;
        if total_trips_in_time(&time, mid) < total_trips as i64 {
            low = mid + 1;
        } else {
            high = mid
        }
    }
    low
}

#[cfg(test)]
pub mod tests {
    use crate::minimum_time;
    #[test]
    fn run_tc1() {
        let time = vec![1, 2, 3];
        let total_trips = 5;
        assert_eq!(minimum_time(time, total_trips), 3);
    }
    #[test]
    fn run_tc2() {
        let time = vec![2];
        let total_trips = 1;
        assert_eq!(minimum_time(time, total_trips), 2);
    }
}

fn main() {
    let time = vec![1, 2, 3];
    let total_trips = 5;
    assert_eq!(minimum_time(time, total_trips), 3);
}
