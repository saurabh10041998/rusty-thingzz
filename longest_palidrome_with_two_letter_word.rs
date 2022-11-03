use std::cmp::min;
use std::collections::HashMap;

fn longest_palindrome(words: Vec<String>) -> i32 {
    let mut hs: HashMap<String, u32> = HashMap::new();
    let (mut tmp, mut ans) = (0, 0);
    for word in words {
        hs.entry(word).and_modify(|val| *val += 1).or_insert(1);
    }
    let mut single: bool = false;
    for (k, v) in hs.clone().iter_mut() {
        if &k[0..1] == &k[1..] {
            if *v % 2 == 1 {
                single = true;
                tmp += *v - 1;
            } else {
                tmp += *v;
            }
            hs.remove(k);
        } else {
            let mirror = k.chars().rev().collect::<String>();
            if hs.contains_key(&mirror) {
                ans += min(*v, *hs.get(&mirror).unwrap()) * 4;
                hs.remove(k);
                hs.remove(&mirror);
            }
        }
    }
    if single {
        ans += tmp * 2 + 2;
    } else {
        ans += tmp * 2;
    }
    ans as i32
}
fn main() {
    let words = vec![
        String::from("ab"),
        String::from("ty"),
        String::from("yt"),
        String::from("lc"),
        String::from("cl"),
        String::from("ab"),
    ];
    println!("{}", longest_palindrome(words));
}
