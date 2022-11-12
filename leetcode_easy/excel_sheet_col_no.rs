fn title_to_number(s: String) -> i32 {
    let mut result = 0;
    for c in s.chars() {
        result = result * 26;
        result += (c as u8 - 'A' as u8  + 1) as i32;
    }
    result
}

#[cfg(test)]
pub mod tests {
    use crate::title_to_number;
    #[test]
    fn run_tc1() { 
        let s = String::from("A");
        assert_eq!(title_to_number(s), 1);
    }

    #[test]
    fn run_tc2() { 
        let s = String::from("AB");
        assert_eq!(title_to_number(s), 28);
    }

    #[test]
    fn run_tc3() { 
        let s = String::from("ZY");
        assert_eq!(title_to_number(s), 701);
    }
}

fn main() {
    println!("Hello, world!");
    let s = String::from("ZY");
    assert_eq!(title_to_number(s), 701);
}
