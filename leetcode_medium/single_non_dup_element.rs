fn single_non_duplicate(nums: Vec<i32>) -> i32 {
    let mut low = 0;
    let mut high = nums.len() as i32;
    let inside = |x: i32| x >= 0 && x < nums.len() as i32;
    high = high - 1;
    while low <= high {
        let mid = low + (high - low) / 2;

        // For unique element, both of its left and right neighbour
        // Should be unique
        // If either of left or right doesn't exists, then return true..
        if (!inside(mid - 1) || nums[(mid - 1) as usize] != nums[mid as usize])
            && (!inside(mid + 1) || nums[(mid + 1) as usize] != nums[mid as usize])
        {
            return nums[mid as usize];
        }
        // Now two numbers eleminated at a time
        // Shift towards array with odd size (this is where unique element is)
        // [1 1 2](use this to search) 3(mid) 3(mid+1) [4 4](eliminate this.. even size..)
        let leftsize = if mid == 0 {
            0
        } else if nums[(mid - 1) as usize] == nums[mid as usize] {
            mid - 1
        } else {
            mid
        };
        if leftsize % 2 == 1 {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    unreachable!()
}

#[cfg(test)]
pub mod tests {
    use crate::single_non_duplicate;
    #[test]
    fn run_tc1() {
        let nums = vec![1, 1, 2, 3, 3, 4, 4, 8, 8];
        assert_eq!(single_non_duplicate(nums), 2);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![3, 3, 7, 7, 10, 11, 11];
        assert_eq!(single_non_duplicate(nums), 10);
    }
}

fn main() {
    let nums = vec![1, 1, 2, 3, 3, 4, 4, 8, 8];
    assert_eq!(single_non_duplicate(nums), 2);
}
