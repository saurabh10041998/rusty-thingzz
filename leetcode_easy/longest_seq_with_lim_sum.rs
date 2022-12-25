fn upper_bound(arr: &Vec<i32>, n: usize, x: i32) -> usize { 
    let (mut low, mut high) = (0, n);
    while low < high { 
        let mid = low + (high - low) / 2;
        if x >= arr[mid] {
            low = mid + 1;
        }else {
            high = mid;
        }
    }
    if low < n && arr[low] <= x {
        low += 1;
    }
    low
}
fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    nums.sort();
    let mut pref = vec![0;nums.len()];
    pref[0] = nums[0];
    for i in 1..nums.len() {
        pref[i] = pref[i - 1] + nums[i];
    }
    let mut ans = vec![];
    for c in queries.iter() {
        let a = upper_bound(&pref, nums.len(), *c);
        ans.push(a as i32);
    }
    ans
}

#[cfg(test)]
pub mod tests {
    use crate::answer_queries;
    #[test]
    fn run_tc1() {
        let nums = vec![4,5,2,1];
        let queries = vec![3, 10, 21];
        assert_eq!(answer_queries(nums, queries), vec![2,3,4]);
    }

    #[test]
    fn run_tc2() {
        let nums = vec![2,3,4,5];
        let queries = vec![1];
        assert_eq!(answer_queries(nums, queries), vec![0]);
    }
}

fn main() {
    let nums = vec![4,5,2,1];
    let queries = vec![3, 10, 21];
    assert_eq!(answer_queries(nums, queries), vec![2,3,4]);
}
