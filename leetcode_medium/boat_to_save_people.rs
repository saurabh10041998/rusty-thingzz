fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
    let mut boat = 0;
    let mut people = people;
    people.sort();
    let mut people_tracker = people.iter().map(|x| Some(*x)).collect::<Vec<_>>();
    let mut left = 0;
    let mut right = people_tracker.len() - 1;
    while left < right {
        if let (Some(l), Some(r)) = (people_tracker[left], people_tracker[right]) {
            if l + r <= limit {
                boat += 1;
                people_tracker[left] = None;
                people_tracker[right] = None;
                left += 1;
                right = right.checked_sub(1).unwrap();
            } else {
                right = right.checked_sub(1).unwrap();
            }
        } else {
            break;
        }
    }
    for i in 0..people_tracker.len() {
        match people_tracker[i] {
            Some(_) => boat += 1,
            None => {}
        }
    }
    boat
}

#[cfg(test)]
pub mod tests {
    use crate::num_rescue_boats;
    #[test]
    fn run_tc1() {
        let people = vec![1, 2];
        let limit = 3;
        assert_eq!(num_rescue_boats(people, limit), 1)
    }
    #[test]
    fn run_tc2() {
        let people = vec![3, 2, 2, 1];
        let limit = 3;
        assert_eq!(num_rescue_boats(people, limit), 3)
    }
    #[test]
    fn run_tc3() {
        let people = vec![3, 5, 3, 4];
        let limit = 5;
        assert_eq!(num_rescue_boats(people, limit), 4)
    }
}

fn main() {
    let people = vec![1, 2];
    let limit = 3;
    assert_eq!(num_rescue_boats(people, limit), 1)
}
