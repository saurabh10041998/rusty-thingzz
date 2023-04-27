fn bulb_switch(n: i32) -> i32 {
    (n as f64).sqrt() as i32
}

#[cfg(test)]
pub mod tests {
    use crate::bulb_switch;
    #[test]
    fn run_tc1() {
        let n = 3;
        assert_eq!(bulb_switch(n), 1);
    }
    #[test]
    fn run_tc2() {
        let n = 0;
        assert_eq!(bulb_switch(n), 0);
    }
    #[test]
    fn run_tc3() {
        let n = 1;
        assert_eq!(bulb_switch(n), 1);
    }
}

fn main() { 
    let n = 1;
    assert_eq!(bulb_switch(n), 1);
}
