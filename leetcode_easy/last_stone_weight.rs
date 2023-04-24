use std::collections::BinaryHeap;

fn last_stone_weight(stones: Vec<i32>) -> i32 {
    let mut hs = stones.into_iter().collect::<BinaryHeap<i32>>();
    while hs.len() > 1 {
        let x1 = hs.pop().unwrap();
        let x2 = hs.pop().unwrap();
        let res = x1 - x2;
        if res != 0 {
            hs.push(res);
        }
    }
    match hs.pop() {
        Some(val) => val,
        None => 0,
    }
}

#[cfg(test)]
pub mod tests {
    use crate::last_stone_weight;
    #[test]
    fn run_tc1() {
        let stones = vec![2, 7, 4, 1, 8, 1];
        assert_eq!(last_stone_weight(stones), 1);
    }
    #[test]
    fn run_tc2() {
        let stones = vec![1];
        assert_eq!(last_stone_weight(stones), 1);
    }
}

fn main() {
    let stones = vec![2, 7, 4, 1, 8, 1];
    assert_eq!(last_stone_weight(stones), 1);
}
