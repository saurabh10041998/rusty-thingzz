fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
    let (mut bulky, mut heavy) = (false, false);
    if mass >= 100 {
        heavy = true;
    }
    let threshold = i32::pow(10, 4);
    if length >= threshold || width >= threshold || height >= threshold || mass >= threshold {
        bulky = true;
    }
    let volume = (length as i64) * (width as i64) * (height as i64);
    if volume >= i64::pow(10, 9) {
        bulky = true;
    }
    match bulky && heavy {
        true => String::from("Both"),
        false => {
            if bulky {
                String::from("Bulky")
            } else if heavy {
                String::from("Heavy")
            } else {
                String::from("Neither")
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let (length, width, height, mass) = (1000, 35, 700, 300);
        assert_eq!(
            categorize_box(length, width, height, mass),
            String::from("Heavy")
        );
    }
    #[test]
    fn run_tc2() {
        let (length, width, height, mass) = (200, 50, 800, 50);
        assert_eq!(
            categorize_box(length, width, height, mass),
            String::from("Neither")
        );
    }
    #[test]
    fn run_tc3() {
        assert_eq!(true, true);
    }
}
fn main() {
    println!("Hello, world!");
}
