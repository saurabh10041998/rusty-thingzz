fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    nums.sort();
    let n = nums.len();

    let mut dp = vec![1; n];
    let mut parent = vec![0; n];

    for i in 0..n {
        parent[i] = i as i32;
    }

    let mut maxi = 1;
    for i in 1..n {
        for j in 0..i {
            if nums[i] % nums[j] == 0 && dp[j] + 1 > dp[i] {
                dp[i] = dp[j] + 1;
                parent[i] = j as i32;
            }
        }
        maxi = i32::max(maxi, dp[i]);
    }

    let mut idx = 0;
    while dp[idx] != maxi {
        idx += 1;
    }
    //  dp[idx] = maxi
    let mut res = vec![];
    while idx as i32 != parent[idx] {
        res.push(nums[idx]);
        idx = parent[idx] as usize;
    }
    res.push(nums[idx]);
    res.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use crate::largest_divisible_subset;
    #[test]
    fn run_tc1() {
        let nums = vec![1, 2, 3];
        assert_eq!(largest_divisible_subset(nums), vec![1, 2]);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![1, 2, 4, 8];
        assert_eq!(largest_divisible_subset(nums), vec![1, 2, 4, 8]);
    }
}

fn main() {
    println!("Hello, world!");
}
