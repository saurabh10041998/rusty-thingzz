fn hamming_weight(n: u32) -> i32 { 
    let mut count = 0;
    let mut n = n;
    for _ in 0..32 { 
        count += n & 1;
        n >>= 1;
    }
    count as i32 
}

#[cfg(test)]
pub mod tests {
    use crate::hamming_weight;
    #[test]
    fn run_tc1() { 
        let n = u32::from_str_radix("00000000000000000000000000001011", 2).unwrap();
        assert_eq!(hamming_weight(n), 3);
    }

    #[test]
    fn run_tc2() { 
        let n = u32::from_str_radix("00000000000000000000000010000000", 2).unwrap();
        assert_eq!(hamming_weight(n), 1);
    }
    #[test]
    fn run_tc3() { 
        let n = u32::from_str_radix("11111111111111111111111111111101", 2).unwrap();
        assert_eq!(hamming_weight(n), 31);
    }
}

fn main() {
    println!("Hello, world!");
    let n = u32::from_str_radix("11111111111111111111111111111101", 2).unwrap();
    assert_eq!(hamming_weight(n), 31);
}
