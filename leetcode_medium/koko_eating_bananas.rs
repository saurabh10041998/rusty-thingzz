fn ceil_div(a: i32, b: i32) -> i32 {
    (a / b) + (a % b != 0) as i32
}

fn possible(max_rate: i32, piles: &Vec<i32>, h: i32) -> bool {
    let mut hr = 0;
    for p in piles {
        let t = ceil_div(*p, max_rate);
        hr += t;
    }
    hr <= h
}

fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let mut low = 1;
    let mut high = piles.iter().max().copied().unwrap();
    while low < high {
        let mid = low + (high - low) / 2;
        match possible(mid, &piles, h) {
            true => {
                high = mid;
            }
            false => {
                low = mid + 1;
            }
        }
    }
    low
}

#[cfg(test)]
pub mod tests {
    use crate::min_eating_speed;
    #[test]
    fn run_tc1() {
        let piles = vec![3, 6, 7, 11];
        let h = 8;
        assert_eq!(min_eating_speed(piles, h), 4);
    }
    #[test]
    fn run_tc2() {
        let piles = vec![30, 11, 23, 4, 20];
        let h = 5;
        assert_eq!(min_eating_speed(piles, h), 30);
    }

    #[test]
    fn run_tc3() {
        let piles = vec![30, 11, 23, 4, 20];
        let h = 6;
        assert_eq!(min_eating_speed(piles, h), 23);
    }
}

fn main() {
    let piles = vec![3, 6, 7, 11];
    let h = 8;
    assert_eq!(min_eating_speed(piles, h), 4);
}
