use std::cmp::Ordering;
use std::cmp::Reverse;

#[derive(Debug, PartialEq, Eq)]
pub struct Player {
    score: i32,
    age: i32,
}

impl Player {
    #[allow(dead_code)]
    fn new(score: i32, age: i32) -> Self {
        Player { score, age }
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.age == other.age {
            return self.score.partial_cmp(&other.score);
        }
        self.age.partial_cmp(&other.age)
    }
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.age == other.age {
            return self.score.cmp(&other.score);
        }
        self.age.cmp(&other.age)
    }
}

impl From<(i32, i32)> for Player {
    fn from(value: (i32, i32)) -> Self {
        Player {
            age: value.0,
            score: value.1,
        }
    }
}

fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
    let mut players: Vec<Player> = vec![];
    for (&x, &y) in ages.iter().zip(scores.iter()) {
        players.push((x, y).into())
    }
    players.sort_by(|a, b| Reverse(a).cmp(&Reverse(b)));

    let mut dp = vec![0; players.len()];
    for i in 0..players.len() {
        dp[i] = players[i].score;
        for j in 0..i {
            if players[j].score >= players[i].score {
                dp[i] = i32::max(dp[i], dp[j] + players[i].score);
            }
        }
    }
    *dp.iter().max().unwrap()
}

#[cfg(test)]
pub mod tests {
    use crate::best_team_score;
    #[test]
    fn run_tc1() {
        let scores = vec![1, 3, 5, 10, 15];
        let ages = vec![1, 2, 3, 4, 5];
        assert_eq!(best_team_score(scores, ages), 34);
    }

    #[test]
    fn run_tc2() {
        let scores = vec![4, 5, 6, 5];
        let ages = vec![2, 1, 2, 1];
        assert_eq!(best_team_score(scores, ages), 16);
    }

    #[test]
    fn run_tc3() {
        let scores = vec![1, 2, 3, 5];
        let ages = vec![8, 9, 10, 1];
        assert_eq!(best_team_score(scores, ages), 6);
    }
}

fn main() {
    let scores = vec![1, 3, 5, 10, 15];
    let ages = vec![1, 2, 3, 4, 5];
    best_team_score(scores, ages);
}
