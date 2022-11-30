use std::collections::HashMap;
use std::collections::HashSet;

fn unique_occurrences(arr: Vec<i32>) -> bool {
    let mut mp = HashMap::new();
    for c in arr {
        mp.entry(c).and_modify(|v| *v += 1).or_insert(1);
    }
    let mut hs = HashSet::new();
    for (_k, v) in mp.iter() {
        hs.insert(*v);
    }
    hs.len() == mp.len()
}

#[cfg(test)]
pub mod tests {
    use crate::unique_occurrences;
    #[test]
    fn run_tc1() {
        let arr = vec![1, 2, 2, 1, 1, 3];
        assert_eq!(unique_occurrences(arr), true);
    }

    #[test]
    fn run_tc2() {
        let arr = vec![1, 2];
        assert_eq!(unique_occurrences(arr), false);
    }

    #[test]
    fn run_tc3() {
        let arr = vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0];
        assert_eq!(unique_occurrences(arr), true);
    }
}

fn main() {
    //println!("Hello, world!");
    let arr = vec![1, 2, 2, 1, 1, 3];
    assert_eq!(unique_occurrences(arr), true);
}
