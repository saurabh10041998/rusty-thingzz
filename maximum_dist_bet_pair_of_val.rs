use std::cmp;
fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    if nums1.last() > nums2.first() { 
        return 0;
    }
    let (mut i, mut j, mut max_dist) = (0, 0, 0);
    while i < nums1.len() {
        if j < nums2.len() && nums1[i] <= nums2[j] {
           max_dist = cmp::max(max_dist, j - i);
           j += 1 
        }else  {
            i += 1;
            j += 1;
        }
    }
    max_dist as i32
}
fn main() {
    let vector1 = vec![55,30,5,4,2]; 
    let vector2 = vec![100,20,10,10,5];
    
    println!("test #1: {:?}", max_distance(vector1, vector2));

    let vector1 = vec![2,2,2];
    let vector2 = vec![10,10,1];

    println!("test #2: {:?}", max_distance(vector1, vector2));

    let vector1 = vec![30,29,19,5];
    let vector2 = vec![25,25,25,25,25];

    println!("test #3: {:?}", max_distance(vector1, vector2));

}
