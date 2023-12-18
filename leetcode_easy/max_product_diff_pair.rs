//! Answer to this problem is
//! (product of highest and second highest number) -
//!        (product of lowest and second lowest number)
//! This can be implemented in many ways like sorting, heap, set etc;
//! I stand with heap implementation.
//! Maintain the minheap and maxheap of numbers of `nums` array
//! p1 = get highest and second highest from max_heap
//! p2 = get lowest and second lowest from min_heap
//!
//! p1 - p2 is the ans
//!
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn max_product_difference(nums: Vec<i32>) -> i32 {
    let (mut min_heap, mut max_heap) = nums.into_iter().fold(
        (BinaryHeap::new(), BinaryHeap::new()),
        |(mut min_h, mut max_h), ele| {
            min_h.push(Reverse(ele));
            max_h.push(ele);
            (min_h, max_h)
        },
    );
    match (
        min_heap.pop(),
        min_heap.pop(),
        max_heap.pop(),
        max_heap.pop(),
    ) {
        (Some(Reverse(min1)), Some(Reverse(min2)), Some(max1), Some(max2)) => {
            max2 * max1 - min1 * min2
        }
        (_, _, _, _) => unreachable!(), // According constraints in question nums.len >= 4
    }
}

#[cfg(test)]
pub mod tests {
    use crate::max_product_difference;
    #[test]
    fn run_tc1() {
        let nums = vec![5, 6, 7, 2, 4];
        assert_eq!(max_product_difference(nums), 34);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![4, 2, 5, 9, 7, 4, 8];
        assert_eq!(max_product_difference(nums), 64);
    }
}

fn main() {
    let nums = vec![4, 2, 5, 9, 7, 4, 8];
    assert_eq!(max_product_difference(nums), 64);
}
