fn reverse_bits(x: u32) -> u32 {
    let mut res: u32 = 0;
    for i in 0..32{
        let bit = (x >> i) & 1;
        res = res | (bit << (31 - i));
    }
    res
}

#[cfg(test)]
pub mod tests {
    use crate::reverse_bits;
    #[test]
    fn run_tc1() {
        let x: u32 = u32::from_str_radix("00000010100101000001111010011100", 2).unwrap();
        assert_eq!(reverse_bits(x), u32::from_str_radix("00111001011110000010100101000000", 2).unwrap());
    }

    #[test]
    fn run_tc2() {
        let x: u32 = u32::from_str_radix("11111111111111111111111111111101", 2).unwrap();
        assert_eq!(reverse_bits(x), u32::from_str_radix("10111111111111111111111111111111", 2).unwrap());
    }
}

fn main() {
    println!("Hello, world!");
    let x: u32 = 43261596;
    assert_eq!(reverse_bits(x), 964176192)
}
