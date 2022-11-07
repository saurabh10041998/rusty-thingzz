fn maximum69_number(num: i32) -> i32 {
    let mut num = num;
    let mut buffer =  vec![0i32;4];
    let mut i:i32 =  3;
    while num != 0 {
        buffer[i as usize] = num % 10;
        i -= 1;
        num /= 10;
    }
    for n in buffer.iter_mut() {
        if *n == 6 {
            *n =  9;
            break;
        }
    }
    let mut mul = 1;
    let mut ans = 0;
    for c in buffer.iter().rev() { 
        ans = ans + (c * mul);
        mul = mul * 10;
    }
    ans
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let num = 9669;
        assert_eq!(maximum69_number(num), 9969);
    }

    #[test]
    fn run_tc2() {
        let num = 9996;
        assert_eq!(maximum69_number(num), 9999);
    }

    #[test]
    fn run_tc3() {
        let num = 9999;
        assert_eq!(maximum69_number(num), 9999);
    }
}

fn main() {
    println!("Hello, world!");
    let num = 9999;
    assert_eq!(maximum69_number(num), 9999);
}
