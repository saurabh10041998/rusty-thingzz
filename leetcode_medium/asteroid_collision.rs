fn asteroid_collision(asteroid: Vec<i32>) -> Vec<i32> {
    let mut stack = vec![];
    for a in asteroid {
        if a < 0 {
            let mut finished = false;
            while let Some(&top) = stack.last() {
                if top < 0 {
                    break;
                }
                if -a > top {
                    stack.pop().unwrap();
                } else if -a == top {
                    stack.pop().unwrap();
                    finished = true;
                    break;
                } else {
                    finished = true;
                    break;
                }
            }
            if finished {
                continue;
            }
        }
        stack.push(a);
    }
    stack
}

#[cfg(test)]
pub mod tests {
    use crate::asteroid_collision;
    #[test]
    fn run_tc1() {
        let asteroid = vec![5, 10, -5];
        assert_eq!(asteroid_collision(asteroid), vec![5, 10]);
    }
    #[test]
    fn run_tc2() {
        let asteroid = vec![8, -8];
        assert_eq!(asteroid_collision(asteroid), vec![]);
    }
    #[test]
    fn run_tc3() {
        let asteroid = vec![10, 2, -5];
        assert_eq!(asteroid_collision(asteroid), vec![10]);
    }
}

fn main() {
    let asteroid = vec![5, 10, -5];
    assert_eq!(asteroid_collision(asteroid), vec![5, 10]);
}
