use std::collections::HashSet;
use std::mem::drop;

fn partition_string(s: String) -> i32 {
    let mut hs = HashSet::new();
    let mut partitions = 1;
    for c in s.chars() {
        if hs.contains(&c) {
            partitions += 1;
            drop(hs);
            hs = HashSet::new();
        }
        hs.insert(c);
    }
    partitions
}

#[cfg(test)]
pub mod tests {
    use crate::partition_string;
    #[test]
    fn run_tc1() {
        let s = String::from("abacaba");
        assert_eq!(partition_string(s), 4);
    }
    #[test]
    fn run_tc2() {
        let s = String::from("ssssss");
        assert_eq!(partition_string(s), 6);
    }
}

fn main() {
    let s = String::from("abacaba");
    assert_eq!(partition_string(s), 4);
}