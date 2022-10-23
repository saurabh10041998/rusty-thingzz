use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize)->(&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    assert!(mid <= len);

    //(&mut slice[..mid], &mut slice[mid..])
    // SAFETY : we are borrowing from the slice
    // with valid index and borrowed two parts are not overlapping
    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
        slice::from_raw_parts_mut(ptr.offset(mid as isize), len  - mid))
    }
}



fn main() {
    let mut nums = vec![1,2,3,4,5,6,];

    let r = &mut nums;
    
    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1,2,3]);
    assert_eq!(b, &mut [4,5,6,]);
}
