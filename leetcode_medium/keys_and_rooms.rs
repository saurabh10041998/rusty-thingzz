use std::collections::HashSet;
use std::collections::VecDeque;
fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
    let mut q = VecDeque::new();
    let mut visited = HashSet::new();
    visited.insert(0);
    q.push_back(0);
    while let Some(curr) = q.pop_front() {
        visited.insert(curr);
        for room in rooms[curr as usize].iter() {
            if !visited.contains(room) {
                q.push_back(*room);
            }
        }
    }
    visited.len() == rooms.len()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let rooms = vec![vec![1], vec![2], vec![3], vec![]];
        assert_eq!(can_visit_all_rooms(rooms), true);
    }

    #[test]
    fn run_tc2() {
        let rooms = vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]];
        assert_eq!(can_visit_all_rooms(rooms), false);
    }
}

fn main() {
    //println!("Hello, world!");
    let rooms = vec![vec![1], vec![2], vec![3], vec![]];
    assert_eq!(can_visit_all_rooms(rooms), true);
}
