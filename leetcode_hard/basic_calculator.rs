fn is_digit(c: &char) -> bool {
    match c {
        '0'..='9' => true,
        _ => false,
    }
}
fn dig_from_slice(s: &str) -> i32 {
    (*&s[0..1].chars().nth(0).unwrap() as u8 - '0' as u8) as i32
}
fn calculate(s: String) -> i32 {
    let n = s.len();
    let (mut ans, mut cur_no, mut i, mut sign) = (0, 0, 0, 1);
    let mut st = Vec::new();
    while i < n {
        if is_digit(&s[i..i+1].chars().nth(0).unwrap()) {
            cur_no += dig_from_slice(&s[i..i+1]);
            while i + 1 < n && is_digit(&s[i+1..i+2].chars().nth(0).unwrap()) {
                cur_no = cur_no * 10  + dig_from_slice(&s[i+1..i+2]);
                i = i + 1;
            }
            ans += sign * cur_no;
            cur_no = 0;
            sign = 1;
        }else {
            match &s[i..i+1] {
                "+" => { 
                    sign = 1;
                },
                "-" => {
                    sign = -1;
                },
                "(" => {
                    st.push(ans);
                    st.push(sign);
                    ans = 0;
                    sign = 1;
                }, 
                ")" => {
                    let prev_sign = st.pop().unwrap();
                    ans = ans * prev_sign;
                    let prev_ans = st.pop().unwrap();
                    ans = prev_ans + ans;
                }
                _ => {
                    
                }            
            };
        }
        i = i + 1;
    }
    ans
}

#[cfg(test)]
pub mod tests {
    use crate::calculate;
    #[test]
    fn run_tc1() {
        let s = String::from("1 + 1");
        assert_eq!(calculate(s) , 2);
    }
    #[test]
    fn run_tc2() {
        let s = String::from(" 2-1 + 2 ");
        assert_eq!(calculate(s) , 3);
    }
    #[test]
    fn run_tc3() {
        let s = String::from("(1+(4+5+2)-3)+(6+8)");
        assert_eq!(calculate(s) , 23);
    }

    #[test]
    fn run_tc4(){
        let s = String::from("2147483647");
        assert_eq!(calculate(s), 2147483647);
    }
}

fn main() {
    let s = String::from("1 + 1");
    assert_eq!(calculate(s) , 2);
}
