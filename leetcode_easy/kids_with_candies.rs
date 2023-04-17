fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max = *candies.iter().max().unwrap();
    candies
        .into_iter()
        .map(|c| c + extra_candies >= max)
        .collect::<Vec<bool>>()
}

#[cfg(test)]
pub mod tests {
    use crate::kids_with_candies;
    #[test]
    fn run_tc1() {
        let candies = vec![2, 3, 5, 1, 3];
        let extra_candies = 3;
        assert_eq!(
            kids_with_candies(candies, extra_candies),
            vec![true, true, true, false, true]
        );
    }
    #[test]
    fn run_tc2() {
        let candies = vec![4, 2, 1, 1, 2];
        let extra_candies = 1;
        assert_eq!(
            kids_with_candies(candies, extra_candies),
            vec![true, false, false, false, false]
        );
    }
    #[test]
    fn run_tc3() {
        let candies = vec![12, 1, 12];
        let extra_candies = 10;
        assert_eq!(
            kids_with_candies(candies, extra_candies),
            vec![true, false, true]
        );
    }
}

fn main() {
    let candies = vec![2, 3, 5, 1, 3];
    let extra_candies = 3;
    assert_eq!(
        kids_with_candies(candies, extra_candies),
        vec![true, true, true, false, true]
    );
}