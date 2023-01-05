use std::cmp::Ordering;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    first: i64,
    second: i64,
}

impl From<&Vec<i32>> for Point {
    fn from(buffer: &Vec<i32>) -> Self {
        Point {
            first: buffer[0] as i64,
            second: buffer[1] as i64,
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.first == other.first {
            return self.second.partial_cmp(&other.second);
        }
        self.first.partial_cmp(&other.first)
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.first == other.first {
            return self.second.cmp(&other.second);
        }
        self.first.cmp(&other.first)
    }
}

fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
    let mut arr = vec![];
    for v in points.iter() {
        let p: Point = v.into();
        arr.push(p);
    }
    arr.sort();
    let mut count = 1;
    let mut current_shooting_range = Point {
        first: arr[0].first,
        second: arr[0].second,
    };

    for &p in arr.iter().skip(1) {
        if p.first > current_shooting_range.second || p.second < current_shooting_range.first {
            // No overlap
            count += 1;
            current_shooting_range.first = p.first;
            current_shooting_range.second = p.second;
        } else {
            // partial or no overlap
            current_shooting_range.first = i64::max(current_shooting_range.first, p.first);
            current_shooting_range.second = i64::min(current_shooting_range.second, p.second);
        }
    }
    count
}

#[cfg(test)]
pub mod tests {
    use crate::find_min_arrow_shots;
    #[test]
    fn run_tc1() {
        let points = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];
        assert_eq!(find_min_arrow_shots(points), 2);
    }
    #[test]
    fn run_tc2() {
        let points = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]];
        assert_eq!(find_min_arrow_shots(points), 4);
    }
    #[test]
    fn run_tc3() {
        let points = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]];
        assert_eq!(find_min_arrow_shots(points), 2);
    }
    #[test]
    fn run_tc4() {
        let points = vec![vec![-2147483648, 2147483647]];
        assert_eq!(find_min_arrow_shots(points), 1);
    }
}

fn main() {
    let points = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];
    assert_eq!(find_min_arrow_shots(points), 2);
}
