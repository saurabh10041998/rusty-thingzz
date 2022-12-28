fn get_char(offset: u8) -> char {
    (0x41 + offset) as char
}
fn convert_to_title(column_number: i32) -> String {
    let mut column_number = column_number;
    let mut log = vec![];
    while column_number != 0 {
        column_number -= 1;
        let offset = column_number % 26;
        log.push(get_char(offset as u8));
        column_number /= 26;
    }
    let mut ans = String::new();
    while let Some(c) = log.pop() {
        ans.push(c);
    }
    ans
}

#[cfg(test)]
pub mod tests {
    use crate::convert_to_title;
    #[test]
    fn run_tc1() {
        let column_number = 28;
        assert_eq!(convert_to_title(column_number), String::from("AB"));
    }
    #[test]
    fn run_tc2() {
        let column_number = 701;
        assert_eq!(convert_to_title(column_number), String::from("ZY"));
    }
    #[test]
    fn run_tc3() {
        let column_number = 1;
        assert_eq!(convert_to_title(column_number), String::from("A"));
    }
}
fn main() {
    let column_number = 28;
    assert_eq!(convert_to_title(column_number), String::from("AB"));
}
