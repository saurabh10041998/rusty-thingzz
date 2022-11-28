use std::collections::HashMap;
use std::collections::HashSet;

pub struct Match {
    winner: i32,
    looser: i32,
}

impl From<Vec<i32>> for Match {
    fn from(buffer: Vec<i32>) -> Self {
        Match {
            winner: buffer[0],
            looser: buffer[1],
        }
    }
}

fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut visited = HashSet::new();
    let mut loss = HashMap::new();

    for v in matches {
        let m: Match = v.into();
        visited.insert(m.winner);
        visited.insert(m.looser);
        loss.entry(m.looser).and_modify(|v| *v += 1).or_insert(1);
    }

    let (mut ans1, mut ans2) = (vec![], vec![]);
    for p in visited {
        if !loss.contains_key(&p) {
            ans1.push(p);
        } else if *loss.get(&p).unwrap() == 1 {
            ans2.push(p);
        }
    }
    ans1.sort();
    ans2.sort();
    vec![ans1, ans2]
}

#[cfg(test)]
pub mod tests {
    use crate::find_winners;
    #[test]
    fn run_tc1() {
        let maches = vec![
            vec![1, 3],
            vec![2, 3],
            vec![3, 6],
            vec![5, 6],
            vec![5, 7],
            vec![4, 5],
            vec![4, 8],
            vec![4, 9],
            vec![10, 4],
            vec![10, 9],
        ];
        assert_eq!(find_winners(maches), vec![vec![1, 2, 10], vec![4, 5, 7, 8]]);
    }

    #[test]
    fn run_tc2() {
        let maches = vec![vec![2, 3], vec![1, 3], vec![5, 4], vec![6, 4]];
        assert_eq!(find_winners(maches), vec![vec![1, 2, 5, 6], vec![]]);
    }
}

fn main() {
    println!("Hello, world!");

    let maches = vec![
        vec![1, 3],
        vec![2, 3],
        vec![3, 6],
        vec![5, 6],
        vec![5, 7],
        vec![4, 5],
        vec![4, 8],
        vec![4, 9],
        vec![10, 4],
        vec![10, 9],
    ];
    assert_eq!(find_winners(maches), vec![vec![1, 2, 10], vec![4, 5, 7, 8]]);
}
