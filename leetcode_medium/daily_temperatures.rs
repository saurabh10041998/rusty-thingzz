fn daily_temperatures(mut temperatures: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![];
    let mut stack = vec![];
    ans.push(0);
    stack.push((temperatures.pop().unwrap(), 0));
    for c in temperatures.iter().rev() {
        let mut cnt = 0;
        while stack.last().is_some() && stack.last().unwrap().0 <= *c {
            cnt += stack.last().unwrap().1;
            stack.pop().unwrap();
        }
        if stack.last().is_some() {
            cnt += 1;
        } else {
            cnt = 0;
        }
        ans.push(cnt);
        stack.push((*c, cnt));
    }
    ans.into_iter().rev().collect()
}

#[cfg(test)]
pub mod tests {
    use crate::daily_temperatures;
    #[test]
    fn run_tc1() {
        let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];
        assert_eq!(
            daily_temperatures(temperatures),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
    }

    #[test]
    fn run_tc2() {
        let temperatures = vec![30, 40, 50, 60];
        assert_eq!(daily_temperatures(temperatures), vec![1, 1, 1, 0]);
    }

    #[test]
    fn run_tc3() {
        let temperatures = vec![30, 60, 90];
        assert_eq!(daily_temperatures(temperatures), vec![1, 1, 0]);
    }
}

fn main() {
    //println!("Hello, world!");
    let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];
    assert_eq!(
        daily_temperatures(temperatures),
        vec![1, 1, 4, 2, 1, 1, 0, 0]
    );
}
