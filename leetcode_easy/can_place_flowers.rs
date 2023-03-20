fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let mut flowerbed = flowerbed;
    let mut n = n;
    for i in 0..flowerbed.len() {
        let prev = match i.checked_sub(1) {
            Some(idx) => flowerbed[idx] == 0,
            None => true,
        };
        let next = match i.checked_add(1) {
            Some(idx) if idx == flowerbed.len() => true,
            Some(idx) => flowerbed[idx] == 0,
            None => {
                unreachable!()
            }
        };
        if flowerbed[i] == 0 && prev && next {
            n -= 1;
            flowerbed[i] = 1;
        }
        if n <= 0 {
            return true;
        }
    }
    false
}

#[cfg(test)]
pub mod tests {
    use crate::can_place_flowers;
    #[test]
    fn run_tc1() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        let n = 1;
        assert_eq!(can_place_flowers(flowerbed, n), true);
    }
    #[test]
    fn run_tc2() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        let n = 2;
        assert_eq!(can_place_flowers(flowerbed, n), false);
    }
    #[test]
    fn run_tc3() {
        let flowerbed = vec![1, 0, 0, 0, 0, 1];
        let n = 2;
        assert_eq!(can_place_flowers(flowerbed, n), false);
    }
}

fn main() {
    let flowerbed = vec![1, 0, 0, 0, 1];
    let n = 1;
    assert_eq!(can_place_flowers(flowerbed, n), true);
}
