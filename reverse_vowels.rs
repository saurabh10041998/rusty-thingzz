fn is_vowel(c: char) -> bool {
    for i in "aeiou".chars() {
        if i == c {
            return true;
        }
    }
    for i in "AEIOU".chars() {
        if i == c { 
            return true;
        }
    }
    false
}

fn reverse_vowel(s: String) -> String {
    let mut vowels = String::new(); 
    for i in s.chars() {
        if is_vowel(i) {
            vowels = vowels + &i.to_string();
        }
    }
    let vowels = vowels.chars().rev().collect::<String>();
    let mut j = 0;
    let mut ans = String::new();
    for i in s.chars() {
        if is_vowel(i) {
            ans = ans + &vowels[j..j + 1];
            j += 1;
        }else {
            ans = ans + &i.to_string();
        }
    }
    ans
}
fn main() {
    let t: char  = 'a';
    let tc: String = String::from("leetcode");
    println!("{}", is_vowel(t));
    println!("{}", reverse_vowel(tc));
}
