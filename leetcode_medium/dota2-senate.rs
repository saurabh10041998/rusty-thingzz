use std::collections::VecDeque;
use std::iter::FromIterator;

// Neat macro for initialization
#[derive(Default)]
struct DotaSenate {
    vote_queue: VecDeque<char>,
    ban_queue: VecDeque<char>,
    radiant: usize,
    dire: usize,
}

impl DotaSenate {
    fn add(&mut self, voter: char) {
        match voter {
            'D' => {
                self.vote_queue.push_back('D');
                self.dire += 1;
            }
            'R' => {
                self.vote_queue.push_back('R');
                self.radiant += 1;
            }
            _ => {
                unreachable!()
            }
        }
    }
    // Runs iterator until a winner is found
    fn get_winner(&mut self) -> String {
        self.find_map(|maybe_winner| maybe_winner).unwrap()
    }
}

impl FromIterator<char> for DotaSenate {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = char>,
    {
        let mut dota_senate = DotaSenate::default();
        iter.into_iter().for_each(|ch| dota_senate.add(ch));
        dota_senate
    }
}

impl Iterator for DotaSenate {
    type Item = Option<String>;
    fn next(&mut self) -> Option<Self::Item> {
        let voter = self.vote_queue.pop_front()?;
        // Check if this voter has been banned
        match self.ban_queue.front() {
            Some(banned_voter) if *banned_voter == voter => {
                self.ban_queue.pop_front().unwrap();
                match voter {
                    'D' => self.dire -= 1,
                    'R' => self.radiant -= 1,
                    _ => {}
                }
                // banned user should not get chance, continue election
                return Some(None);
            }
            Some(_) => {}
            None => {}
        }
        if voter == 'D' && self.radiant == 0 {
            // Dire victory
            Some(Some(String::from("Dire")))
        } else if voter == 'R' && self.dire == 0 {
            // Radiant victory
            Some(Some(String::from("Radiant")))
        } else {
            // Exercise your own voting right
            // Ban other party
            match voter {
                'D' => {
                    self.vote_queue.push_back('D');
                    self.ban_queue.push_back('R');
                }
                'R' => {
                    self.vote_queue.push_back('R');
                    self.ban_queue.push_back('D');
                }
                _ => {}
            }
            // Continue election
            Some(None)
        }
    }
}

fn predict_party_victory(senate: String) -> String {
    senate.chars().collect::<DotaSenate>().get_winner()
}

#[cfg(test)]
pub mod tests {
    use crate::predict_party_victory;
    #[test]
    fn run_tc1() {
        let senate = String::from("RD");
        assert_eq!(predict_party_victory(senate), String::from("Radiant"));
    }
    #[test]
    fn run_tc2() {
        let senate = String::from("RDD");
        assert_eq!(predict_party_victory(senate), String::from("Dire"));
    }
}

fn main() {
    let senate = String::from("RD");
    assert_eq!(predict_party_victory(senate), String::from("Radiant"));
}
