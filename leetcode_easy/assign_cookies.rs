use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
    let mut cookies = s
        .into_iter()
        .map(|x| Reverse(x))
        .collect::<BinaryHeap<Reverse<i32>>>();
    let mut contents = g
        .into_iter()
        .map(|x| Reverse(x))
        .collect::<BinaryHeap<Reverse<i32>>>();

    let mut count = 0;
    while let Some(Reverse(content)) = contents.pop() {
        while let Some(Reverse(cookie)) = cookies.pop() {
            if cookie >= content {
                count += 1;
                break;
            }
        }
    }
    count
}

#[cfg(test)]
pub mod tests {
    use crate::find_content_children;
    #[test]
    fn run_tc1() {
        let g = vec![1, 2, 3];
        let s = vec![1, 1];
        assert_eq!(find_content_children(g, s), 1);
    }
    #[test]
    fn run_tc2() {
        let g = vec![1, 2];
        let s = vec![1, 2, 3];
        assert_eq!(find_content_children(g, s), 2);
    }
}

fn main() {
    let g = vec![1, 2];
    let s = vec![1, 2, 3];
    assert_eq!(find_content_children(g, s), 2);
}
