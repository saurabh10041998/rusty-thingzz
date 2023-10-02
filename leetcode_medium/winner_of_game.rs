use std::collections::BinaryHeap;
#[derive(Debug)]
struct Game {
    aliceq: BinaryHeap<i32>,
    bobq: BinaryHeap<i32>,
    turn: bool,
}

impl Game {
    fn new() -> Self {
        Game {
            aliceq: BinaryHeap::new(),
            bobq: BinaryHeap::new(),
            turn: true,
        }
    }
    fn add(&mut self, colors: String) {
        let (mut last_val, mut count) = (0, 1);
        let colors = colors.chars().collect::<Vec<char>>();
        for i in 1..colors.len() {
            if colors[i] != colors[i - 1] {
                if colors[i - 1] == 'A' && count > 2 {
                    self.aliceq.push(count);
                }
                if colors[i - 1] == 'B' && count > 2 {
                    self.bobq.push(count);
                }
                count = 1;
            } else {
                count += 1;
            }
            last_val = i;
        }
        if colors[last_val] == 'A' && count > 2 {
            self.aliceq.push(count)
        }
        if colors[last_val] == 'B' && count > 2 {
            self.bobq.push(count);
        }
    }
    fn find_winner(&mut self) -> bool {
        self.find_map(|maybe_winner| maybe_winner).unwrap()
    }
}

impl Iterator for Game {
    type Item = Option<bool>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.turn {
            if let Some(mut count) = self.aliceq.pop() {
                count -= 1;
                if count > 2 {
                    self.aliceq.push(count);
                }
                self.turn = false;
                return Some(None);
            } else {
                // first Some : valid iterator element
                // Second Some :  we have winner as bob
                return Some(Some(false));
            }
        } else {
            if let Some(mut count) = self.bobq.pop() {
                count -= 1;
                if count > 2 {
                    self.bobq.push(count);
                }
                self.turn = true;
                return Some(None);
            } else {
                return Some(Some(true));
            }
        }
    }
}

fn winner_of_game(colors: String) -> bool {
    let mut g = Game::new();
    g.add(colors);
    g.find_winner()
}

#[cfg(test)]
pub mod tests {
    use crate::winner_of_game;
    #[test]
    fn run_tc1() {
        let colors = String::from("AAABABB");
        assert_eq!(winner_of_game(colors), true);
    }
    #[test]
    fn run_tc2() {
        let colors = String::from("ABBBBBBBAAA");
        assert_eq!(winner_of_game(colors), false);
    }
    #[test]
    fn run_tc3() {
        let colors = String::from("AA");
        assert_eq!(winner_of_game(colors), false);
    }
}

fn main() {
    let colors = String::from("ABBBBBBBAAA");
    assert_eq!(winner_of_game(colors), false);
}
