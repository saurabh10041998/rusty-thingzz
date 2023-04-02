fn helper(potions: &Vec<i32>, spell: i32, success: i64) -> i32 {
    let mut low = 0;
    let mut high = potions.len() - 1;
    let mut ans = potions.len();
    while low <= high {
        let mid = low
            + match high.checked_sub(low) {
                Some(offset) => offset / 2,
                None => unreachable!(),
            };
        let product: i64 = (potions[mid] as i64) * (spell as i64);
        if product >= success {
            ans = mid;
            high = match mid.checked_sub(1) {
                Some(idx) => idx,
                None => break,
            }
        } else {
            low = mid + 1;
        }
    }
    (potions.len() - ans) as i32
}

fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
    let mut potions = potions;
    potions.sort();
    let mut res = Vec::new();
    for s in spells {
        res.push(helper(&potions, s, success));
    }
    res
}

#[cfg(test)]
pub mod tests {
    use crate::successful_pairs;
    #[test]
    fn run_tc1() {
        let spells = vec![5, 1, 3];
        let potions = vec![1, 2, 3, 4, 5];
        let success = 7;
        assert_eq!(successful_pairs(spells, potions, success), vec![4, 0, 3]);
    }
    #[test]
    fn run_tc2() {
        let spells = vec![3, 1, 2];
        let potions = vec![8, 5, 8];
        let success = 16;
        assert_eq!(successful_pairs(spells, potions, success), vec![2, 0, 2]);
    }
    #[test]
    fn run_tc3() {
        let spells = vec![1, 2, 3, 4, 5, 6, 7];
        let potions = vec![1, 2, 3, 4, 5, 6, 7];
        let success = 25;
        assert_eq!(
            successful_pairs(spells, potions, success),
            vec![0, 0, 0, 1, 3, 3, 4]
        );
    }
}

fn main() {
    let spells = vec![5, 1, 3];
    let potions = vec![1, 2, 3, 4, 5];
    let success = 7;
    assert_eq!(successful_pairs(spells, potions, success), vec![4, 0, 3]);
}
