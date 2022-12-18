use std::collections::HashSet;


fn similar_pairs(words: Vec<String>) -> i32 {
    let mut count = 0;
    for i in 0..words.len() {
        for j in i + 1..words.len() {
            let hs1 = words[i].chars().collect::<HashSet<char>>();
            let hs2 = words[j].chars().collect::<HashSet<char>>();
            if hs1 == hs2 {
                count += 1;
            } 
        }
    }     
    count
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let data = ["aba","aabb","abcd","bac","aabc"];
        let mut words = vec![];
        for c in data {
            words.push(String::from(c));
        }
        assert_eq!(similar_pairs(words), 2);
    }
}
fn main() {
    println!("Hello, world!");
}
