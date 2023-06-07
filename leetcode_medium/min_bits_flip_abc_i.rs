enum Bit {
    One,
    Zero
}

impl From<i32> for Bit { 
    fn from(value: i32) -> Self {
        match value {
            0 => Bit::Zero,
            1 => Bit::One,
            _ => panic!("[!!] Invalid binary symbol")
        }
    }
}

fn min_flips(mut a: i32, mut b: i32, mut c: i32) -> i32 {
    let mut res = 0;
    while a > 0 || b > 0 || c > 0 {
        let bit_a: Bit = (a & 1).into();
        let bit_b: Bit = (b & 1).into();
        let bit_c: Bit = (c & 1).into();

        match bit_c {
            Bit::Zero => {
                match (bit_a, bit_b) {
                    (Bit::Zero, Bit::Zero) => res += 0, 
                    (Bit::Zero, Bit::One) => res += 1,
                    (Bit::One, Bit::Zero) => res += 1,
                    (Bit::One, Bit::One) => res += 2,
                }
            },
            Bit::One => {
                match (bit_a, bit_b) {
                    (Bit::Zero, Bit::Zero) => res += 1,
                    (_, _) => res += 0
                }
            }
        }
        a >>= 1;
        b >>= 1;
        c >>= 1;
    }
    res
}

#[cfg(test)]
pub mod tests {
    use crate::min_flips;
    #[test]
    fn run_tc1() {
        let a = 2;
        let b = 6;
        let c = 5;
        assert_eq!(min_flips(a, b, c),  3);
    }
    #[test]
    fn run_tc2() {
        let a = 4;
        let b = 2;
        let c = 7;
        assert_eq!(min_flips(a, b, c),  1);
    }
    
}

fn main() {
    let a = 4;
    let b = 2;
    let c = 7;
    assert_eq!(min_flips(a, b, c),  1);
}
