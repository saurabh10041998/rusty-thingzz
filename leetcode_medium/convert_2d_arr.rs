use std::collections::HashMap;

fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.into_iter()
        .fold((HashMap::new(), Vec::new()), |(mut freq, mut ans), ele| {
            freq.entry(ele).and_modify(|f| *f += 1).or_insert(1usize);
            match freq.get(&ele) {
                Some(&freq) => {
                    while freq - 1 >= ans.len() {
                        ans.push(Vec::new());
                    }
                    ans[freq - 1].push(ele);
                }
                None => {
                    unreachable!();
                }
            }
            (freq, ans)
        })
        .1
}

mod validator {
    use std::collections::HashSet;
    pub fn valid_configuration(nums: Vec<Vec<i32>>, expected_rows: usize) -> bool {
        let rows = nums.len();
        for n in nums {
            let hs = n.iter().collect::<HashSet<_>>();
            if hs.len() != n.len() {
                return false;
            }
        }
        true && expected_rows == rows
    }
}

#[cfg(test)]
pub mod tests {
    use crate::find_matrix;
    use crate::validator::valid_configuration;
    #[test]
    fn run_tc1() {
        let nums = vec![1, 3, 4, 1, 2, 3, 1];
        assert!(valid_configuration(find_matrix(nums), 3));
    }
    #[test]
    fn run_tc2() {
        let nums = vec![1, 2, 3, 4];
        assert!(valid_configuration(find_matrix(nums), 1));
    }
}

fn main() {
    use crate::validator::valid_configuration;
    let nums = vec![1, 2, 3, 4];
    assert!(valid_configuration(find_matrix(nums), 1));
}
