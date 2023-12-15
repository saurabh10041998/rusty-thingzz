//! Here in this question, we have given path from src -> dst
//! Prepare the list of source by collecting the first element
//! in vector from Vec<Vec<String>>
//!
//! Then, to have destination city with no outgoing edge,
//! we just jave to find dst such that it is not in source list
//! we just prepared.
//! You can get list of dst by taking the second element
//! from vector from Vec<Vec<String>>

use std::collections::HashSet;

fn dest_city(paths: Vec<Vec<String>>) -> String {
    let sources = paths
        .iter()
        .map(|route| route[0].clone())
        .collect::<HashSet<String>>();
    paths
        .into_iter()
        .filter_map(|route| {
            if !sources.contains(&route[1]) {
                Some(route[1].clone())
            } else {
                None
            }
        })
        .nth(0)
        .unwrap()
}

#[cfg(test)]
pub mod tests {
    use crate::dest_city;
    #[test]
    fn run_tc1() {
        let paths = vec![
            vec![String::from("London"), String::from("New York")],
            vec![String::from("New York"), String::from("Lima")],
            vec![String::from("Lima"), String::from("Sao Paulo")],
        ];
        assert_eq!(dest_city(paths), String::from("Sao Paulo"));
    }
    #[test]
    fn run_tc2() {
        let paths = vec![
            vec![String::from("B"), String::from("C")],
            vec![String::from("D"), String::from("B")],
            vec![String::from("C"), String::from("A")],
        ];
        assert_eq!(dest_city(paths), String::from("A"));
    }
    #[test]
    fn run_tc3() {
        let paths = vec![vec![String::from("A"), String::from("Z")]];
        assert_eq!(dest_city(paths), String::from("Z"));
    }
}

fn main() {
    let paths = vec![
        vec![String::from("London"), String::from("New York")],
        vec![String::from("New York"), String::from("Lima")],
        vec![String::from("Lima"), String::from("Sao Paulo")],
    ];
    assert_eq!(dest_city(paths), String::from("Sao Paulo"));
}
