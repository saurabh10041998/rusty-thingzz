pub struct Question {
    points: usize,
    brainpower: usize,
}

impl From<Vec<i32>> for Question {
    fn from(value: Vec<i32>) -> Self {
        Question {
            points: value[0] as usize,
            brainpower: value[1] as usize,
        }
    }
}

fn most_points(questions: Vec<Vec<i32>>) -> i64 {
    let mut qna = vec![];
    for question in questions {
        let q: Question = question.into();
        qna.push(q);
    }
    let n = qna.len();
    let mut dp = vec![0i64; n + 1];
    for i in (0..n).rev() {
        let next_question = usize::min(n, i + qna[i].brainpower + 1);
        dp[i] = i64::max(dp[i + 1], dp[next_question] + qna[i].points as i64);
    }
    dp[0]
}

#[cfg(test)]
pub mod tests {
    use crate::most_points;
    #[test]
    fn run_tc1() {
        let questions = vec![vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]];
        assert_eq!(most_points(questions), 5);
    }
    #[test]
    fn run_tc2() {
        let questions = vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]];
        assert_eq!(most_points(questions), 7);
    }
}
fn main() {
    let questions = vec![vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]];
    assert_eq!(most_points(questions), 5);
}
