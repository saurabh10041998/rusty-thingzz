use std::collections::HashMap;

fn total_fruit(fruits: Vec<i32>) -> i32 {
    let mut type_count = 0;
    let mut cur_max = 0;
    let mut start = 0;
    let mut freq = HashMap::new();
    let mut ans = 0;
    for i in 0..fruits.len() {
        if !freq.contains_key(&fruits[i]) {
            type_count += 1;
        }
        freq.entry(fruits[i]).and_modify(|v| *v += 1).or_insert(1);
        cur_max += 1;
        while type_count > 2 && start < i {
            freq.entry(fruits[start]).and_modify(|v| *v -= 1);
            match freq.get_mut(&fruits[start]) {
                Some(v) if *v == 0 => {
                    freq.remove(&fruits[start]);
                    type_count -= 1;
                }
                Some(_v) => {}
                None => unreachable!(),
            }
            start += 1; //slide the window
            cur_max -= 1; // decrease the window size
        }
        ans = i32::max(ans, cur_max);
    }
    ans
}

#[cfg(test)]
pub mod tests {
    use crate::total_fruit;
    #[test]
    fn run_tc1() {
        let fruits = vec![1, 2, 1];
        assert_eq!(total_fruit(fruits), 3);
    }

    #[test]
    fn run_tc2() {
        let fruits = vec![0, 1, 2, 2];
        assert_eq!(total_fruit(fruits), 3);
    }
    #[test]
    fn run_tc3() {
        let fruits = vec![1, 2, 3, 2, 2];
        assert_eq!(total_fruit(fruits), 4);
    }
    #[test]
    fn run_tc4() {
        let fruits = vec![3,3,3,1,2,1,1,2,3,3,4];
        assert_eq!(total_fruit(fruits), 5);
    }
}

fn main() {
    let fruits = vec![1, 2, 1];
    assert_eq!(total_fruit(fruits), 3);
}
