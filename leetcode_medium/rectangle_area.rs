use std::ops::Sub;

fn abs_diff<T: PartialOrd + Sub<Output = T>>(x: T, y: T) -> T {
    if x >= y {
        return x - y;
    }
    y - x
}

fn compute_area(
    ax1: i32,
    ay1: i32,
    ax2: i32,
    ay2: i32,
    bx1: i32,
    by1: i32,
    bx2: i32,
    by2: i32,
) -> i32 {
    let area1 = abs_diff(ax1, ax2) * abs_diff(ay1, ay2);
    let area2 = abs_diff(bx1, bx2) * abs_diff(by1, by2);
    let l = i32::max(
        i32::min(i32::max(ax1, ax2), i32::max(bx1, bx2))
            - i32::max(i32::min(ax1, ax2), i32::min(bx1, bx2)),
        0,
    );
    let r = i32::max(
        i32::min(i32::max(ay1, ay2), i32::max(by1, by2))
            - i32::max(i32::min(ay1, ay2), i32::min(by1, by2)),
        0,
    );
    area1 + area2 - (l * r)
}

#[cfg(test)]
pub mod tests {
    use crate::compute_area;
    #[test]
    fn run_tc1() {
        let (ax1, ay1, ax2, ay2, bx1, by1, bx2, by2) = (-3, 0, 3, 4, 0, -1, 9, 2);
        assert_eq!(compute_area(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2), 45);
    }
    #[test]
    fn run_tc2() {
        let (ax1, ay1, ax2, ay2, bx1, by1, bx2, by2) = (-2, -2, 2, 2, -2, -2, 2, 2);
        assert_eq!(compute_area(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2), 16);
    }
}

fn main() {
    println!("Hello, world!");
    let (ax1, ay1, ax2, ay2, bx1, by1, bx2, by2) = (-2, -2, 2, 2, -2, -2, 2, 2);
    assert_eq!(compute_area(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2), 16);
}
