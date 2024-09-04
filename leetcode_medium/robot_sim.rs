use std::collections::HashSet;

fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
    let (mut x, mut y, mut d) = (0, 0, 0);
    let mut res = 0;
    let dir = ["N", "E", "S", "W"];
    let obstacle_set = obstacles
        .into_iter()
        .map(|x| (x[0], x[1]))
        .collect::<HashSet<(i32, i32)>>();
    for command in commands {
        if command == -1 {
            d += 1;
            d %= 4;
        } else if command == -2 {
            d -= 1;
            d = (d + 4) % 4;
        } else {
            match dir[d as usize] {
                "N" => {
                    for _ in 0..command {
                        if obstacle_set.contains(&(x, y + 1)) {
                            break;
                        }
                        y = y + 1;
                    }
                }
                "E" => {
                    for _ in 0..command {
                        if obstacle_set.contains(&(x + 1, y)) {
                            break;
                        }
                        x = x + 1;
                    }
                }
                "S" => {
                    for _ in 0..command {
                        if obstacle_set.contains(&(x, y - 1)) {
                            break;
                        }
                        y = y - 1;
                    }
                }
                "W" => {
                    for _ in 0..command {
                        if obstacle_set.contains(&(x - 1, y)) {
                            break;
                        }
                        x = x - 1;
                    }
                }
                _ => {
                    unreachable!("I don't where I am {}", d);
                }
            }
        }
        res = i32::max(res, x * x + y * y);
    }
    res
}

#[cfg(test)]
pub mod tests {
    use crate::robot_sim;
    #[test]
    fn run_tc1() {
        let commands = vec![4, -1, 3];
        let obstacles = vec![];
        assert_eq!(robot_sim(commands, obstacles), 25);
    }
    #[test]
    fn run_tc2() {
        let commands = vec![4, -1, 4, -2, 4];
        let obstacles = vec![vec![2, 4]];
        assert_eq!(robot_sim(commands, obstacles), 65);
    }
    #[test]
    fn run_tc3() {
        let commands = vec![6, -1, -1, 6];
        let obstacles = vec![];
        assert_eq!(robot_sim(commands, obstacles), 36);
    }
    #[test]
    fn run_tc4() {
        let commands = vec![-2, 8, 3, 7, -1];
        let obstacles = vec![
            vec![-4, -1],
            vec![1, -1],
            vec![1, 4],
            vec![5, 0],
            vec![4, 5],
            vec![-2, -1],
            vec![2, -5],
            vec![5, 1],
            vec![-3, -1],
            vec![5, -3],
        ];
        assert_eq!(robot_sim(commands, obstacles), 324);
    }
}

fn main() {
    let commands = vec![4, -1, 4, -2, 4];
    let obstacles = vec![vec![2, 4]];
    assert_eq!(robot_sim(commands, obstacles), 65);
}
