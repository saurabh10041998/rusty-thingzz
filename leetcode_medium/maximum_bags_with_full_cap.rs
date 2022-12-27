fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, additional_rocks: i32) -> i32 {
    let mut remaining = vec![0; capacity.len()];
    for i in 0..(capacity.len()) {
        remaining[i] = capacity[i] - rocks[i];
    }
    remaining.sort();
    let (mut count, mut additional_rocks) = (0, additional_rocks);
    for i in 0..capacity.len() {
        if additional_rocks >= remaining[i] {
            additional_rocks -= remaining[i];
            count += 1;
        } else {
            break;
        }
    }
    count
}

#[cfg(test)]
pub mod tests {
    use crate::maximum_bags;
    #[test]
    fn run_tc1() {
        let (capacity, rocks, additional_rocks) = (vec![2, 3, 4, 5], vec![1, 2, 4, 4], 2);
        assert_eq!(maximum_bags(capacity, rocks, additional_rocks), 3);
    }
    #[test]
    fn run_tc2() {
        let (capacity, rocks, additional_rocks) = (vec![10, 2, 2], vec![2, 2, 0], 100);
        assert_eq!(maximum_bags(capacity, rocks, additional_rocks), 3);
    }
}

fn main() {
    let (capacity, rocks, additional_rocks) = (vec![2, 3, 4, 5], vec![1, 2, 4, 4], 2);
    assert_eq!(maximum_bags(capacity, rocks, additional_rocks), 3);
}
