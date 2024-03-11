use std::collections::HashMap;

fn custom_sort_string(order: String, s: String) -> String {
    let ord: HashMap<char, usize> = order.chars().enumerate().map(|(i, x)| (x, i)).collect();
    let mut buckets = vec![vec![]; ord.len() + 1];
    for c in s.chars() {
        match ord.get(&c) {
            Some(&idx) => {
                buckets[idx].push(c);
            }
            None => {
                buckets[ord.len()].push(c);
            }
        }
    }
    buckets
        .into_iter()
        .flat_map(|bucket| bucket.into_iter())
        .collect()
}

#[cfg(test)]
pub mod tests {
    use crate::custom_sort_string;
    #[test]
    fn run_tc1() {
        let order = String::from("cba");
        let s = String::from("abcd");
        assert_eq!(custom_sort_string(order, s), String::from("cbad"));
    }
    #[test]
    fn run_tc2() {
        let order = String::from("bcafg");
        let s = String::from("abcd");
        assert_eq!(custom_sort_string(order, s), String::from("bcad"));
    }
}

fn main() {
    let order = String::from("cba");
    let s = String::from("abcd");
    assert_eq!(custom_sort_string(order, s), String::from("cbad"));
}
