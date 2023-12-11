use std::collections::HashSet;

fn find_special_integer(arr: Vec<i32>) -> i32 {
    arr.windows(arr.len() / 4 + 1)
        .filter_map(|w| {
            if w.into_iter().all(|n| *n == w[0]) {
                Some(w[0])
            } else {
                None
            }
        })
        .collect::<HashSet<i32>>()
        .into_iter()
        .nth(0)
        .unwrap()
}

#[cfg(test)]
pub mod tests {
    use crate::find_special_integer;
    #[test]
    fn run_tc1() {
        let arr = vec![1, 2, 2, 6, 6, 6, 6, 7, 10];
        assert_eq!(find_special_integer(arr), 6);
    }
    #[test]
    fn run_tc2() {
        let arr = vec![1, 1];
        assert_eq!(find_special_integer(arr), 1);
    }
}

fn main() {
    let arr = vec![1, 1];
    assert_eq!(find_special_integer(arr), 1);
}
