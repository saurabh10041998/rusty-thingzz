use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Sub;

// need to create this struct as f64 cannot be used as
// key in HashMap
// as f64 does not implement hash trait
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Fraction {
    numerator: i32,
    denomenator: i32,
}

impl Fraction {
    fn new(numerator: i32, denomenator: i32) -> Self {
        Fraction {
            numerator,
            denomenator,
        }
    }
}

impl Hash for Fraction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.numerator, self.denomenator).hash(state);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

impl From<&Vec<i32>> for Point {
    fn from(buffer: &Vec<i32>) -> Self {
        Point::new(buffer[0], buffer[1])
    }
}

fn abs<T: PartialOrd + Sub<Output = T>>(x: T, y: T) -> T {
    if x > y {
        return x - y;
    }
    y - x
}

fn gcd(first: i32, second: i32) -> i32 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }
    if min == 0 {
        return max;
    }
    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }
        max = min;
        min = res;
    }
}

fn max_points(points: Vec<Vec<i32>>) -> i32 {
    let mut points_lst = vec![];
    for p in points.iter() {
        let pt: Point = p.into();
        points_lst.push(pt);
    }
    let mut max = 0;
    for i in 0..points_lst.len() {
        let mut slopes = HashMap::new();
        for j in i + 1..points_lst.len() {
            let (numerator, denominator) = if points_lst[i].x == points_lst[j].x {
                (i32::MAX, 1)
            } else {
                let mut numerator = points_lst[i].y - points_lst[j].y;
                let mut denomenator = points_lst[i].x - points_lst[j].x;
                // transfer sign from denomenator to numerator
                if denomenator < 0 {
                    numerator *= -1;
                    denomenator *= -1;
                }
                let _gcd = gcd(abs(numerator, 0), abs(denomenator, 0));
                (numerator / _gcd, denomenator / _gcd)
            };
            let fraction = Fraction::new(numerator, denominator);
            slopes.entry(fraction).and_modify(|v| *v += 1).or_insert(1);
        }
        let tmp = match slopes.values().max() {
            Some(val) => *val + 1,
            None => 1,
        };
        max = i32::max(tmp, max);
    }
    max
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let points = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
        assert_eq!(max_points(points), 3);
    }
    #[test]
    fn run_tc2() {
        let points = vec![
            vec![1, 1],
            vec![3, 2],
            vec![5, 3],
            vec![4, 1],
            vec![2, 3],
            vec![1, 4],
        ];
        assert_eq!(max_points(points), 4);
    }

    #[test]
    fn run_tc3() {
        let points = vec![vec![0, 0], vec![2, 2], vec![-1, -1]];
        assert_eq!(max_points(points), 3);
    }
}

fn main() {
    let points = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
    assert_eq!(max_points(points), 3);
}
