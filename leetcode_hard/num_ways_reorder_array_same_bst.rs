const MOD: i64 = i64::pow(10, 9) + 7;

fn dfs(nums: &Vec<i32>, tables: &Vec<Vec<i64>>) -> i64 {
    let mut left = vec![];
    let mut right = vec![];

    if nums.len() <= 2 {
        return 1;
    }

    for &num in nums.iter().skip(1) {
        if num < nums[0] {
            left.push(num);
        } else {
            right.push(num);
        }
    }

    let left_count = dfs(&left, tables);
    let right_count = dfs(&right, tables);
    let left_len = left.len();
    let right_len = right.len();
    let total = left_len + right_len;
    (tables[total][left_len] * left_count % MOD * right_count % MOD) % MOD
}

fn num_of_ways(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut tables = vec![vec![0; len + 1]; len + 1];

    // pascal triangle for efficient combination calculations
    for i in 1..=len {
        tables[i][0] = 1 as i64;
        tables[i][i] = 1 as i64;
        for j in 1..i {
            tables[i][j] = (tables[i - 1][j - 1] + tables[i - 1][j]) % MOD;
        }
    }

    let ans = dfs(&nums, &tables);
    ((ans % MOD) - 1) as i32
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let nums = vec![2, 1, 3];
        let result = num_of_ways(nums);
        assert_eq!(result, 1);
    }

    #[test]
    fn run_tc2() {
        let nums = vec![3, 4, 5, 1, 2];
        let result = num_of_ways(nums);
        assert_eq!(result, 5);
    }

    #[test]
    fn run_tc3() {
        let nums = vec![1, 2, 3];
        let result = num_of_ways(nums);
        assert_eq!(result, 0);
    }
}

fn main() {}
