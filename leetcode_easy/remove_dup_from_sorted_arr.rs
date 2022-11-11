fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut i:usize = 0;
    for j in 0..nums.len() {
        if  nums[i] != nums[j] { 
            i += 1;
        }
        nums[i] = nums[j];
    }
    (i + 1) as i32
}


// To check the mutable array.. while running the test
// use  cargo test -- --nocapture

#[cfg(test)]
pub mod tests {
    use crate::remove_duplicates;
 
    #[test]
    fn run_tc1() {
        let mut nums = vec![1,1,2];
        let result = remove_duplicates(&mut nums);
        println!("{:#?}", nums);
        assert_eq!(result, 2);
    }

    #[test]
    fn run_tc2() {
        let mut nums = vec![0,0,1,1,1,2,2,3,3,4];
        let result = remove_duplicates(&mut nums);
        println!("{:#?}", nums);
        assert_eq!(result, 5);
    }
}

fn main() {
    println!("Hello, world!");
    let mut nums = vec![0,0,1,1,1,2,2,3,3,4];
    let result = remove_duplicates(&mut nums);
    println!("{:#?}", nums);
    assert_eq!(result, 5);
}
