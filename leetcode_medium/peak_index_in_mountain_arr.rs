fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
    let mut low = 0;
    let mut high = arr.len() - 1;
    while low < high {
        let offset = match high.checked_sub(low) {
            Some(offset) => offset / 2,
            None => break,
        };
        let mid = low + offset;
        let prev = match mid.checked_sub(1) {
            Some(prev_idx) => arr[prev_idx],
            None => break,
        };
        let next = match mid.checked_add(1) {
            Some(next_idx) => arr[next_idx],
            None => unreachable!(),
        };
        if prev < arr[mid] && next < arr[mid] {
            return mid as i32;
        } else if next > arr[mid] {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    unreachable!()
}

#[cfg(test)]
pub mod tests {
    use crate::peak_index_in_mountain_array;
    #[test]
    fn run_tc1() {
        let arr = vec![0, 1, 0];
        assert_eq!(peak_index_in_mountain_array(arr), 1);
    }
    #[test]
    fn run_tc2() {
        let arr = vec![0, 2, 1, 0];
        assert_eq!(peak_index_in_mountain_array(arr), 1);
    }
    #[test]
    fn run_tc3() {
        let arr = vec![0, 10, 5, 2];
        assert_eq!(peak_index_in_mountain_array(arr), 1);
    }
}

fn main() {
    let arr = vec![0, 10, 5, 2];
    assert_eq!(peak_index_in_mountain_array(arr), 1);
}
