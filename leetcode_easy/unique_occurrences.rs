use std::collections::{HashMap, HashSet};

fn unique_occurrences(arr: Vec<i32>) -> bool {
    let freq = arr.into_iter().fold(HashMap::new(), |mut accum, n| {
        accum
            .entry(n)
            .and_modify(|f: &mut usize| *f += 1)
            .or_insert(1usize);
        accum
    });
    let hs = freq.iter().fold(HashSet::new(), |mut accum, (&_k, &v)| {
        accum.insert(v);
        accum
    });
    freq.len() == hs.len()
}

#[cfg(test)]
pub mod tests {
    use crate::unique_occurrences;
    #[test]
    fn run_tc1() {
        let arr = vec![1, 2, 2, 1, 1, 3];
        assert!(unique_occurrences(arr));
    }
    #[test]
    fn run_tc2() {
        let arr = vec![1, 2];
        assert!(!unique_occurrences(arr));
    }
    #[test]
    fn run_tc3() {
        let arr = vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0];
        assert!(unique_occurrences(arr));
    }
}

fn main() {
    let arr = vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0];
    assert!(unique_occurrences(arr));
}
