use std::collections::VecDeque;

fn valid_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
    let mut stack = vec![];
    let mut q = popped.into_iter().collect::<VecDeque<i32>>();
    for c in pushed {
        stack.push(c);
        while let (Some(&x), Some(&y)) = (stack.last(), q.front()) {
            if x == y {
                stack.pop().unwrap();
                q.pop_front().unwrap();
            } else {
                break;
            }
        }
    }
    stack.is_empty()
}

#[cfg(test)]
pub mod tests {
    use crate::valid_stack_sequences;
    #[test]
    fn run_tc1() {
        let pushed = vec![1, 2, 3, 4, 5];
        let popped = vec![4, 5, 3, 2, 1];
        assert_eq!(valid_stack_sequences(pushed, popped), true);
    }
    #[test]
    fn run_tc2() {
        let pushed = vec![1, 2, 3, 4, 5];
        let popped = vec![4, 3, 5, 1, 2];
        assert_eq!(valid_stack_sequences(pushed, popped), false);
    }
}

fn main() {
    let pushed = vec![1, 2, 3, 4, 5];
    let popped = vec![4, 5, 3, 2, 1];
    assert_eq!(valid_stack_sequences(pushed, popped), true);
}
