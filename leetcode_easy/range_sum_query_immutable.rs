struct NumArray {
    pref: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut n = NumArray {
            pref: vec![0;nums.len()]
        };
        n.pref[0] = nums[0];
        for i in 1..nums.len() {
            n.pref[i] = n.pref[i - 1] + nums[i];
        }
        n
    }
    fn num_ranges(&self, left: i32, right: i32) -> i32 {
        assert!(left >= 0 && right < self.pref.len() as i32 );
        match (left as usize).checked_sub(1) {
            Some(v) => self.pref[right as usize] - self.pref[v],
            None => self.pref[right as usize],
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let num_arr = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
        assert_eq!(num_arr.num_ranges(0, 2), 1);
        assert_eq!(num_arr.num_ranges(2, 5), -1);
        assert_eq!(num_arr.num_ranges(0, 5), -3);
    }
}

fn main() {
    let num_arr = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
    assert_eq!(num_arr.num_ranges(0, 2), 1);
    assert_eq!(num_arr.num_ranges(2, 5), -1);
    assert_eq!(num_arr.num_ranges(0, 5), -3);
}