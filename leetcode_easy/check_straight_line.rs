struct Point {
    x: i64,
    y: i64,
}

impl From<&Vec<i32>> for Point {
    fn from(values: &Vec<i32>) -> Self {
        assert_eq!(values.len(), 2);
        Point {
            x: values[0] as i64,
            y: values[1] as i64,
        }
    }
}

fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
    for c in coordinates.windows(3) {
        let p1: Point = (&c[0]).into();
        let p2: Point = (&c[1]).into();
        let p3: Point = (&c[2]).into();

        if (p2.y - p1.y) * (p3.x - p2.x) != (p3.y - p2.y) * (p2.x - p1.x) {
            return false;
        }
    }
    true
}

#[cfg(test)]
pub mod tests {
    use crate::check_straight_line;
    #[test]
    fn run_tc1() {
        let coordinates = vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![6, 7],
        ];
        assert_eq!(check_straight_line(coordinates), true);
    }
    #[test]
    fn run_tc2() {
        let coordinates = vec![
            vec![1, 1],
            vec![2, 2],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![7, 7],
        ];
        assert_eq!(check_straight_line(coordinates), false);
    }
}

fn main() {
    let coordinates = vec![
        vec![1, 2],
        vec![2, 3],
        vec![3, 4],
        vec![4, 5],
        vec![5, 6],
        vec![6, 7],
    ];
    assert_eq!(check_straight_line(coordinates), true);
}
